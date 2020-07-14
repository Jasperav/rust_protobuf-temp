use crate::calculators::{ValueCalculator, Assign, add_ampersand, RepeatedComputer, wire_check};
use proc_macro2::{Ident, TokenStream};
use crate::parser::OneOfMapping;
use crate::calculators::one_of::add_deref;

impl ValueCalculator for f64 {
    fn read(&self, ident: &Ident, wire_type_ident: &Ident, is_ident: &Ident, _: &TokenStream) -> (Assign, TokenStream) {
        (Assign::Set, quote! {
            #is_ident.read_double()?
        })
    }

    fn size(&self, ident: &TokenStream, size_ident: &Ident, field_number: u32, type_without_opt: &TokenStream, is_reference: bool, tag_size: u32) -> TokenStream {
        quote! {
            #size_ident += 9;
        }
    }

    fn write(&self, ident: &TokenStream, os_ident: &Ident, field_number: u32, type_without_opt: &TokenStream, is_reference: bool) -> TokenStream {
        let deref = add_deref(is_reference);
        quote! {
            #os_ident.write_double(#field_number, #deref#ident)?;
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

    fn read_repeated(&self) -> (RepeatedComputer, TokenStream) {
        (RepeatedComputer::UseLen, quote! {
            read_repeated_double_into
        })
    }
}