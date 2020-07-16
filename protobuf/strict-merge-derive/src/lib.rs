use syn::{DeriveInput, Data, Fields, Field};
use quote::{quote, format_ident};
use proc_macro2::{TokenStream, Punct, Literal};
use syn::parse::{Parse, ParseBuffer};
use syn::parse_macro_input;

fn parse_literal(input: &ParseBuffer) -> syn::Result<String> {
    Ok(Literal::parse(input)?.to_string().replace("\"", ""))
}

struct Prototype(String);

impl Parse for Prototype {
    fn parse(input: &ParseBuffer) -> syn::Result<Self> {
        // Ignore the '='
        Punct::parse(input)?;

        Ok(Prototype(parse_literal(input)?))
    }
}

struct FieldNumber(i32);

impl Parse for FieldNumber {
    fn parse(input: &ParseBuffer) -> syn::Result<Self> {
        // Ignore the '='
        Punct::parse(&input)?;

        let val = Literal::parse(input)?;

        Ok(FieldNumber(val.to_string().parse().unwrap()))
    }
}

#[derive(Debug)]
struct OneOfMapping {
    name: String,
    prototype: String,
    field_number: i32,
}

impl Parse for OneOfMapping {
    fn parse(input: &ParseBuffer) -> syn::Result<Self> {
        // Ignore the '='
        Punct::parse(&input)?;

        let info = parse_literal(input)?;
        let (name, info) = info.split_at(info.find("|").unwrap() + 1);
        let (prototype, field_number) = info.split_at(info.find("|").unwrap() + 1);

        Ok(OneOfMapping {
            name: name.replace("|", ""),
            prototype: prototype.replace("|", ""),
            field_number: field_number.parse().unwrap(),
        })
    }
}

fn find_attr(field: &Field, attr: &'static str) -> Vec<proc_macro::TokenStream> {
    field.attrs.iter().filter(|f| f.path.segments.iter().find(|f| attr == &f.ident.to_string()).is_some()).map(|a| a.tokens.clone().into()).collect()
}

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

                let (deserialize_field, default_value) = match prototype.as_str() {
                    "double" => {
                        compute_sizer.push(quote! {
                            if self.#ident != 0. {
                                size += 9;
                            }
                        });

                        os_writer.push(quote! {
                            if self.#ident != 0. {
                                os.write_double(#field_number, self.#ident)?;
                            }
                        });

                        (quote! {
                            if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                                return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                            }
                            #ident = is.read_double()?;
                        }, Some(quote! {
                            0 as f64
                        }))
                    }
                    "oneof" => {
                        let mut mapping = vec![];

                        // Cant get it to work with into_iter and mapping...
                        for att in find_attr(field, "oneof") {
                            mapping.push(parse_macro_input!(att as OneOfMapping));
                        }

                        let size_computer = mapping
                            .iter()
                            .map(|m| {

                            })

                        panic!("{:#?}", mapping);
                    }
                    "uint32" => {
                        compute_sizer.push(quote! {
                            if self.#ident != 0 {
                                size += ::protobuf::rt::value_size(#field_number, self.#ident, ::protobuf::wire_format::WireTypeVarint);
                            }
                        });

                        os_writer.push(quote! {
                            if self.#ident != 0 {
                                os.write_uint32(#field_number, self.#ident)?;
                            }
                        });

                        (quote! {
                            if wire_type != ::protobuf::wire_format::WireTypeVarint {
                                return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                            }
                            #ident = is.read_uint32()?;
                        }, Some(quote! {
                            0 as u32
                        }))
                    }
                    "sfixed64" => {
                        compute_sizer.push(quote! {
                            if self.#ident != 0 {
                                size += 9;
                            }
                        });

                        os_writer.push(quote! {
                            if self.#ident != 0 {
                                os.write_sfixed64(#field_number, self.#ident)?;
                            }
                        });

                        (quote! {
                            if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                                return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                            }
                            #ident = is.read_sfixed64()?;
                        }, Some(quote! {
                            0 as i64
                        }))
                    }
                    "uuid" => {
                        compute_sizer.push(quote! {
                            size += ::protobuf::rt::string_size(#field_number, &self.#ident.to_string());
                        });

                        os_writer.push(quote! {
                             os.write_string(#field_number, &self.#ident.to_string())?;
                        });

                        (quote! {
                            let mut string = String::new();

                            ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut string)?;

                            let uuid = match uuid::Uuid::from_str(&string) {
                                Ok(u) => u,
                                // TODO: change return type
                                Err(_) => {
                                    debug_assert!(false, "Invalid UUID found");

                                    return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                                }
                            };

                            #field_opt = Some(uuid);
                        }, None)
                    }
                    "string" => {
                        compute_sizer.push(quote! {
                            if !self.#ident.is_empty() {
                                size += ::protobuf::rt::string_size(#field_number, &self.#ident);
                            }
                        });

                        os_writer.push(quote! {
                            if !self.#ident.is_empty() {
                                os.write_string(#field_number, &self.#ident)?;
                            }
                        });

                        (quote! {
                            ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut #ident)?;
                        }, Some(quote! {
                            String::new()
                        }))
                    }
                    "bytes" => {
                        compute_sizer.push(quote! {
                            if !self.#ident.is_empty() {
                                size += ::protobuf::rt::bytes_size(#field_number, &self.#ident);
                            }
                        });

                        os_writer.push(quote! {
                            if !self.#ident.is_empty() {
                                os.write_bytes(#field_number, &self.#ident)?;
                            }
                        });

                        (quote! {
                            ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut #ident)?;
                        }, Some(quote! {
                            vec![]
                        }))
                    }

                    _ => unreachable!("Unexpected attributed found: {} for field {:#?}", prototype, field)
                };

                deserialize.push(quote! {
                    #field_number => {
                        debug_assert!(processed_field_indexes.insert(#field_number), "Double processed field index found for matching field {} (note that fields indexes start with 1, not 0)", #field_number);
                        #deserialize_field
                    }
                });

                if let Some(e) = default_value {
                    declarations.push(quote! {
                        #ident = #e
                    });
                    assign.push(quote! {
                        #ident
                    })
                } else {
                    declarations.push(quote! {
                        #field_opt = None
                    });
                    check_opt_is_filled.push(quote! {
                        if #field_opt.is_none() {
                            debug_assert!(false, "Unexpected empty optional found while deserializing property {}", stringify!(#field_opt));
                            return ::protobuf::ProtobufResult::Err(::protobuf::ProtobufError::WireError(::protobuf::error::WireError::IncorrectVarint));
                        }
                    });
                    assign.push(quote! {
                        #field_opt.unwrap()
                    });
                }
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