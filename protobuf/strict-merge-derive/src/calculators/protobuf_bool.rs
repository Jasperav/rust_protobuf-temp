use crate::calculators::{ValueCalculator, Assign, add_ampersand, RepeatedComputer, wire_check};
use proc_macro2::{Ident, TokenStream};
use crate::calculators::one_of::add_deref;

impl ValueCalculator for bool {
    fn read(&self, ident: &Ident, wire_type_ident: &Ident, is_ident: &Ident, type_without_opt: &TokenStream) -> (Assign, TokenStream) {
        (Assign::Set, quote! {
            #is_ident.read_bool()?;
        })
    }

    fn size(&self, ident: &TokenStream, size_ident: &Ident, field_number: u32, type_without_opt: &TokenStream, is_reference: bool, tag_size: u32) -> TokenStream {
        // +1 comes from method 'field_type_size' in protobuf
        quote! {
            #size_ident += #tag_size + 1;
        }
    }

    fn write(&self, ident: &TokenStream, os_ident: &Ident, field_number: u32, type_without_opt: &TokenStream, is_reference: bool) -> TokenStream {
        let deref = add_deref(is_reference);

        quote! {
            #os_ident.write_bool(#field_number, #deref#ident)?;
        }
    }

    fn wire_check(&self) -> Option<TokenStream> {
        Some(quote! {
            WireTypeVarint
        })
    }

    fn default_value(&self, ident: &TokenStream) -> Option<TokenStream> {
        Some(quote! {
            false
        })
    }

    fn check_has_not_default_value(&self, ident: &TokenStream) -> Option<TokenStream> {
        Some(quote! {
            #ident != false
        })
    }

    fn read_repeated(&self) -> (RepeatedComputer, TokenStream) {
        unimplemented!()
    }
}