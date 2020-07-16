use protobuf::{parse_from_bytes_strict};
use protobuf::StrictMerge;
use crate::api::{Location, ConversationCreateRespError};

mod api;

fn main() {
    let lo = Location {
        double: 1.0,
        o_double_default: 0.0,
        _enum: ConversationCreateRespError::CONVERSATION_CREATE_RESP_ERROR_NOT_SET
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