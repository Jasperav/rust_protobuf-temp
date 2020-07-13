use crate::calculators::{ValueCalculator, Assign, add_ampersand, RepeatedComputer, wire_check, add_deserialization};
use proc_macro2::{Ident, TokenStream};
use crate::parser::OneOfMapping;

pub struct OneOfMapper {
    pub mapping: Vec<OneOfMapping>,
}

impl OneOfMapper {
    fn loop_through_cases<T: Fn(&OneOfMapping, &TokenStream) -> TokenStream>(&self, ident: &TokenStream, type_without_opt: &TokenStream, is_reference: bool, gen_ts: T) -> TokenStream {
        let ampersand = add_ampersand(is_reference);
        let mut ts = vec![];
        let dummy_ident = quote! { dummy_ident };

        for mapping in self.mapping.iter() {
            let field_number = mapping.field_number;
            let enum_case = &mapping.enum_case;
            let gen = gen_ts(&mapping, &dummy_ident);
            let keyword = mapping.proto_mapping.keyword_match_statement().unwrap_or(TokenStream::new());

            ts.push(quote! {
               &#type_without_opt::#enum_case(#keyword #dummy_ident) => {
                   #gen
               }
            });
        }

        quote! {
            match #ampersand#ident {
                #(#ts),*
            }
        }
    }
}

impl ValueCalculator for OneOfMapper {
    fn read(&self, ident: &Ident, wire_type_ident: &Ident, is_ident: &Ident, type_without_opt: &TokenStream) -> (Assign, TokenStream) {
        let mut ts = TokenStream::new();

        for mapping in self.mapping.iter() {
            let wire_check = wire_check(mapping.proto_mapping.wire_check());
            let (assign, read) = mapping.proto_mapping.read(ident, wire_type_ident, is_ident, type_without_opt);
            let enum_case = &mapping.enum_case;

            let assign = match assign {
                Assign::Set => quote! {
                    #wire_check
                    #ident = Some(#type_without_opt::#enum_case(#read));
                },
                Assign::Mutate => quote! {
                    #wire_check
                    Some(#type_without_opt::#enum_case(#read));
                },
            };

            ts.extend(add_deserialization(mapping.field_number, assign));
        }

        (Assign::Set, ts)
    }

    fn size(&self, ident: &TokenStream, size_ident: &Ident, field_number: u32, type_without_opt: &TokenStream, is_reference: bool) -> TokenStream {
        self.loop_through_cases(ident, type_without_opt, is_reference, |mapping, dummy_ident| {
            let size = mapping.proto_mapping.size(dummy_ident, size_ident, mapping.field_number, type_without_opt, mapping.proto_mapping.keyword_match_statement().is_some());

            quote! {
                #size
            }
        })
    }

    fn write(&self, ident: &TokenStream, os_ident: &Ident, field_number: u32, type_without_opt: &TokenStream, is_reference: bool) -> TokenStream {
        self.loop_through_cases(ident, type_without_opt, is_reference, |mapping, dummy_ident| {
            let write = mapping.proto_mapping.write(dummy_ident, os_ident, mapping.field_number, type_without_opt, mapping.proto_mapping.keyword_match_statement().is_some());

            quote! {
                #write
            }
        })
    }

    fn read_repeated(&self) -> (RepeatedComputer, TokenStream) {
        unreachable!()
    }
}


pub fn add_deref(is_reference: bool) -> TokenStream {
    if is_reference {
        quote! { * }
    } else {
        quote! {}
    }
}

