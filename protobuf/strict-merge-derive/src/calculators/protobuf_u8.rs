use crate::calculators::{ValueCalculator, Assign, add_ampersand, RepeatedComputer, wire_check};
use proc_macro2::{Ident, TokenStream};
use crate::parser::OneOfMapping;

impl ValueCalculator for u8 {
    fn read(&self, ident: &Ident, wire_type_ident: &Ident, is_ident: &Ident, type_without_opt: &TokenStream) -> (Assign, TokenStream) {
        (Assign::Set, quote! {
            ::protobuf::rt::read_singular_proto3_bytes(#wire_type_ident, #is_ident)?
        })
    }

    fn size(&self, ident: &TokenStream, size_ident: &Ident, field_number: u32, type_without_opt: &TokenStream, is_reference: bool) -> TokenStream {
        quote! {
            #size_ident += ::protobuf::rt::bytes_size(#field_number, &#ident);
        }
    }

    fn write(&self, ident: &TokenStream, os_ident: &Ident, field_number: u32, type_without_opt: &TokenStream, is_reference: bool) -> TokenStream {
        quote! {
            #os_ident.write_bytes(#field_number, &#ident)?;
        }
    }

    fn default_value(&self, ident: &TokenStream) -> Option<TokenStream> {
        Some(quote! {
            vec![]
        })
    }

    fn check_has_not_default_value(&self, ident: &TokenStream) -> Option<TokenStream> {
        Some(quote! {
            !#ident.is_empty()
        })
    }

    fn read_repeated(&self) -> (RepeatedComputer, TokenStream) {
        unreachable!()
    }
}