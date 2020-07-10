use syn::{DeriveInput, Data, Fields, Field};
use quote::{quote, format_ident};
use proc_macro2::{TokenStream, Punct, Literal, Ident};
use syn::parse::{Parse, ParseBuffer};
use syn::parse_macro_input;
use crate::parser::{Prototype, FieldNumber, OneOfMapping, OneOfMapper};
use crate::value_calculator::{Calculator, ValueCalculator, ProtobufEnum};
use std::str::FromStr;

pub fn find_attr(field: &Field, attr: &'static str) -> Vec<proc_macro::TokenStream> {
    field.attrs.iter().filter(|f| f.path.segments.iter().find(|f| attr == &f.ident.to_string()).is_some()).map(|a| a.tokens.clone().into()).collect()
}

pub enum MapType {
    OneOf,
    // Fieldnumber
    Simple(u32)
}

pub enum Proto<'a> {
    // Holds the declaration of values
    // so if this struct is derialized:
    // struct my_struct { my_value: i32 }
    // first all struct fields get a default/option value
    // like: my_value_temp = 0;
    // The declaration of variables is inside this first value case
    //
    // The second variable is optional checking
    // The first variable can hold options, the second variable should check after
    // deserialization if all mandatory fields are filled
    //
    // The third variable is the code used to assign it to a struct
    // e.g.
    // Ok(MyStruct { ... })
    // The dots represent this variable
    //
    // Fourth variable is true when dealing with a oneof variable
    Simple(&'a mut Vec<TokenStream>, &'a mut Vec<TokenStream>, &'a mut Vec<TokenStream>, Option<OneOfMapper>),
}

pub fn str_to_value_calculator(str: &str) -> Box<dyn ValueCalculator> {
    match str {
        "oneof" => unimplemented!("Oneof is not supported"),
        "double" => Box::new(0 as f64),
        "enum" => Box::new(ProtobufEnum),
        _ => unimplemented!("Nothing configured for type: {}", str)
    }
}

//         "uint32" => {
//
//
//
//
//
//             compute_sizer.push(quote! {
// if self.#ident != 0 {
// size += ::protobuf::rt::value_size(#field_number, self.#ident, ::protobuf::wire_format::WireTypeVarint);
// }
// });
//
//             os_writer.push(quote! {
// if self.#ident != 0 {
// os.write_uint32(#field_number, self.#ident)?;
// }
// });
//
//             (quote! {
// if wire_type != ::protobuf::wire_format::WireTypeVarint {
// return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
// }
// #ident = is.read_uint32()?;
// }, Some(quote! {
// 0 as u32
// }))
//         }
//         "sfixed64" => {
//             compute_sizer.push(quote! {
// if self.#ident != 0 {
// size += 9;
// }
// });
//
//             os_writer.push(quote! {
// if self.#ident != 0 {
// os.write_sfixed64(#field_number, self.#ident)?;
// }
// });
//
//             (quote! {
// if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
// return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
// }
// #ident = is.read_sfixed64()?;
// }, Some(quote! {
// 0 as i64
// }))
//         }
//         "uuid" => {
//             compute_sizer.push(quote! {
// size += ::protobuf::rt::string_size(#field_number, &self.#ident.to_string());
// });
//
//             os_writer.push(quote! {
// os.write_string(#field_number, &self.#ident.to_string())?;
// });
//
//             (quote! {
// let mut string = String::new();
//
// ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut string)?;
//
// let uuid = match uuid::Uuid::from_str(&string) {
// Ok(u) => u,
// // TODO: change return type
// Err(_) => {
// debug_assert!(false, "Invalid UUID found");
//
// return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
// }
// };
//
// #field_opt = Some(uuid);
// }, None)
//         }
//         "string" => {
//             compute_sizer.push(quote! {
// if !self.#ident.is_empty() {
// size += ::protobuf::rt::string_size(#field_number, &self.#ident);
// }
// });
//
//             os_writer.push(quote! {
// if !self.#ident.is_empty() {
// os.write_string(#field_number, &self.#ident)?;
// }
// });
//
//             (quote! {
// ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut #ident)?;
// }, Some(quote! {
// String::new()
// }))
//         }
//         "bytes" => {
//             compute_sizer.push(quote! {
// if !self.#ident.is_empty() {
// size += ::protobuf::rt::bytes_size(#field_number, &self.#ident);
// }
// });
//
//             os_writer.push(quote! {
// if !self.#ident.is_empty() {
// os.write_bytes(#field_number, &self.#ident)?;
// }
// });
//
//             (quote! {
// ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut #ident)?;
// }, Some(quote! {
// vec![]
// }))
//         },
//         "oneof" => {
//             let mut mapping = vec![];
//
//             // Cant get it to work with into_iter and mapping...
//             for att in find_attr(field, "oneof") {
//                 mapping.push(parse_macro_input!(att as OneOfMapping));
//             }
//
//             // let size_computer = mapping
//             //     .iter()
//             //     .map(|m| {}
//
//             panic!("{:#?}", mapping);
//         }
//         _ => unreachable!("Unexpected attributed found: {} for field {:#?}", prototype, field)