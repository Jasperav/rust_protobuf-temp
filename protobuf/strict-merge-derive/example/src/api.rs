use protobuf::{ProtobufResult, ProtobufError, ProtobufEnumStrict};
use protobuf::error::ProtobufError::WireError;
use std::fmt::{Debug, Display};
use crate::other_messages::{compound, Inner};

#[derive(Debug, PartialEq)]
#[derive(protobuf::StrictMerge)]
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
    #[tagsize = 1]
    pub message_o_default: Option<Inner>,
    #[prototype = "message"]
    #[fieldnumber = 8]
    #[tagsize = 1]
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
    #[oneof = "message|message|17|2"]
    pub one_of_message: compound::OneOfSomething,

    #[prototype = "u8"]
    #[fieldnumber = 18]
    pub bytes_default: ::std::vec::Vec<u8>,
    #[prototype = "u8"]
    #[fieldnumber = 19]
    pub bytes_o_default: ::std::vec::Vec<u8>,
    #[prototype = "u8"]
    #[fieldnumber = 20]
    pub bytes_o_empty: ::std::vec::Vec<u8>,

    #[prototype = "repeated"]
    #[repeatedinner = "double"]
    #[tagsize = 10]
    #[fieldnumber = 21]
    pub vec_double_default: ::std::vec::Vec<f64>,
    #[prototype = "repeated"]
    #[repeatedinner = "double"]
    #[tagsize = 10]
    #[fieldnumber = 22]
    pub vec_double_o_default: ::std::vec::Vec<f64>,
    #[prototype = "repeated"]
    #[repeatedinner = "double"]
    #[tagsize = 10]
    #[fieldnumber = 23]
    pub vec_double_o_empty: ::std::vec::Vec<f64>,

    #[prototype = "repeated"]
    #[repeatedinner = "message"]
    #[tagsize = 2]
    #[fieldnumber = 24]
    pub vec_message_default: ::std::vec::Vec<Inner>,
    #[prototype = "repeated"]
    #[repeatedinner = "message"]
    #[tagsize = 2]
    #[fieldnumber = 25]
    pub vec_message_o_default: ::std::vec::Vec<Inner>,
    #[prototype = "repeated"]
    #[repeatedinner = "message"]
    #[tagsize = 2]
    #[fieldnumber = 26]
    pub vec_message_o_empty: ::std::vec::Vec<Inner>,
}