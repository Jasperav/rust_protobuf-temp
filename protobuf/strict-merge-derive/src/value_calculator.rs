use proc_macro2::{Ident, TokenStream};
use crate::matcher::{Proto, MapType};
use quote::quote;
use quote::format_ident;
use crate::parser::{OneOfMapping, OneOfMapper};

pub struct Calculator<'a> {
    pub map_type: MapType,
    pub ident: &'a Ident,
    pub is_option: bool,
    pub type_without_opt: TokenStream,
    pub declaration: &'a mut Vec<TokenStream>,
    pub opt_checks: &'a mut Vec<TokenStream>,
    pub struct_gen: &'a mut Vec<TokenStream>,
    pub deserializer: &'a mut Vec<TokenStream>,
    pub compute_sizer: &'a mut Vec<TokenStream>,
    pub os_writer: &'a mut Vec<TokenStream>,
}

impl Calculator<'_> {
    pub fn calculate(self, t: Box<dyn ValueCalculator>) {
        t.calculate(self.ident, self.map_type,  self.is_option, self.type_without_opt, self.declaration, self.opt_checks, self.struct_gen, self.deserializer, self.compute_sizer, self.os_writer);
    }
}

fn add_deserialization(field_number: u32, assign: TokenStream) -> TokenStream {
    quote! {
        #field_number => {
            debug_assert!(processed_field_indexes.insert(#field_number), "Double processed field index found for matching field {} (note that fields indexes start with 1, not 0)", #field_number);
            #assign
        },
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
    Mandatory,
}

pub trait ValueCalculator {
    // TODO: Maybe instead of all the arguments, pass in the calculator
    fn read(&self, ident: &Ident, type_without_opt: &TokenStream) -> TokenStream;
    // a tokenstream rather than an ident because self. is not allowed in an ident
    fn size(&self, ident: &TokenStream, size_ident: &Ident, field_number: u32, type_without_opt: &TokenStream) -> TokenStream;
    fn write(&self, ident: &TokenStream, field_number: u32, type_without_opt: &TokenStream) -> TokenStream;
    fn wire_check(&self) -> Option<TokenStream>;
    fn default_value(&self, ident: &TokenStream) -> Option<TokenStream>;
    // TODO: Maybe this can be removed and be replaced by default_value
    fn check_has_not_default_value(&self, ident: &TokenStream) -> Option<TokenStream>;

    fn calculate(
        &self,
        ident: &Ident,
        map_type: MapType,
        is_option: bool,
        type_without_opt: TokenStream,
        declaration: &mut Vec<TokenStream>,
        opt_checks: &mut Vec<TokenStream>,
        struct_gen: &mut Vec<TokenStream>,
        deserializer: &mut Vec<TokenStream>,
        compute_sizer: &mut Vec<TokenStream>,
        os_writer: &mut Vec<TokenStream>) {
        // TODO: Not sure how to name this
        let ident_with_self = quote! { self.#ident };
        let field_number = match map_type {
            // This is not used anyway in the value_calculator of the one_ofs
            MapType::OneOf => 0,
            MapType::Simple(s) => s,
        };

        let reader = self.read(ident, &type_without_opt);
        let mut assign = wire_check(self.wire_check());

        let value_option;
        let ts = match self.default_value(&ident_with_self) {
            None => {
                // TODO: More user friendly
                if is_option {
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
            ValueOption::Optional | ValueOption::Mandatory => struct_gen.push(quote! {
                        #ident
                    }),
            ValueOption::MandatoryOptional => struct_gen.push(quote! {
                        #ident.unwrap()
                    }),
        }

        let ident_some = match &value_option {
            ValueOption::Optional => quote! { e },
            ValueOption::MandatoryOptional | ValueOption::Mandatory => quote! { # ident_with_self },
        };
        let size = self.size(&ident_some, &format_ident!("size"), field_number, &type_without_opt);
        let write = self.write(&ident_some, field_number, &type_without_opt);

        let (compute_size, writer) = match self.check_has_not_default_value(&ident_with_self) {
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
                            if let Some(#ident_some) = #ident_with_self {
                                #compute_size
                            }
                        });
                os_writer.push(quote! {
                            if let Some(#ident_some) = #ident_with_self {
                                #writer
                            }
                        });
            }
            ValueOption::MandatoryOptional | ValueOption::Mandatory => {
                compute_sizer.push(quote! {
                            #compute_size
                        });
                os_writer.push(quote! {
                            #writer
                        });
            }
        }

        match map_type {
            MapType::OneOf => {
                deserializer.push(reader);
            }
            MapType::Simple(_) => {
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
        };
    }
}

impl OneOfMapper {
    fn loop_through_cases<T: Fn(&OneOfMapping, &TokenStream) -> TokenStream>(&self, ident: &TokenStream, type_without_opt: &TokenStream, gen_ts: T) -> TokenStream {
        let mut ts = vec![];
        let dummy_ident = quote! { dummy_ident };

        for mapping in self.mapping.iter() {
            let field_number = mapping.field_number;
            let enum_case = &mapping.enum_case;
            let gen = gen_ts(&mapping, &dummy_ident);

            ts.push(quote! {
               #type_without_opt::#enum_case(#dummy_ident) => {
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

impl ValueCalculator for OneOfMapper {
    fn read(&self, ident: &Ident, type_without_opt: &TokenStream) -> TokenStream {
        let mut ts = TokenStream::new();

        for mapping in self.mapping.iter() {
            let wire_check = wire_check(mapping.proto_mapping.wire_check());
            let read = mapping.proto_mapping.read(ident, type_without_opt);
            let enum_case = &mapping.enum_case;

            let assign = quote! {
                #wire_check
                #ident = Some(#type_without_opt::#enum_case(#read));
            };

            ts.extend(add_deserialization(mapping.field_number, assign));
        }

        ts
    }

    fn size(&self, ident: &TokenStream, size_ident: &Ident, field_number: u32, type_without_opt: &TokenStream) -> TokenStream {
        self.loop_through_cases(ident, type_without_opt, |mapping, dummy_ident| {
            let size = mapping.proto_mapping.size(dummy_ident, size_ident, mapping.field_number, type_without_opt);

            quote! {
                #size
            }
        })
    }

    fn write(&self, ident: &TokenStream, field_number: u32, type_without_opt: &TokenStream) -> TokenStream {
        self.loop_through_cases(ident, type_without_opt, |mapping, dummy_ident| {
            let write = mapping.proto_mapping.write(dummy_ident, mapping.field_number, type_without_opt);

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

pub struct ProtobufEnum;

impl ValueCalculator for ProtobufEnum {
    fn read(&self, ident: &Ident, _: &TokenStream) -> TokenStream {
        quote! {
            is.read_enum_strict()?
        }
    }

    fn size(&self, ident: &TokenStream, size_ident: &Ident, field_number: u32, type_without_opt: &TokenStream) -> TokenStream {
        quote! {
            debug_assert_ne!(0, #ident.value());
            #size_ident += ::protobuf::rt::enum_size_strict(#field_number, #ident);
        }
    }

    fn write(&self, ident: &TokenStream, field_number: u32, type_without_opt: &TokenStream) -> TokenStream {
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

impl ValueCalculator for f64 {
    fn read(&self, ident: &Ident, _: &TokenStream) -> TokenStream {
        quote! {
            is.read_double()?
        }
    }

    fn size(&self, ident: &TokenStream, size_ident: &Ident, field_number: u32, type_without_opt: &TokenStream) -> TokenStream {
        quote! {
            #size_ident += 9;
        }
    }

    fn write(&self, ident: &TokenStream, field_number: u32, type_without_opt: &TokenStream) -> TokenStream {
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