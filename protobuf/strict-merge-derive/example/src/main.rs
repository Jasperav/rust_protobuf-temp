#![feature(fmt_internals)]
#![feature(libstd_sys_internals)]

use protobuf::{parse_from_bytes_strict, ProtobufEnumOrUnknown, Message, parse_from_bytes};
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
    let first_inner = Inner {
        double_default: 1 as f64,
    };
    let second_inner = Inner {
        double_default: 2 as f64,
    };
    let new = Compound {
        double_default: 0.0,
        double_non_default: 1 as f64,
        enum_default: AnEnum::ACase,
        enum_o_default: Some(AnEnum::ACase),
        enum_o_empty: None,
        message_default: first_inner.clone(),
        message_o_default: Some(first_inner.clone()),
        message_o_empty: None,
        one_of_double: OneOfSomething::double(1 as f64),
        one_of_enum: OneOfSomething::_enum(AnEnum::ACase),
        one_of_message: OneOfSomething::message(first_inner.clone()),
        bytes_default: vec![1, 2],
        bytes_o_default: vec![1, 2],
        bytes_o_empty: vec![],
        vec_double_default: vec![1 as f64, 2 as f64],
        vec_double_o_default: vec![1 as f64, 2 as f64],
        vec_double_o_empty: vec![],
        vec_message_default: vec![first_inner.clone(), second_inner.clone()],
        vec_message_o_default: vec![first_inner.clone(), second_inner.clone()],
        vec_message_o_empty: vec![],
    };

    let first_inner = org_message::Inner {
        double_default: 1 as f64,
        unknown_fields: Default::default(),
        cached_size: Default::default()
    };
    let second_inner = org_message::Inner {
        double_default: 2 as f64,
        unknown_fields: Default::default(),
        cached_size: Default::default()
    };
    let old = org_message::Compound {
        double_default: 0 as f64,
        double_non_default: 1 as f64,
        enum_default: ProtobufEnumOrUnknown::new(org_message::AnEnum::A_CASE),
        enum_o_default: ProtobufEnumOrUnknown::new(org_message::AnEnum::A_CASE),
        enum_o_empty: ProtobufEnumOrUnknown::default(),
        message_default: Some(first_inner.clone()),
        message_o_default: Some(first_inner.clone()),
        message_o_empty: None,
        one_of_double: Some(One_of_double::a_double_0(1 as f64)),
        one_of_enum: Some(One_of_enum::an_enum_1(ProtobufEnumOrUnknown::new(org_message::AnEnum::A_CASE))),
        one_of_message: Some(One_of_message::a_message_2(first_inner.clone())),
        bytes_default: vec![1, 2],
        bytes_o_default: vec![1, 2],
        bytes_o_empty: vec![],
        vec_double_default: vec![1 as f64, 2 as f64],
        vec_double_o_default: vec![1 as f64, 2 as f64],
        vec_double_o_empty: vec![],
        vec_message_default: vec![first_inner.clone(), second_inner.clone()],
        vec_message_o_default: vec![first_inner.clone(), second_inner.clone()],
        vec_message_o_empty: vec![],
        // vec_enum_default: vec![ProtobufEnumOrUnknown::new(AnEnum::A_CASE), ProtobufEnumOrUnknown::new(AnEnum::ANOTHER_CASE)],
        // vec_enum_o_default: vec![ProtobufEnumOrUnknown::new(AnEnum::A_CASE), ProtobufEnumOrUnknown::new(AnEnum::ANOTHER_CASE)],
        // vec_enum_o_empty: vec![],
        unknown_fields: Default::default(),
        cached_size: Default::default()
    };

    assert_eq!(new.compute_size(), old.compute_size() as usize);

    let new_bytes = new.write_to_bytes().unwrap();
    let new_deserialized: Compound = parse_from_bytes_strict(&new_bytes).unwrap();
    assert_eq!(new, new_deserialized);

    let old_bytes = old.write_to_bytes().unwrap();
    let old_deserialized: org_message::Compound = parse_from_bytes(&old_bytes).unwrap();

    assert_eq!(old, old_deserialized);
    assert_eq!(new_bytes, old_bytes);

    let old_to_new_deserialized: Compound = parse_from_bytes_strict(&old_bytes).unwrap();
    let new_to_old_deserialized: org_message::Compound = parse_from_bytes(&new_bytes).unwrap();

    assert_eq!(old_to_new_deserialized, new);
    assert_eq!(new_to_old_deserialized, old);
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