use protobuf::{ProtobufResult, ProtobufError, ProtobufEnumStrict};
use protobuf::error::ProtobufError::WireError;

#[derive(protobuf::StrictMerge, Debug, PartialEq)]
pub struct Location {
    #[prototype = "double"]
    #[fieldnumber = 1]
    pub double: f64,
    #[prototype = "double"]
    #[fieldnumber = 2]
    pub o_double_default: f64,
    #[prototype = "enum"]
    #[fieldnumber = 3]
    pub _enum: crate::api::ConversationCreateRespError,
    #[prototype = "enum"]
    #[fieldnumber = 4]
    pub o_enum: Option<crate::api::ConversationCreateRespError>,
    #[prototype = "oneof"]
    #[oneof = "success|double|5"]
    #[oneof = "error|enum|6"]
    pub one_of: some_enum::SomeResult,
    #[prototype = "oneof"]
    #[oneof = "success|double|7"]
    #[oneof = "error|enum|8"]
    pub o_one_of: Option<some_enum::SomeResult>,
    #[prototype = "message"]
    #[fieldnumber = 9]
    pub message: Announcement,

    // #[prototype = "uint32"]
    // #[fieldnumber = 3]
    // pub _u32: u32,
    // #[prototype = "uint32"]
    // #[fieldnumber = 4]
    // pub _u32_default: u32,
    // #[prototype = "sfixed64"]
    // #[fieldnumber = 5]
    // pub sfixed64: i64,
    // #[prototype = "sfixed64"]
    // #[fieldnumber = 6]
    // pub sfixed64_default: i64,
    // #[prototype = "string"]
    // #[fieldnumber = 7]
    // pub string: String,
    // #[prototype = "string"]
    // #[fieldnumber = 8]
    // pub string_default: String,
    // #[prototype = "uuid"]
    // #[fieldnumber = 9]
    // pub uuid: uuid::Uuid,
    // #[prototype = "bytes"]
    // #[fieldnumber = 10]
    // pub bytes: Vec<u8>,
    // #[prototype = "bytes"]
    // #[fieldnumber = 11]
    // pub bytes_default: Vec<u8>,

    // #[prototype = "enum"]
    // #[fieldnumber = 13]
    // pub optional_enum: Option<some_enum::Result>,
}


impl Location {

}



#[derive(protobuf::StrictMerge, Debug, PartialEq)]
pub struct Announcement {
    #[prototype = "sfixed64"]
    #[number = 1]
    pub ts_created: i64,
}

pub mod some_enum {
    #[derive(PartialEq, Debug)]
    pub enum SomeResult {
        success(f64),
        error(crate::api::ConversationCreateRespError),
        //error(super::ConversationCreateRespError),
    }

    impl ::protobuf::Oneof for SomeResult {}
}

//
// #[derive(PartialEq,protobuf::StrictMerge,Debug)]
// pub struct ConversationCreateSuccess {
//     // message fields
//     #[prototype = "uuid"]
//     #[fieldnumber = 1]
//     pub conversation_uuid: uuid::Uuid,
//     #[prototype = "string"]
//     #[fieldnumber = 2]
//     pub image_id: ::std::string::String,
//     // special fields
// }
//
//
// impl ConversationCreateSuccess {
// }
//
//
//
//
// impl ::protobuf::reflect::ProtobufValue for ConversationCreateSuccess {
// }
//
#[derive(PartialEq, Copy, Clone, Debug)]
pub enum ConversationCreateRespError {
    CONVERSATION_CREATE_RESP_ERROR_NOT_SET = 0,
    TOO_CLOSE_TO_OTHER_CONVERSATION = 1,
}

// TODO: Derive
impl ProtobufEnumStrict for ConversationCreateRespError {
    fn value(self) -> i32 {
        match self {
            ConversationCreateRespError::CONVERSATION_CREATE_RESP_ERROR_NOT_SET => {
                debug_assert!(false, "Unexpected found zero value in ConversationCreateRespError");
                0
            }
            ConversationCreateRespError::TOO_CLOSE_TO_OTHER_CONVERSATION => 1,
        }
    }

    fn from_i32(value: i32) -> ProtobufResult<ConversationCreateRespError> {
        match value {
            1 => Ok(ConversationCreateRespError::TOO_CLOSE_TO_OTHER_CONVERSATION),
            _ => {
                debug_assert!(false, "Unexpectedly found unsupported value {} for type ConversationCreateRespError", value);

                // TODO: Strange return type
                ::protobuf::ProtobufResult::Err(::protobuf::ProtobufError::WireError(::protobuf::error::WireError::IncorrectVarint))
            }
        }
    }
}