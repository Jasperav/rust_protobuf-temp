use protobuf::{ProtobufResult, ProtobufError, ProtobufEnumStrict};
use protobuf::error::ProtobufError::WireError;

#[derive(protobuf::StrictMerge, Debug, PartialEq)]
pub struct Location {
    // #[prototype = "double"]
    // #[fieldnumber = 1]
    // pub double: f64,
    // #[prototype = "double"]
    // #[fieldnumber = 2]
    // pub o_double_default: f64,
    // #[prototype = "enum"]
    // #[fieldnumber = 3]
    // pub _enum: crate::api::ConversationCreateRespError,
    // #[prototype = "enum"]
    // #[fieldnumber = 4]
    // pub o_enum: Option<crate::api::ConversationCreateRespError>,
    #[prototype = "oneof"]
    #[oneof = "success|double|1"]
    #[oneof = "error|enum|2"]
    #[fieldnumber = 12]
    pub one_of: some_enum::SomeResult,
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
    fn strict_merge(is: &mut protobuf::CodedInputStream< \'_ >) -> protobuf::
    ProtobufResult<Self>
    {
        use std::str::FromStr;
        let mut processed_field_indexes = std::
        collections::HashSet::new();
        let mut one_of = None;
        while !is.
            eof()?
        {
            l
        }
        et(field_number, wire_type) = is.read_tag_unpack()?;
        match
        field_number
        {
            1u32 =>
                {
                    debug_assert!(processed_field_indexes . insert(1u32),
                                 \"Double processed field index found for matching field {} (note that fields indexes start with 1, not 0)\",\
                                                  1u32) ; if wire_type != :: protobuf :: wire_format ::\
                                                                  WireTypeFixed64\
                                                                                  {\
                                                                                                      return :: std :: result :: Result ::\
                                                                                                                          Err(:: protobuf :: rt :: unexpected_wire_type(wire_type))\
                                                                                                                                              ;\
                                                                                                                                                              } one_of =\
                                                                                                                                                                              Some(some_enum :: SomeResult ::\
                                                                                                                                                                                                   success(is . read_double() ?)),\
                                                                                                                                                                                                               } 2u32 =>\
                                                                                                                                                                                                                           {\
                                                                                                                                                                                                                                           debug_assert !\
                                                                                                                                                                                                                                                           (processed_field_indexes . insert(2u32),\
                                                                                                                                                                                                                                                                            \"Double processed field index found for matching field {} (note that fields indexes start with 1, not 0)\",\
                                                                                                                                                                                                                                                                                             2u32) ; if wire_type != :: protobuf :: wire_format ::\
                                                                                                                                                                                                                                                                                                             WireTypeVarint\
                                                                                                                                                                                                                                                                                                                             {\
                                                                                                                                                                                                                                                                                                                                                 return :: std :: result :: Result ::\
                                                                                                                                                                                                                                                                                                                                                                     Err(:: protobuf :: rt :: unexpected_wire_type(wire_type))\
                                                                                                                                                                                                                                                                                                                                                                                         ;\
                                                                                                                                                                                                                                                                                                                                                                                                         } one_of =\
                                                                                                                                                                                                                                                                                                                                                                                                                         Some(some_enum :: SomeResult ::\
                                                                                                                                                                                                                                                                                                                                                                                                                                              error(is . read_enum_strict() ?)),\
                                                                                                                                                                                                                                                                                                                                                                                                                                                          } _ =>\
                                                                                                                                                                                                                                                                                                                                                                                                                                                                      {\
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      debug_assert !\
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      (false, \"number: {:#?}, wire_type: {:#?}\", field_number,\
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       wire_type) ; return :: protobuf :: ProtobufResult ::\
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       Err(:: protobuf :: ProtobufError ::\
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           WireError(:: protobuf :: error :: WireError ::\
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         IncorrectVarint)) ;\
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     }\
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             }\
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 } if one_of . is_none()\
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     {\
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             debug_assert !\
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     (false,\
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              \"Unexpected empty optional found while deserializing property {}\",\
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       stringify ! (one_of)) ; return :: protobuf :: ProtobufResult ::\
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               Err(:: protobuf :: ProtobufError ::\
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           WireError(:: protobuf :: error :: WireError :: IncorrectVarint)) ;\
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               } let gen_struct = Location { one_of : one_of . unwrap() } ; :: std ::\
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   result :: Result :: Ok(gen_struct)
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   }
            }
}
}
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
    fn value(&self) -> i32 {
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