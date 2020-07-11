#![feature(fmt_internals)]
#![feature(libstd_sys_internals)]

use protobuf::{parse_from_bytes_strict, ProtobufEnumOrUnknown, Message};
use protobuf::StrictMerge;
use crate::api::{Compound};
use crate::other_messages::{AnEnum, Inner};
use crate::other_messages::compound::OneOfSomething;
use crate::org_message::compound::{One_of_double, One_of_enum, One_of_message};

mod api_impl;
mod other_messages;
mod api;
mod org_message;

fn main() {
    let new = Compound {
        double_default: 0.0,
        double_non_default: 1 as f64,
        enum_default: AnEnum::ACase,
        enum_o_default: Some(AnEnum::ACase),
        enum_o_empty: None,
        message_default: Inner { double_default: 1.0 },
        message_o_default: Some(Inner { double_default: 1.0 }),
        message_o_empty: None,
        one_of_double: OneOfSomething::double(1 as f64),
        one_of_enum: OneOfSomething::_enum(AnEnum::ACase),
        one_of_message: OneOfSomething::message(Inner { double_default: 1 as f64 }),
    };

    let old = org_message::Compound {
        double_default: 0 as f64,
        double_non_default: 1 as f64,
        enum_default: ProtobufEnumOrUnknown::new(org_message::AnEnum::A_CASE),
        enum_o_default: ProtobufEnumOrUnknown::new(org_message::AnEnum::A_CASE),
        enum_o_empty: ProtobufEnumOrUnknown::default(),
        message_default: Some(org_message::Inner {
            double_default: 1 as f64,
            unknown_fields: Default::default(),
            cached_size: Default::default()
        }),
        message_o_default: Some(org_message::Inner {
            double_default: 1 as f64,
            unknown_fields: Default::default(),
            cached_size: Default::default()
        }),
        message_o_empty: None,
        one_of_double: Some(One_of_double::a_double_0(1 as f64)),
        one_of_enum: Some(One_of_enum::an_enum_1(ProtobufEnumOrUnknown::new(org_message::AnEnum::A_CASE))),
        one_of_message: Some(One_of_message::a_message_2(org_message::Inner {
            double_default: 1 as f64,
            unknown_fields: Default::default(),
            cached_size: Default::default()
        })),
        unknown_fields: Default::default(),
        cached_size: Default::default()
    };

    assert_eq!(new.compute_size(), old.compute_size() as usize);

    let bytes = new.write_to_bytes().unwrap();

    let again: Compound = parse_from_bytes_strict(&bytes).unwrap();

    assert_eq!(new, again);

    // double: 1.,
    // double_default: 0.0,
    // _u32: 1,
    // _u32_default: 0,
    // sfixed64: 1,
    // sfixed64_default: 0,
    // string: "1".to_string(),
    // string_default: "".to_string(),
    // uuid: uuid::Uuid::new_v4(),
    // bytes: vec![1],
    // bytes_default: vec![]
    //

}