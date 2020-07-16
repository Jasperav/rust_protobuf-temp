use protobuf::{ProtobufResult, ProtobufError, ProtobufEnumStrict};
use protobuf::error::ProtobufError::WireError;
use std::fmt::{Debug, Display};
use crate::other_messages::{compound, Inner};

#[derive(protobuf::StrictMerge, Debug, PartialEq)]
pub struct Compound {
    #[prototype = "double"]
    #[fieldnumber = 1]
    pub double_default: f64,
    #[prototype = "double"]
    #[fieldnumber = 2]
    pub double_non_default: f64,

    #[prototype = "enum"]
    #[fieldnumber = 3]
    pub enum_default: crate::other_messages::AnEnum,
    #[prototype = "enum"]
    #[fieldnumber = 4]
    pub enum_o_default: Option<crate::other_messages::AnEnum>,
    #[prototype = "enum"]
    #[fieldnumber = 5]
    pub enum_o_empty: Option<crate::other_messages::AnEnum>,

    #[prototype = "message"]
    #[fieldnumber = 6]
    #[tagsize = 1]
    pub message_default: Inner,
    #[prototype = "message"]
    #[fieldnumber = 7]
    #[tagsize = 2]
    pub message_o_default: Option<Inner>,
    #[prototype = "message"]
    #[fieldnumber = 8]
    #[tagsize = 2]
    pub message_o_empty: Option<Inner>,

    #[prototype = "oneof"]
    #[oneof = "double|double|9"]
    #[oneof = "_enum|enum|10"]
    #[oneof = "message|message|11|1"]
    pub one_of_double: compound::OneOfSomething,
    #[prototype = "oneof"]
    #[oneof = "double|double|12"]
    #[oneof = "_enum|enum|13"]
    #[oneof = "message|message|14|1"]
    pub one_of_enum: compound::OneOfSomething,
    #[prototype = "oneof"]
    #[oneof = "double|double|15"]
    #[oneof = "_enum|enum|16"]
    #[oneof = "message|message|17|1"]
    pub one_of_message: compound::OneOfSomething,

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