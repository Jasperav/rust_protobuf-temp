use proc_macro2::{Ident, TokenStream};
use crate::matcher::Proto;
use quote::quote;
use quote::format_ident;
use crate::parser::{OneOfMapping, OneOfMapper};

pub struct Calculator<'a> {
    pub proto: Proto<'a>,
    pub field_number: u32,
    pub ident: &'a Ident,
    pub deserializer: &'a mut Vec<TokenStream>,
    pub compute_sizer: &'a mut Vec<TokenStream>,
    pub os_writer: &'a mut Vec<TokenStream>,
}

impl Calculator<'_> {
    pub fn calculate(self, t: Box<dyn ValueCalculator>) {
        t.calculate(self.proto, self.field_number, self.ident, self.deserializer, self.compute_sizer, self.os_writer);
    }
}

fn add_deserialization(field_number: u32, assign: TokenStream) -> TokenStream {
    quote! {
        #field_number => {
            debug_assert!(processed_field_indexes.insert(#field_number), "Double processed field index found for matching field {} (note that fields indexes start with 1, not 0)", #field_number);
            #assign
        }
    }
}

fn wire_check(wire_check: Option<TokenStream>) -> TokenStream {
    wire_check.map(|w| quote! {
            if wire_type != ::protobuf::wire_format::#w {
                return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
            }
        }).unwrap_or(TokenStream::new())
}

pub enum ValueOption {
    Optional,
    MandatoryOptional,
    Mandatory
}

pub trait ValueCalculator {
    fn read(&self, ident: &Ident) -> TokenStream;
    // a tokenstream rather than an ident because self. is not allowed in an ident
    fn size(&self, ident: &TokenStream, size_ident: &Ident, field_number: u32) -> TokenStream;
    fn write(&self, ident: &TokenStream, field_number: u32) -> TokenStream;
    fn wire_check(&self) -> Option<TokenStream>;
    fn default_value(&self, ident: &TokenStream) -> Option<TokenStream>;
    // TODO: Maybe this can be removed and be replaced by default_value
    fn check_has_not_default_value(&self, ident: &TokenStream) -> Option<TokenStream>;

    fn calculate(
        &self,
        proto: Proto,
        field_number: u32,
        ident: &Ident,
        deserializer: &mut Vec<TokenStream>,
        compute_sizer: &mut Vec<TokenStream>,
        os_writer: &mut Vec<TokenStream>) {
        // TODO: Not sure how to name this
        let another_ident = match &proto {
            Proto::OneOfCase(_) => quote! { #ident },
            Proto::Simple(..) => quote! { self.#ident },
        };

        let reader = self.read(ident);
        let mut assign = wire_check(self.wire_check());

        match proto {
            Proto::OneOfCase(enum_case) => {
                let size = self.size(&another_ident, &format_ident!("size"), field_number);
                let write = self.write(&another_ident, field_number);

                compute_sizer.push(size);
                os_writer.push(write);
                assign.extend(quote! {
                    #ident = #enum_case(#reader);
                });
            }
            Proto::Simple(declaration, opt_checks, struct_gen, one_of_mapper) => {
                let value_option;
                let has_optional_type = ident.to_string().starts_with("o_");
                let ts = match self.default_value(&another_ident) {
                    None => {
                        // TODO: More user friendly
                        if has_optional_type {
                            value_option = ValueOption::Optional
                        } else {
                            value_option = ValueOption::MandatoryOptional;
                            opt_checks.push(quote! {
                                if #ident.is_none() {
                                    debug_assert!(false, "Unexpected empty optional found while deserializing property {}", stringify!(#ident));
                                    // TODO: return type is strange
                                    return ::protobuf::ProtobufResult::Err(::protobuf::ProtobufError::WireError(::protobuf::error::WireError::IncorrectVarint));
                                }
                            });
                        }
                        quote! {
                            None
                        }
                    }
                    Some(v) => {
                        value_option = ValueOption::Mandatory;
                        v
                    }
                };

                declaration.push(quote! {
                    #ident = #ts
                });

                // TODO: To many match's on the same value
                match &value_option {
                    ValueOption::Optional | ValueOption::Mandatory =>struct_gen.push(quote! {
                        #ident
                    }),
                    ValueOption::MandatoryOptional => struct_gen.push(quote! {
                        #ident.unwrap()
                    }),
                }

                let ident_some = match &value_option {
                    ValueOption::Optional =>  quote! { e } ,
                    ValueOption::MandatoryOptional | ValueOption::Mandatory => quote! { # another_ident },
                };
                let size = self.size(&ident_some, &format_ident!("size"), field_number);
                let write = self.write(&ident_some, field_number);

                let (compute_size, writer) = match self.check_has_not_default_value(&another_ident) {
                    None => (quote! {
                        #size
                    },
                     quote! {
                        #write
                     }),
                    Some(ts) => (quote! {
                        if #ts {
                            #size;
                        }
                    }, quote! {
                        if #ts {
                            #write
                        }
                    }),
                };

                match &value_option {
                    ValueOption::Optional => {
                        compute_sizer.push(quote! {
                            if let Some(#ident_some) = #another_ident {
                                #compute_size
                            }
                        });
                        os_writer.push(quote! {
                            if let Some(#ident_some) = #another_ident {
                                #writer
                            }
                        });
                    },
                    ValueOption::MandatoryOptional | ValueOption::Mandatory =>  {
                        compute_sizer.push(quote! {
                            #compute_size
                        });
                        os_writer.push(quote! {
                            #writer
                        });
                    }
                }

                let prefix_assign = match one_of_mapper {
                    None => {
                        match &value_option {
                            ValueOption::Optional | ValueOption::MandatoryOptional => assign.extend(quote! {
                                #ident = Some(#reader);
                            }),
                            ValueOption::Mandatory => assign.extend(quote! {
                                #ident = #reader;
                            }),
                        }

                        deserializer.push(add_deserialization(field_number, assign));
                    }
                    Some(o) => {
                        deserializer.push(reader);
                    }
                };
            }
        };
    }
}

impl OneOfMapper {
    fn loop_through_cases<T: Fn(&OneOfMapping, &TokenStream) -> TokenStream>(&self, ident: &TokenStream, gen_ts: T) -> TokenStream {
        let mut ts = vec![];
        let dummy_ident = quote! { dummy_ident };
        let full_type = &self.full_type;

        for mapping in self.mapping.iter() {
            let field_number = mapping.field_number;
            let enum_case = &mapping.enum_case;
            let gen = gen_ts(&mapping, &dummy_ident);

            ts.push(quote! {
               #full_type::#enum_case(#dummy_ident) => {
                   #gen
               }
            });
        }

        quote! {
            match #ident {
                #(#ts),*
            }
        }
    }
}

pub struct ProtobufEnum;

impl ValueCalculator for ProtobufEnum {
    fn read(&self, ident: &Ident) -> TokenStream {
        quote! {
            is.read_enum_strict()?
        }
    }

    fn size(&self, ident: &TokenStream, size_ident: &Ident, field_number: u32) -> TokenStream {
        quote! {
            debug_assert_ne!(0, #ident.value());
            #size_ident += ::protobuf::rt::enum_size_strict(#field_number, #ident);
        }
    }

    fn write(&self, ident: &TokenStream, field_number: u32) -> TokenStream {
        quote! {
            os.write_enum(#field_number, ::protobuf::ProtobufEnumStrict::value(&#ident))?;
        }
    }

    fn wire_check(&self) -> Option<TokenStream> {
        Some(quote! {
            WireTypeVarint
        })
    }

    fn default_value(&self, ident: &TokenStream) -> Option<TokenStream> {
        None
    }

    fn check_has_not_default_value(&self, ident: &TokenStream) -> Option<TokenStream> {
        None
    }
}

impl ValueCalculator for OneOfMapper {
    fn read(&self, ident: &Ident) -> TokenStream {
        let full_type = &self.full_type;
        let mut ts = TokenStream::new();

        for mapping in self.mapping.iter() {
            let wire_check = wire_check(mapping.proto_mapping.wire_check());
            let read = mapping.proto_mapping.read(ident);
            let enum_case = &mapping.enum_case;

            let assign = quote! {
                #wire_check
                #ident = Some(#full_type::#enum_case(#read)),
            };

            ts.extend(add_deserialization(mapping.field_number, assign));
        }

        ts
    }

    fn size(&self, ident: &TokenStream, size_ident: &Ident, field_number: u32) -> TokenStream {
        self.loop_through_cases(ident, |mapping, dummy_ident| {
            let size = mapping.proto_mapping.size(dummy_ident, size_ident, mapping.field_number);

            quote! {
                #size
            }
        })
    }

    fn write(&self, ident: &TokenStream, field_number: u32) -> TokenStream {
        self.loop_through_cases(ident, |mapping, dummy_ident| {
            let write = mapping.proto_mapping.write(dummy_ident, mapping.field_number);

            quote! {
                #write
            }
        })
    }

    fn wire_check(&self) -> Option<TokenStream> {
        None
    }

    fn default_value(&self, ident: &TokenStream) -> Option<TokenStream> {
        None
    }

    fn check_has_not_default_value(&self, ident: &TokenStream) -> Option<TokenStream> {
        None
    }
}

impl ValueCalculator for f64 {
    fn read(&self, ident: &Ident) -> TokenStream {
        quote! {
            is.read_double()?
        }
    }

    fn size(&self, ident: &TokenStream, size_ident: &Ident, field_number: u32) -> TokenStream {
        quote! {
            #size_ident += 9;
        }
    }

    fn write(&self, ident: &TokenStream, field_number: u32) -> TokenStream {
        quote! {
            os.write_double(#field_number, #ident)?;
        }
    }

    fn wire_check(&self) -> Option<TokenStream> {
        Some(quote! {
            WireTypeFixed64
        })
    }

    fn default_value(&self, ident: &TokenStream) -> Option<TokenStream> {
        Some(quote! {
            0 as f64
        })
    }

    fn check_has_not_default_value(&self, ident: &TokenStream) -> Option<TokenStream> {
        Some(quote! {
            #ident != 0 as f64
        })
    }
}