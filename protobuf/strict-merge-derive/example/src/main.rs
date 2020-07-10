#![feature(fmt_internals)]
#![feature(libstd_sys_internals)]

use protobuf::{parse_from_bytes_strict};
use protobuf::StrictMerge;
use crate::api::{Location, ConversationCreateRespError, Announcement};
use crate::api::some_enum::SomeResult;

mod api;

fn main() {
    let lo = Location {
        double_default: 0.0,
        double_non_default: 1 as f64,
        enum_default: ConversationCreateRespError::TOO_CLOSE_TO_OTHER_CONVERSATION,
        enum_o_default: Some(ConversationCreateRespError::TOO_CLOSE_TO_OTHER_CONVERSATION),
        enum_o_empty: None,
        one_of_double: SomeResult::double(1 as f64),
        one_of_enum: SomeResult::_enum(ConversationCreateRespError::TOO_CLOSE_TO_OTHER_CONVERSATION),
        one_of_message: SomeResult::message(Announcement { ts_created: 1 as f64 }),
        message_default: Announcement { ts_created: 0.0 },
        message_o_default: Some(Announcement { ts_created: 1.0 }),
        message_o_empty: None
    };

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

    let bytes = lo.write_to_bytes().unwrap();

    let again: Location = parse_from_bytes_strict(&bytes).unwrap();

    assert_eq!(lo, again);
}