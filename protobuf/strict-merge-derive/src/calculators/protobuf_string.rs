use crate::calculators::{ValueCalculator, RepeatedComputer, Assign, add_ampersand};
use proc_macro2::{TokenStream, Ident};

impl ValueCalculator for &str {
    fn read(&self, ident: &Ident, wire_type_ident: &Ident, is_ident: &Ident, type_without_opt: &TokenStream) -> (Assign, TokenStream) {
        (Assign::Mutate, quote! {
            ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut #ident)?;
        })
    }

    fn size(&self, ident: &TokenStream, size_ident: &Ident, field_number: u32, type_without_opt: &TokenStream, is_reference: bool, tag_size: u32) -> TokenStream {
        quote! {
            #size_ident += ::protobuf::rt::string_size(#field_number, &#ident);
        }
    }

    fn write(&self, ident: &TokenStream, os_ident: &Ident, field_number: u32, type_without_opt: &TokenStream, is_reference: bool) -> TokenStream {
        let reference = add_ampersand(is_reference);

        quote! {
            #os_ident.write_string(#field_number, #reference#ident)?;
        }
    }

    fn default_value(&self, ident: &TokenStream) -> Option<TokenStream> {
        Some(quote! {
            String::new()
        })
    }

    fn check_has_not_default_value(&self, ident: &TokenStream) -> Option<TokenStream> {
        Some(quote! {
            !#ident.is_empty()
        })
    }

    fn read_repeated(&self) -> (RepeatedComputer, TokenStream) {
        unimplemented!()
    }
}