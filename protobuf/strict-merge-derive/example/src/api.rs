use protobuf::{ProtobufResult, ProtobufError, ProtobufEnumStrict};
use protobuf::error::ProtobufError::WireError;
use std::fmt::{Debug, Display};
use crate::other_messages::{compound, Inner};


#[derive(Debug, PartialEq)]
pub struct Compound {
    //
    //
    // pub double_default: f64,
    //
    //
    // pub double_non_default: f64,
    //
    //
    //
    // pub enum_default: crate::other_messages::AnEnum,
    //
    //
    // pub enum_o_default: Option<crate::other_messages::AnEnum>,
    //
    //
    // pub enum_o_empty: Option<crate::other_messages::AnEnum>,
    //
    //
    //
    //
    // pub message_default: Inner,
    //
    //
    //
    // pub message_o_default: Option<Inner>,
    //
    //
    //
    // pub message_o_empty: Option<Inner>,





    pub one_of_double: compound::OneOfSomething,




    pub one_of_enum: compound::OneOfSomething,




    pub one_of_message: compound::OneOfSomething,



    pub bytes_default: ::std::vec::Vec<u8>,


    pub bytes_o_default: ::std::vec::Vec<u8>,


    pub bytes_o_empty: ::std::vec::Vec<u8>,
}