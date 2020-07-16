use syn::{DeriveInput, Data, Fields, Field};
use quote::{quote, format_ident};
use proc_macro2::{TokenStream, Punct, Literal};
use syn::parse::{Parse, ParseBuffer};
use syn::parse_macro_input;
use crate::matcher::{Proto, find_attr};
use crate::parser::{Prototype, FieldNumber, OneOfMapping};


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
            let mut deserialize = vec![];
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
                let field_opt = format_ident!("{}", ident.to_string() + "_opt");

                crate::matcher::calculate_values(&prototype, Proto::Simple(&mut declarations, &mut check_opt_is_filled, &mut assign), field_number, ident, &mut deserialize, &mut compute_sizer, &mut os_writer);
            }

            let reader = quote! {
                while !is.eof()? {
                    let (field_number, wire_type) = is.read_tag_unpack()?;
                    match field_number {
                        #(#deserialize),*
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

