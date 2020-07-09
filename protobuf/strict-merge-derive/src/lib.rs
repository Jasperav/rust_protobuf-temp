use syn::{DeriveInput, Data, Fields, Field};
use quote::{quote, format_ident};
use proc_macro2::{TokenStream, Punct, Literal};
use syn::parse::{Parse, ParseBuffer};
use syn::parse_macro_input;
use crate::matcher::{Proto, find_attr, str_to_value_calculator};
use crate::parser::{Prototype, FieldNumber, OneOfMapping, OneOfMapper};
use crate::value_calculator::{Calculator, ValueCalculator};


mod matcher;
mod parser;
mod value_calculator;

#[proc_macro_derive(StrictMerge, attributes(prototype, fieldnumber, oneof))]
pub fn strict_merge(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derive_input: DeriveInput = syn::parse(input).unwrap();
    let name = derive_input.ident;
    let mut methods = vec![];

    match derive_input.data {
        Data::Struct(s) => {
            let named_fields: Vec<Field> = if let Fields::Named(n) = s.fields {
                n.named.into_iter().collect()
            } else {
                unreachable!("All fields should be named");
            };

            let mut declarations = vec![quote! {
                processed_field_indexes = std::collections::HashSet::new()
            }];
            let mut deserializer = vec![];
            let mut check_opt_is_filled = vec![];
            let mut assign = vec![];
            let mut compute_sizer = vec![];
            let mut os_writer = vec![];

            let names = named_fields
                .iter()
                .map(|f| f.ident.clone().unwrap())
                .collect::<Vec<_>>();
            let names_opt = named_fields
                .iter()
                .map(|f| f.ident.clone().unwrap())
                .map(|f| f.to_string() + "_opt")
                .map(|f| format_ident!("{}", f))
                .collect::<Vec<_>>();
            for field in named_fields.iter() {
                let p = || panic!("{:#?}", field);

                let prototype = find_attr(field, "prototype").remove(0);
                // Remove weird \"
                let prototype = parse_macro_input!(prototype as Prototype).0;
                let field_number = find_attr(field, "fieldnumber").remove(0);
                let field_number = parse_macro_input!(field_number as FieldNumber).0;
                let ident = field.ident.clone().unwrap();

                let (proto, value_calculator): (_, Box<dyn ValueCalculator>) = if prototype == "oneof" {
                    let mut oneofs = vec![];
                    // TODO Not sure how to make ValueCalculator Clone because that gives weird errors
                    let mut oneofs_2 = vec![];

                    for oneof in find_attr(field, "oneof") {
                        // For some reason, this loop does not work in calculate_values
                        let clone = oneof.clone();
                        oneofs.push(parse_macro_input!(clone as OneOfMapping));
                        oneofs_2.push(parse_macro_input!(oneof as OneOfMapping));
                    }

                    let one_of_mapping = OneOfMapper {
                        mapping: oneofs,
                        full_type: field.ty.clone()
                    };
                    let one_of_mapping_2 = OneOfMapper {
                        mapping: oneofs_2,
                        full_type: field.ty.clone()
                    };

                    (Proto::Simple(&mut declarations, &mut check_opt_is_filled, &mut assign, Some(one_of_mapping)), Box::new(one_of_mapping_2))
                } else {
                    (Proto::Simple(&mut declarations, &mut check_opt_is_filled, &mut assign, None), str_to_value_calculator(&prototype))
                };

                let calculator = Calculator {
                    proto,
                    field_number,
                    ident: &ident,
                    deserializer: &mut deserializer,
                    compute_sizer: &mut compute_sizer,
                    os_writer: &mut os_writer,
                };

                calculator.calculate(value_calculator)
            }

            let reader = quote! {
                while !is.eof()? {
                    let (field_number, wire_type) = is.read_tag_unpack()?;
                    match field_number {
                        #(#deserializer),*
                        _ => {
                            debug_assert!(false, "number: {:#?}, wire_type: {:#?}", field_number, wire_type);
                            return ::protobuf::ProtobufResult::Err(::protobuf::ProtobufError::WireError(::protobuf::error::WireError::IncorrectVarint));
                        }
                    }
                }
            };

            let gen_struct = quote! {
                let gen_struct = #name {
                    #(#names: #assign),*
                };

                ::std::result::Result::Ok(gen_struct)
            };

            let strict_merge = quote! {
                #(let mut #declarations;)*
                #reader
                #(#check_opt_is_filled)*
                #gen_struct
            };

            methods.push(quote! {
                fn strict_merge(is: &mut protobuf::CodedInputStream<'_>) -> protobuf::ProtobufResult<Self> {
                    use std::str::FromStr;
                    #strict_merge
                }
            });

            methods.push(quote! {
                fn write_to_os(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
                    #(#os_writer)*
                    ::std::result::Result::Ok(())
                }
            });

            methods.push(quote! {
                fn compute_size(&self) -> usize {
                    let mut size = 0;

                    #(#compute_sizer)*

                    size as usize
                }
            })
        }
        Data::Enum(e) => {
            panic!();
        }
        Data::Union(_) => unreachable!("Unions in protobuf are not possible"),
    };

    let tokens = quote! {
        impl protobuf::StrictMerge<#name> for #name {
            #(#methods)*
        }
    };

    tokens.into()
}

