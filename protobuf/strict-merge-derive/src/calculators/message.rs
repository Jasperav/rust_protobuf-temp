use crate::calculators::{ValueCalculator, Assign, add_ampersand, RepeatedComputer};
use proc_macro2::{Ident, TokenStream};

pub struct ProtobufMessage {
    pub(crate) tag_size: u32
}

impl ValueCalculator for ProtobufMessage {
    fn read(&self, ident: &Ident, wire_type_ident: &Ident, is_ident: &Ident, type_without_opt: &TokenStream) -> (Assign, TokenStream) {
        (Assign::Set, quote! {
            ::protobuf::rt::read_message::<_>(#wire_type_ident, #is_ident)?
        })
    }

    fn size(&self, ident: &TokenStream, size_ident: &Ident, field_number: u32, type_without_opt: &TokenStream, is_reference: bool) -> TokenStream {
        let tag_size = self.tag_size;

        quote! {
            let len = #ident.compute_size() as u32;
            #size_ident += #tag_size + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
    }

    fn write(&self, ident: &TokenStream, os_ident: &Ident, field_number: u32, type_without_opt: &TokenStream, is_reference: bool) -> TokenStream {
        let ts = add_ampersand(is_reference);
        quote! {
            ::protobuf::rt::write_strict_message_field_with_cached_size(#field_number, #ts#ident, #os_ident)?;
        }
    }

    fn keyword_match_statement(&self) -> Option<TokenStream> {
        Some(quote! {
            ref
        })
    }

    fn read_repeated(&self) -> (RepeatedComputer, TokenStream) {
        (RepeatedComputer::Reuse, quote! {
            read_repeated_message_strict_into_vec
        })
    }
}