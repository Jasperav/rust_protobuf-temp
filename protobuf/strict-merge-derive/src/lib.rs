use syn::{DeriveInput, Data, Fields, Field};
use quote::{quote, format_ident};
use proc_macro2::TokenStream;

#[proc_macro_derive(StrictMerge, attributes(t_double, t_float, t_int32, t_int64, t_uint32, t_uint64, t_sint32, t_sint64, t_fixed32, t_fixed64, t_sfixed32, t_sfixed64, t_bool, t_string, t_bytes, t_enum, t_message, t_group, t_uuid))]
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
            for (index, field) in named_fields.iter().enumerate() {
                let index = index as u32 + 1;
                let p = || panic!("{:#?}", field);

                let attr = field.attrs.last().unwrap().path.segments.first().unwrap().ident.to_string().replace("t_", "");
                let ident = field.ident.clone().unwrap();
                let field_opt = format_ident!("{}", ident.to_string() + "_opt");

                let (deserialize_field, default_value) = match attr.as_str() {
                    "double" => {
                        compute_sizer.push(quote! {
                            if self.#ident != 0. {
                                size += 9;
                            }
                        });

                        os_writer.push(quote! {
                            if self.#ident != 0. {
                                os.write_double(#index, self.#ident)?;
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
                    },
                    "uint32" => {
                        compute_sizer.push(quote! {
                            if self.#ident != 0 {
                                size += ::protobuf::rt::value_size(#index, self.#ident, ::protobuf::wire_format::WireTypeVarint);
                            }
                        });

                        os_writer.push(quote! {
                            if self.#ident != 0 {
                                os.write_uint32(#index, self.#ident)?;
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
                    },
                    "sfixed64" => {
                        compute_sizer.push(quote! {
                            if self.#ident != 0 {
                                size += 9;
                            }
                        });

                        os_writer.push(quote! {
                            if self.#ident != 0 {
                                os.write_sfixed64(#index, self.#ident)?;
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
                    },
                    "uuid" => {
                        compute_sizer.push(quote! {
                            size += ::protobuf::rt::string_size(#index, &self.#ident.to_string());
                        });

                        os_writer.push(quote! {
                             os.write_string(#index, &self.#ident.to_string())?;
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
                    },
                    "string" => {
                        compute_sizer.push(quote! {
                            if !self.#ident.is_empty() {
                                size += ::protobuf::rt::string_size(#index, &self.#ident);
                            }
                        });

                        os_writer.push(quote! {
                            if !self.#ident.is_empty() {
                                os.write_string(#index, &self.#ident)?;
                            }
                        });

                        (quote! {
                            ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut #ident)?;
                        }, Some(quote! {
                            String::new()
                        }))
                    },
                    "bytes" => {
                        compute_sizer.push(quote! {
                            if !self.#ident.is_empty() {
                                size += ::protobuf::rt::bytes_size(#index, &self.#ident);
                            }
                        });

                        os_writer.push(quote! {
                            if !self.#ident.is_empty() {
                                os.write_bytes(#index, &self.#ident)?;
                            }
                        });

                        (quote! {
                            ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut #ident)?;
                        }, Some(quote! {
                            vec![]
                        }))
                    }

                    _ => unreachable!("Unexpected attributed found: {}", attr)
                };

                deserialize.push(quote! {
                    #index => {
                        debug_assert!(processed_field_indexes.insert(#index), "Double processed field index found for matching field {} (note that fields indexes start with 1, not 0)", #index);
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