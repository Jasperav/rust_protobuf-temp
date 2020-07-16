pub mod protobuf_f64;
pub mod protobuf_enum;
pub mod protobuf_u8;
pub mod message;
pub mod repeated;
pub mod protobuf_bool;
pub mod protobuf_string;
pub mod one_of;
pub mod value_calculator;
pub use value_calculator::*;

use proc_macro2::TokenStream;

fn add_ampersand(is_reference: bool) -> TokenStream {
    if is_reference {
        quote! {}
    } else {
        quote! { & }
    }
}

fn add_deserialization(field_number: u32, assign: TokenStream) -> TokenStream {
    quote! {
        #field_number => {
            #assign
        },
    }
}