use crate::calculators::{ValueCalculator, Assign, add_ampersand, RepeatedComputer, wire_check};
use proc_macro2::{Ident, TokenStream};
use crate::parser::OneOfMapping;
use crate::calculators::one_of::add_deref;

pub struct ProtobufEnum;

impl ValueCalculator for ProtobufEnum {
    fn read(&self, ident: &Ident, wire_type_ident: &Ident, is_ident: &Ident, _: &TokenStream) -> (Assign, TokenStream) {
        (Assign::Set, quote! {
            #is_ident.read_enum_strict()?
        })
    }

    fn size(&self, ident: &TokenStream, size_ident: &Ident, field_number: u32, type_without_opt: &TokenStream, is_reference: bool) -> TokenStream {
        let deref = add_deref(is_reference);
        quote! {
            debug_assert_ne!(0, #ident.value());
            #size_ident += ::protobuf::rt::enum_size_strict(#field_number, #deref#ident);
        }
    }

    fn write(&self, ident: &TokenStream, os_ident: &Ident, field_number: u32, type_without_opt: &TokenStream, is_reference: bool) -> TokenStream {
        let deref = add_deref(is_reference);
        quote! {
            #os_ident.write_enum(#field_number, ::protobuf::ProtobufEnumStrict::value(#deref#ident))?;
        }
    }

    fn wire_check(&self) -> Option<TokenStream> {
        Some(quote! {
            WireTypeVarint
        })
    }

    fn read_repeated(&self) -> (RepeatedComputer, TokenStream) {
        (RepeatedComputer::Custom(Box::new(|rcs| {
            let field_number = rcs.field_number;
            let size_ident = rcs.size_ident;
            let loop_variable = rcs.loop_variable;

            quote! {
                #size_ident += ::protobuf::rt::enum_size_strict(#field_number, *#loop_variable);
            }
        })), quote! {
            read_repeated_enum_strict_into_vec
        })
    }
}