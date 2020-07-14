use protobuf::{ProtobufEnumStrict, ProtobufResult};

#[derive(protobuf::StrictMerge, Debug, PartialEq, Clone)]
pub struct Inner {
    #[prototype = "double"]
    #[fieldnumber = 1]
    #[tagsize = 10]
    pub double_default: f64,
}

pub mod compound {

    #[derive(PartialEq, Debug)]
    pub enum OneOfSomething {
        double(f64),
        _enum(crate::other_messages::AnEnum),
        message(crate::other_messages::Inner),
        //error(super::ConversationCreateRespError),
    }

    impl ::protobuf::Oneof for OneOfSomething {}
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
pub enum AnEnum {
    NotSet = 0,
    ACase = 1,
    AnotherCase = 2,
}

// TODO: Derive
impl ProtobufEnumStrict for AnEnum {
    fn value(self) -> i32 {
        match self {
            AnEnum::NotSet => {
                debug_assert!(false, "Unexpected found zero value in ConversationCreateRespError");
                0
            }
            AnEnum::ACase => 1,
            AnEnum::AnotherCase => 2
        }
    }

    fn from_i32(value: i32) -> ProtobufResult<AnEnum> {
        match value {
            1 => Ok(AnEnum::ACase),
            2 => Ok(AnEnum::AnotherCase),
            _ => {
                debug_assert!(false, "Unexpectedly found unsupported value {} for type ConversationCreateRespError", value);

                // TODO: Strange return type
                ::protobuf::ProtobufResult::Err(::protobuf::ProtobufError::WireError(::protobuf::error::WireError::IncorrectVarint))
            }
        }
    }
}