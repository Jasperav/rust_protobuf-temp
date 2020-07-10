use protobuf::{ProtobufResult, ProtobufError, ProtobufEnumStrict};
use protobuf::error::ProtobufError::WireError;
use std::fmt::{Debug, Display};

#[derive(Debug, PartialEq)]
pub struct Location {
    pub double_default: f64,
    pub double_non_default: f64,
    pub enum_default: crate::api::ConversationCreateRespError,
    pub enum_o_default: Option<crate::api::ConversationCreateRespError>,
    pub enum_o_empty: Option<crate::api::ConversationCreateRespError>,
    pub one_of_double: some_enum::SomeResult,
    pub one_of_enum: some_enum::SomeResult,
    pub one_of_message: some_enum::SomeResult,
    pub message_default: Announcement,
    pub message_o_default: Option<Announcement>,
    pub message_o_empty: Option<Announcement>,

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





impl protobuf::StrictMerge<Location> for Location {
    fn strict_merge(is: &mut protobuf::CodedInputStream<'_>) -> protobuf::ProtobufResult<Self> {
        use std::str::FromStr;
        let mut processed_field_indexes = std::collections::HashSet::new();
        let mut double_default = 0 as f64;
        let mut double_non_default = 0 as f64;
        let mut enum_default = None;
        let mut enum_o_default = None;
        let mut enum_o_empty = None;
        let mut one_of_double = None;
        let mut one_of_enum = None;
        let mut one_of_message = None;
        let mut message_default = None;
        let mut message_o_default = None;
        let mut message_o_empty = None;
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1u32 => {
                    if true {
                        if !processed_field_indexes.insert(1u32) {
                            {
                                :: std :: rt :: begin_panic_fmt ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "Double processed field index found for matching field " , " (note that fields indexes start with 1, not 0)" ] , & match ( & 1u32 , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Display :: fmt ) ] , } ) )
                            }
                        };
                    };
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(
                            ::protobuf::rt::unexpected_wire_type(wire_type),
                        );
                    }
                    double_default = is.read_double()?;
                }
                2u32 => {
                    if true {
                        if !processed_field_indexes.insert(2u32) {
                            {
                                :: std :: rt :: begin_panic_fmt ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "Double processed field index found for matching field " , " (note that fields indexes start with 1, not 0)" ] , & match ( & 2u32 , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Display :: fmt ) ] , } ) )
                            }
                        };
                    };
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(
                            ::protobuf::rt::unexpected_wire_type(wire_type),
                        );
                    }
                    double_non_default = is.read_double()?;
                }
                3u32 => {
                    if true {
                        if !processed_field_indexes.insert(3u32) {
                            {
                                :: std :: rt :: begin_panic_fmt ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "Double processed field index found for matching field " , " (note that fields indexes start with 1, not 0)" ] , & match ( & 3u32 , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Display :: fmt ) ] , } ) )
                            }
                        };
                    };
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(
                            ::protobuf::rt::unexpected_wire_type(wire_type),
                        );
                    }
                    enum_default = Some(is.read_enum_strict()?);
                }
                4u32 => {
                    if true {
                        if !processed_field_indexes.insert(4u32) {
                            {
                                :: std :: rt :: begin_panic_fmt ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "Double processed field index found for matching field " , " (note that fields indexes start with 1, not 0)" ] , & match ( & 4u32 , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Display :: fmt ) ] , } ) )
                            }
                        };
                    };
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(
                            ::protobuf::rt::unexpected_wire_type(wire_type),
                        );
                    }
                    enum_o_default = Some(is.read_enum_strict()?);
                }
                5u32 => {
                    if true {
                        if !processed_field_indexes.insert(5u32) {
                            {
                                :: std :: rt :: begin_panic_fmt ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "Double processed field index found for matching field " , " (note that fields indexes start with 1, not 0)" ] , & match ( & 5u32 , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Display :: fmt ) ] , } ) )
                            }
                        };
                    };
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(
                            ::protobuf::rt::unexpected_wire_type(wire_type),
                        );
                    }
                    enum_o_empty = Some(is.read_enum_strict()?);
                }
                6u32 => {
                    if true {
                        if !processed_field_indexes.insert(6u32) {
                            {
                                :: std :: rt :: begin_panic_fmt ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "Double processed field index found for matching field " , " (note that fields indexes start with 1, not 0)" ] , & match ( & 6u32 , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Display :: fmt ) ] , } ) )
                            }
                        };
                    };
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(
                            ::protobuf::rt::unexpected_wire_type(wire_type),
                        );
                    }
                    one_of_double = Some(some_enum::SomeResult::double(is.read_double()?));
                }
                7u32 => {
                    if true {
                        if !processed_field_indexes.insert(7u32) {
                            {
                                :: std :: rt :: begin_panic_fmt ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "Double processed field index found for matching field " , " (note that fields indexes start with 1, not 0)" ] , & match ( & 7u32 , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Display :: fmt ) ] , } ) )
                            }
                        };
                    };
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(
                            ::protobuf::rt::unexpected_wire_type(wire_type),
                        );
                    }
                    one_of_double = Some(some_enum::SomeResult::_enum(is.read_enum_strict()?));
                }
                8u32 => {
                    if true {
                        if !processed_field_indexes.insert(8u32) {
                            {
                                :: std :: rt :: begin_panic_fmt ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "Double processed field index found for matching field " , " (note that fields indexes start with 1, not 0)" ] , & match ( & 8u32 , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Display :: fmt ) ] , } ) )
                            }
                        };
                    };
                    one_of_double = Some(some_enum::SomeResult::message(
                        ::protobuf::rt::read_message::<_>(wire_type, is)?,
                    ));
                }
                9u32 => {
                    if true {
                        if !processed_field_indexes.insert(9u32) {
                            {
                                :: std :: rt :: begin_panic_fmt ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "Double processed field index found for matching field " , " (note that fields indexes start with 1, not 0)" ] , & match ( & 9u32 , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Display :: fmt ) ] , } ) )
                            }
                        };
                    };
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(
                            ::protobuf::rt::unexpected_wire_type(wire_type),
                        );
                    }
                    one_of_enum = Some(some_enum::SomeResult::double(is.read_double()?));
                }
                10u32 => {
                    if true {
                        if !processed_field_indexes.insert(10u32) {
                            {
                                :: std :: rt :: begin_panic_fmt ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "Double processed field index found for matching field " , " (note that fields indexes start with 1, not 0)" ] , & match ( & 10u32 , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Display :: fmt ) ] , } ) )
                            }
                        };
                    };
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(
                            ::protobuf::rt::unexpected_wire_type(wire_type),
                        );
                    }
                    one_of_enum = Some(some_enum::SomeResult::_enum(is.read_enum_strict()?));
                }
                11u32 => {
                    if true {
                        if !processed_field_indexes.insert(11u32) {
                            {
                                :: std :: rt :: begin_panic_fmt ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "Double processed field index found for matching field " , " (note that fields indexes start with 1, not 0)" ] , & match ( & 11u32 , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Display :: fmt ) ] , } ) )
                            }
                        };
                    };
                    one_of_enum = Some(some_enum::SomeResult::message(
                        ::protobuf::rt::read_message::<_>(wire_type, is)?,
                    ));
                }
                12u32 => {
                    if true {
                        if !processed_field_indexes.insert(12u32) {
                            {
                                :: std :: rt :: begin_panic_fmt ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "Double processed field index found for matching field " , " (note that fields indexes start with 1, not 0)" ] , & match ( & 12u32 , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Display :: fmt ) ] , } ) )
                            }
                        };
                    };
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(
                            ::protobuf::rt::unexpected_wire_type(wire_type),
                        );
                    }
                    one_of_message = Some(some_enum::SomeResult::double(is.read_double()?));
                }
                13u32 => {
                    if true {
                        if !processed_field_indexes.insert(13u32) {
                            {
                                :: std :: rt :: begin_panic_fmt ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "Double processed field index found for matching field " , " (note that fields indexes start with 1, not 0)" ] , & match ( & 13u32 , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Display :: fmt ) ] , } ) )
                            }
                        };
                    };
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(
                            ::protobuf::rt::unexpected_wire_type(wire_type),
                        );
                    }
                    one_of_message = Some(some_enum::SomeResult::_enum(is.read_enum_strict()?));
                }
                14u32 => {
                    if true {
                        if !processed_field_indexes.insert(14u32) {
                            {
                                :: std :: rt :: begin_panic_fmt ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "Double processed field index found for matching field " , " (note that fields indexes start with 1, not 0)" ] , & match ( & 14u32 , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Display :: fmt ) ] , } ) )
                            }
                        };
                    };
                    one_of_message = Some(some_enum::SomeResult::message(
                        ::protobuf::rt::read_message::<_>(wire_type, is)?,
                    ));
                }
                15u32 => {
                    if true {
                        if !processed_field_indexes.insert(15u32) {
                            {
                                :: std :: rt :: begin_panic_fmt ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "Double processed field index found for matching field " , " (note that fields indexes start with 1, not 0)" ] , & match ( & 15u32 , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Display :: fmt ) ] , } ) )
                            }
                        };
                    };
                    message_default =
                        Some(::protobuf::rt::read_message::<Announcement>(wire_type, is)?);
                }
                16u32 => {
                    if true {
                        if !processed_field_indexes.insert(16u32) {
                            {
                                :: std :: rt :: begin_panic_fmt ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "Double processed field index found for matching field " , " (note that fields indexes start with 1, not 0)" ] , & match ( & 16u32 , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Display :: fmt ) ] , } ) )
                            }
                        };
                    };
                    message_o_default =
                        Some(::protobuf::rt::read_message::<Announcement>(wire_type, is)?);
                }
                17u32 => {
                    if true {
                        if !processed_field_indexes.insert(17u32) {
                            {
                                :: std :: rt :: begin_panic_fmt ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "Double processed field index found for matching field " , " (note that fields indexes start with 1, not 0)" ] , & match ( & 17u32 , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Display :: fmt ) ] , } ) )
                            }
                        };
                    };
                    message_o_empty =
                        Some(::protobuf::rt::read_message::<Announcement>(wire_type, is)?);
                }
                _ => {
                    if true {
                        if !false {
                            {
                                ::std::rt::begin_panic_fmt(
                                    &::core::fmt::Arguments::new_v1_formatted(
                                        &["number: ", ", wire_type: "],
                                        &match (&field_number, &wire_type) {
                                            (arg0, arg1) => [
                                                ::core::fmt::ArgumentV1::new(
                                                    arg0,
                                                    ::core::fmt::Debug::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg1,
                                                    ::core::fmt::Debug::fmt,
                                                ),
                                            ],
                                        },
                                        &[
                                            ::core::fmt::rt::v1::Argument {
                                                position: 0usize,
                                                format: ::core::fmt::rt::v1::FormatSpec {
                                                    fill: ' ',
                                                    align:
                                                    ::core::fmt::rt::v1::Alignment::Unknown,
                                                    flags: 4u32,
                                                    precision:
                                                    ::core::fmt::rt::v1::Count::Implied,
                                                    width: ::core::fmt::rt::v1::Count::Implied,
                                                },
                                            },
                                            ::core::fmt::rt::v1::Argument {
                                                position: 1usize,
                                                format: ::core::fmt::rt::v1::FormatSpec {
                                                    fill: ' ',
                                                    align:
                                                    ::core::fmt::rt::v1::Alignment::Unknown,
                                                    flags: 4u32,
                                                    precision:
                                                    ::core::fmt::rt::v1::Count::Implied,
                                                    width: ::core::fmt::rt::v1::Count::Implied,
                                                },
                                            },
                                        ],
                                    ),
                                )
                            }
                        };
                    };
                    return ::protobuf::ProtobufResult::Err(
                        ::protobuf::ProtobufError::WireError(
                            ::protobuf::error::WireError::IncorrectVarint,
                        ),
                    );
                }
            }
        }
        if enum_default.is_none() {
            if true {
                if !false {
                    {
                        ::std::rt::begin_panic_fmt(&::core::fmt::Arguments::new_v1(
                            &["Unexpected empty optional found while deserializing property "],
                            &match (&"enum_default",) {
                                (arg0,) => [::core::fmt::ArgumentV1::new(
                                    arg0,
                                    ::core::fmt::Display::fmt,
                                )],
                            },
                        ))
                    }
                };
            };
            return ::protobuf::ProtobufResult::Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::IncorrectVarint,
            ));
        }
        if one_of_double.is_none() {
            if true {
                if !false {
                    {
                        ::std::rt::begin_panic_fmt(&::core::fmt::Arguments::new_v1(
                            &["Unexpected empty optional found while deserializing property "],
                            &match (&"one_of_double",) {
                                (arg0,) => [::core::fmt::ArgumentV1::new(
                                    arg0,
                                    ::core::fmt::Display::fmt,
                                )],
                            },
                        ))
                    }
                };
            };
            return ::protobuf::ProtobufResult::Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::IncorrectVarint,
            ));
        }
        if one_of_enum.is_none() {
            if true {
                if !false {
                    {
                        ::std::rt::begin_panic_fmt(&::core::fmt::Arguments::new_v1(
                            &["Unexpected empty optional found while deserializing property "],
                            &match (&"one_of_enum",) {
                                (arg0,) => [::core::fmt::ArgumentV1::new(
                                    arg0,
                                    ::core::fmt::Display::fmt,
                                )],
                            },
                        ))
                    }
                };
            };
            return ::protobuf::ProtobufResult::Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::IncorrectVarint,
            ));
        }
        if one_of_message.is_none() {
            if true {
                if !false {
                    {
                        ::std::rt::begin_panic_fmt(&::core::fmt::Arguments::new_v1(
                            &["Unexpected empty optional found while deserializing property "],
                            &match (&"one_of_message",) {
                                (arg0,) => [::core::fmt::ArgumentV1::new(
                                    arg0,
                                    ::core::fmt::Display::fmt,
                                )],
                            },
                        ))
                    }
                };
            };
            return ::protobuf::ProtobufResult::Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::IncorrectVarint,
            ));
        }
        if message_default.is_none() {
            if true {
                if !false {
                    {
                        ::std::rt::begin_panic_fmt(&::core::fmt::Arguments::new_v1(
                            &["Unexpected empty optional found while deserializing property "],
                            &match (&"message_default",) {
                                (arg0,) => [::core::fmt::ArgumentV1::new(
                                    arg0,
                                    ::core::fmt::Display::fmt,
                                )],
                            },
                        ))
                    }
                };
            };
            return ::protobuf::ProtobufResult::Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::IncorrectVarint,
            ));
        }
        let gen_struct = Location {
            double_default: double_default,
            double_non_default: double_non_default,
            enum_default: enum_default.unwrap(),
            enum_o_default: enum_o_default,
            enum_o_empty: enum_o_empty,
            one_of_double: one_of_double.unwrap(),
            one_of_enum: one_of_enum.unwrap(),
            one_of_message: one_of_message.unwrap(),
            message_default: message_default.unwrap(),
            message_o_default: message_o_default,
            message_o_empty: message_o_empty,
        };
        ::std::result::Result::Ok(gen_struct)
    }
    fn write_to_os(
        &self,
        os: &mut ::protobuf::CodedOutputStream<'_>,
    ) -> ::protobuf::ProtobufResult<()> {
        if self.double_default != 0 as f64 {
            os.write_double(1u32, self.double_default)?;
        }
        if self.double_non_default != 0 as f64 {
            os.write_double(2u32, self.double_non_default)?;
        }
        os.write_enum(
            3u32,
            ::protobuf::ProtobufEnumStrict::value(self.enum_default),
        )?;
        if let Some(e) = &self.enum_o_default {
            os.write_enum(4u32, ::protobuf::ProtobufEnumStrict::value(*e))?;
        }
        if let Some(e) = &self.enum_o_empty {
            os.write_enum(5u32, ::protobuf::ProtobufEnumStrict::value(*e))?;
        }
        match &self.one_of_double {
            &some_enum::SomeResult::double(dummy_ident) => {
                os.write_double(6u32, dummy_ident)?;
            }
            &some_enum::SomeResult::_enum(dummy_ident) => {
                os.write_enum(7u32, ::protobuf::ProtobufEnumStrict::value(dummy_ident))?;
            }
            &some_enum::SomeResult::message(ref dummy_ident) => {
                ::protobuf::rt::write_strict_message_field_with_cached_size(
                    8u32,
                    dummy_ident,
                    os,
                )?;
            }
        }
        match &self.one_of_enum {
            &some_enum::SomeResult::double(dummy_ident) => {
                os.write_double(9u32, dummy_ident)?;
            }
            &some_enum::SomeResult::_enum(dummy_ident) => {
                os.write_enum(10u32, ::protobuf::ProtobufEnumStrict::value(dummy_ident))?;
            }
            &some_enum::SomeResult::message(ref dummy_ident) => {
                ::protobuf::rt::write_strict_message_field_with_cached_size(
                    11u32,
                    dummy_ident,
                    os,
                )?;
            }
        }
        match &self.one_of_message {
            &some_enum::SomeResult::double(dummy_ident) => {
                os.write_double(12u32, dummy_ident)?;
            }
            &some_enum::SomeResult::_enum(dummy_ident) => {
                os.write_enum(13u32, ::protobuf::ProtobufEnumStrict::value(dummy_ident))?;
            }
            &some_enum::SomeResult::message(ref dummy_ident) => {
                ::protobuf::rt::write_strict_message_field_with_cached_size(
                    14u32,
                    dummy_ident,
                    os,
                )?;
            }
        }
        ::protobuf::rt::write_strict_message_field_with_cached_size(
            15u32,
            &self.message_default,
            os,
        )?;
        if let Some(e) = &self.message_o_default {
            ::protobuf::rt::write_strict_message_field_with_cached_size(16u32, e, os)?;
        }
        if let Some(e) = &self.message_o_empty {
            ::protobuf::rt::write_strict_message_field_with_cached_size(17u32, e, os)?;
        }
        ::std::result::Result::Ok(())
    }
    fn compute_size(&self) -> usize {
        let mut size = 0;
        if self.double_default != 0 as f64 {
            size += 9;
        }
        if self.double_non_default != 0 as f64 {
            size += 9;
        }
        if true {
            {
                match (&0, &self.enum_default.value()) {
                    (left_val, right_val) => {
                        if *left_val == *right_val {
                            {
                                ::std::rt::begin_panic_fmt(&::core::fmt::Arguments::new_v1(
                                    &[
                                        "assertion failed: `(left != right)`\n  left: `",
                                        "`,\n right: `",
                                        "`",
                                    ],
                                    &match (&&*left_val, &&*right_val) {
                                        (arg0, arg1) => [
                                            ::core::fmt::ArgumentV1::new(
                                                arg0,
                                                ::core::fmt::Debug::fmt,
                                            ),
                                            ::core::fmt::ArgumentV1::new(
                                                arg1,
                                                ::core::fmt::Debug::fmt,
                                            ),
                                        ],
                                    },
                                ))
                            }
                        }
                    }
                }
            };
        };
        size += ::protobuf::rt::enum_size_strict(3u32, self.enum_default);
        if let Some(e) = &self.enum_o_default {
            if true {
                {
                    match (&0, &e.value()) {
                        (left_val, right_val) => {
                            if *left_val == *right_val {
                                {
                                    ::std::rt::begin_panic_fmt(&::core::fmt::Arguments::new_v1(
                                        &[
                                            "assertion failed: `(left != right)`\n  left: `",
                                            "`,\n right: `",
                                            "`",
                                        ],
                                        &match (&&*left_val, &&*right_val) {
                                            (arg0, arg1) => [
                                                ::core::fmt::ArgumentV1::new(
                                                    arg0,
                                                    ::core::fmt::Debug::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg1,
                                                    ::core::fmt::Debug::fmt,
                                                ),
                                            ],
                                        },
                                    ))
                                }
                            }
                        }
                    }
                };
            };
            size += ::protobuf::rt::enum_size_strict(4u32, *e);
        }
        if let Some(e) = &self.enum_o_empty {
            if true {
                {
                    match (&0, &e.value()) {
                        (left_val, right_val) => {
                            if *left_val == *right_val {
                                {
                                    ::std::rt::begin_panic_fmt(&::core::fmt::Arguments::new_v1(
                                        &[
                                            "assertion failed: `(left != right)`\n  left: `",
                                            "`,\n right: `",
                                            "`",
                                        ],
                                        &match (&&*left_val, &&*right_val) {
                                            (arg0, arg1) => [
                                                ::core::fmt::ArgumentV1::new(
                                                    arg0,
                                                    ::core::fmt::Debug::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg1,
                                                    ::core::fmt::Debug::fmt,
                                                ),
                                            ],
                                        },
                                    ))
                                }
                            }
                        }
                    }
                };
            };
            size += ::protobuf::rt::enum_size_strict(5u32, *e);
        }
        match &self.one_of_double {
            &some_enum::SomeResult::double(dummy_ident) => {
                size += 9;
            }
            &some_enum::SomeResult::_enum(dummy_ident) => {
                if true {
                    {
                        match (&0, &dummy_ident.value()) {
                            (left_val, right_val) => {
                                if *left_val == *right_val {
                                    {
                                        :: std :: rt :: begin_panic_fmt ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "assertion failed: `(left != right)`\n  left: `" , "`,\n right: `" , "`" ] , & match ( & & * left_val , & & * right_val ) { ( arg0 , arg1 ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Debug :: fmt ) , :: core :: fmt :: ArgumentV1 :: new ( arg1 , :: core :: fmt :: Debug :: fmt ) ] , } ) )
                                    }
                                }
                            }
                        }
                    };
                };
                size += ::protobuf::rt::enum_size_strict(7u32, dummy_ident);
            }
            &some_enum::SomeResult::message(ref dummy_ident) => {
                let len = dummy_ident.compute_size() as u32;
                size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
            }
        }
        match &self.one_of_enum {
            &some_enum::SomeResult::double(dummy_ident) => {
                size += 9;
            }
            &some_enum::SomeResult::_enum(dummy_ident) => {
                if true {
                    {
                        match (&0, &dummy_ident.value()) {
                            (left_val, right_val) => {
                                if *left_val == *right_val {
                                    {
                                        :: std :: rt :: begin_panic_fmt ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "assertion failed: `(left != right)`\n  left: `" , "`,\n right: `" , "`" ] , & match ( & & * left_val , & & * right_val ) { ( arg0 , arg1 ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Debug :: fmt ) , :: core :: fmt :: ArgumentV1 :: new ( arg1 , :: core :: fmt :: Debug :: fmt ) ] , } ) )
                                    }
                                }
                            }
                        }
                    };
                };
                size += ::protobuf::rt::enum_size_strict(10u32, dummy_ident);
            }
            &some_enum::SomeResult::message(ref dummy_ident) => {
                let len = dummy_ident.compute_size() as u32;
                size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
            }
        }
        match &self.one_of_message {
            &some_enum::SomeResult::double(dummy_ident) => {
                size += 9;
            }
            &some_enum::SomeResult::_enum(dummy_ident) => {
                if true {
                    {
                        match (&0, &dummy_ident.value()) {
                            (left_val, right_val) => {
                                if *left_val == *right_val {
                                    {
                                        :: std :: rt :: begin_panic_fmt ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "assertion failed: `(left != right)`\n  left: `" , "`,\n right: `" , "`" ] , & match ( & & * left_val , & & * right_val ) { ( arg0 , arg1 ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Debug :: fmt ) , :: core :: fmt :: ArgumentV1 :: new ( arg1 , :: core :: fmt :: Debug :: fmt ) ] , } ) )
                                    }
                                }
                            }
                        }
                    };
                };
                size += ::protobuf::rt::enum_size_strict(13u32, dummy_ident);
            }
            &some_enum::SomeResult::message(ref dummy_ident) => {
                let len = dummy_ident.compute_size() as u32;
                size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
            }
        }
        let len = self.message_default.compute_size() as u32;
        size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        if let Some(e) = &self.message_o_default {
            let len = e.compute_size() as u32;
            size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(e) = &self.message_o_empty {
            let len = e.compute_size() as u32;
            size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        size as usize
    }
}




impl Location {

}


#[derive(protobuf::StrictMerge, Debug, PartialEq)]
pub struct Announcement {
    #[prototype = "double"]
    #[fieldnumber = 1]
    pub ts_created: f64,
}

pub mod some_enum {
    #[derive(PartialEq, Debug)]
    pub enum SomeResult {
        double(f64),
        _enum(crate::api::ConversationCreateRespError),
        message(crate::api::Announcement),
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