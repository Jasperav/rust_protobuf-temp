use proc_macro2::{Ident, TokenStream};
use crate::matcher::Proto;
use quote::quote;
use quote::format_ident;

pub struct Calculator<'a> {
    pub proto: Proto<'a>,
    pub field_number: u32,
    pub ident: &'a Ident,
    pub deserializer: &'a mut Vec<TokenStream>,
    pub compute_sizer: &'a mut Vec<TokenStream>,
    pub os_writer: &'a mut Vec<TokenStream>
}

impl Calculator<'_> {
    pub fn calculate<T: ValueCalculator>(self) {
        T::calculate(self.proto, self.field_number, self.ident, self.deserializer, self.compute_sizer, self.os_writer);
    }
}

pub trait ValueCalculator {
    fn size(ident: &TokenStream) -> TokenStream;
    fn write(field_number: u32, ident: &TokenStream) -> TokenStream;
    fn read() -> TokenStream;
    fn wire_check() -> TokenStream;
    fn default_value(ident: &TokenStream) -> Option<TokenStream>;
    // TODO: Maybe this can be removed and be replaced by default_value
    fn check_has_not_default_value(ident: &TokenStream) -> Option<TokenStream>;

    fn calculate(proto: Proto,
                 field_number: u32,
                 ident: &Ident,
                 deserializer: &mut Vec<TokenStream>,
                 compute_sizer: &mut Vec<TokenStream>,
                 os_writer: &mut Vec<TokenStream>) {
        // TODO: Not sure how to name this
        let another_ident = match &proto {
            Proto::OneOf(_) => quote! { #ident },
            Proto::Simple(_, _, _) => quote! { self.#ident },
        };

        let size = Self::size(&another_ident);
        let write = Self::write(field_number, &another_ident);
        let reader = Self::read();
        let mut assign = Self::wire_check();

        match proto {
            Proto::OneOf(enum_case) => {
                compute_sizer.push(size);
                os_writer.push(write);
                assign.extend(quote! {
                    #ident = #enum_case(#reader);
                });
            }
            Proto::Simple(declaration, opt_checks, struct_gen) => {
                let is_option;
                let is_mandatory;
                let ts = match Self::default_value(&another_ident) {
                    None => {
                        // TODO: More user friendly
                        if !ident.to_string().starts_with("o_") {
                            is_option = true;
                            is_mandatory = false;
                            opt_checks.push(quote! {
                                if #ident.is_none() {
                                    debug_assert!(false, "Unexpected empty optional found while deserializing property {}", stringify!(#ident));
                                    // TODO: return type is strange
                                    return ::protobuf::ProtobufResult::Err(::protobuf::ProtobufError::WireError(::protobuf::error::WireError::IncorrectVarint));
                                }
                            });
                        } else {
                            is_option = false;
                            is_mandatory = true;
                        }
                        quote! {
                            None
                        }
                    }
                    Some(v) => {
                        is_option = false;
                        is_mandatory = true;
                        v
                    }
                };

                declaration.push(quote! {
                    #ident = #ts
                });

                let ident = if is_option {
                    assign.extend(quote! {
                        #ident = Some(#reader);
                    });
                    format_ident!("e")
                } else {
                    assign.extend(quote! {
                        #ident = #reader;
                    });
                    ident.clone()
                };

                if is_mandatory && is_option {
                    struct_gen.push(quote! {
                        #ident.unwrap()
                    });
                } else {
                    struct_gen.push(quote! {
                        #ident
                    });
                }

                let ts = Self::check_has_not_default_value(&another_ident);

                let (compute_size, writer) = match ts {
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

                if is_option {
                    compute_sizer.push(quote! {
                        if let Some(e) = #ident {
                            #compute_size
                        }
                    });
                    os_writer.push(quote! {
                        if let Some(e) = #ident {
                            #writer
                        }
                    });
                } else {
                    compute_sizer.push(quote! {
                        #compute_size
                    });
                    os_writer.push(quote! {
                        #writer
                    });
                }
            }
        };

        deserializer.push(quote! {
            #field_number => {
                debug_assert!(processed_field_indexes.insert(#field_number), "Double processed field index found for matching field {} (note that fields indexes start with 1, not 0)", #field_number);
                #assign
            }
        });
    }
}

impl ValueCalculator for f64 {
    fn size(ident: &TokenStream) -> TokenStream {
        quote! {
            size += 9;
        }
    }

    fn write(field_number: u32, ident: &TokenStream) -> TokenStream {
        quote! {
            os.write_double(#field_number, #ident)?;
        }
    }

    fn read() -> TokenStream {
        quote! {
            is.read_double()?
        }
    }

    fn wire_check() -> TokenStream {
        quote! {
            if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
            }
        }
    }

    fn default_value(ident: &TokenStream) -> Option<TokenStream> {
        Some(quote! {
            0 as f64
        })
    }

    fn check_has_not_default_value(ident: &TokenStream) -> Option<TokenStream> {
        Some(quote! {
            #ident != 0 as f64
        })
    }
}