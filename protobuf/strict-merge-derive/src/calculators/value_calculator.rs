use proc_macro2::{Ident, TokenStream};
use crate::matcher::{Proto, MapType};
use quote::quote;
use quote::format_ident;
use crate::calculators::add_deserialization;

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
        t.calculate(self.ident, self.map_type, self.is_option, self.type_without_opt, self.declaration, self.opt_checks, self.struct_gen, self.deserializer, self.compute_sizer, self.os_writer);
    }
}

pub(crate) fn wire_check(wire_check: Option<TokenStream>) -> TokenStream {
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

#[derive(PartialEq, Debug)]
pub enum Assign {
    Set,
    Mutate,
}

pub struct RepeatedCustomComputer {
    pub size_ident: Ident,
    pub field_number: u32,
    pub loop_variable: TokenStream
}

pub enum RepeatedComputer {
    Reuse,
    UseLen,
    Custom(Box<dyn Fn(RepeatedCustomComputer) -> TokenStream>)
}

pub trait ValueCalculator {
    // TODO: Maybe instead of all the arguments, pass in the calculator
    fn read(&self, ident: &Ident, wire_type_ident: &Ident, is_ident: &Ident, type_without_opt: &TokenStream) -> (Assign, TokenStream);
    // a tokenstream rather than an ident because self. is not allowed in an ident
    fn size(&self, ident: &TokenStream, size_ident: &Ident, field_number: u32, type_without_opt: &TokenStream, is_reference: bool) -> TokenStream;
    fn write(&self, ident: &TokenStream, os_ident: &Ident, field_number: u32, type_without_opt: &TokenStream, is_reference: bool) -> TokenStream;
    fn keyword_match_statement(&self) -> Option<TokenStream> {
        None
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
    fn read_repeated(&self) -> (RepeatedComputer, TokenStream);

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

        let is_ident = format_ident!("is");
        let wire_type_ident = format_ident!("wire_type");
        let reader = self.read(ident, &wire_type_ident, &is_ident, &type_without_opt);
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
        let size = self.size(&ident_some, &format_ident!("size"), field_number, &type_without_opt, is_option);
        let os_ident = format_ident!("os");
        let write = self.write(&ident_some, &os_ident, field_number, &type_without_opt, is_option);

        let (compute_size, writer) = match self.check_has_not_default_value(&ident_with_self) {
            None => (size, write),
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
                            if let Some(#ident_some) = &#ident_with_self {
                                #compute_size
                            }
                        });
            os_writer.push(quote! {
                            if let Some(#ident_some) = &#ident_with_self {
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

        match map_type {
            MapType::OneOf => {
                assert_eq!(Assign::Set, reader.0);

                deserializer.push(reader.1);
            }
            MapType::Simple(_) => {
                let (read_assign, read) = reader;

                match &value_option {
                    ValueOption::Optional | ValueOption::MandatoryOptional =>
                        match read_assign {
                            Assign::Set => assign.extend(quote! {
                                #ident = Some(#read);
                            }),
                            Assign::Mutate => assign.extend(quote! {
                                Some(#read);
                            }),
                        }
                    ValueOption::Mandatory => {
                        match read_assign {
                            Assign::Set => assign.extend(quote! {
                                #ident = #read;
                            }),
                            Assign::Mutate => assign.extend(quote! {
                                #read;
                            }),
                        }
                    }
                }

                deserializer.push(add_deserialization(field_number, assign));
            }
        };
    }
}








