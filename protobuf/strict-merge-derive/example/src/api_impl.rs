use std::fmt::{Display, Debug};
use crate::api::Compound;
use crate::other_messages::compound;
use protobuf::ProtobufEnumStrict;

impl protobuf::StrictMerge<Compound> for Compound {
    fn strict_merge(is: &mut protobuf::CodedInputStream<'_>) -> protobuf::ProtobufResult<Self> {
        use std::str::FromStr;
        let mut processed_field_indexes = std::collections::HashSet::new();
        let mut one_of_double = None;
        let mut one_of_enum = None;
        let mut one_of_message = None;
        let mut bytes_default = vec![];
        let mut bytes_o_default = vec![];
        let mut bytes_o_empty = vec![];
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
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
                    one_of_double = Some(compound::OneOfSomething::double(is.read_double()?));
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
                    one_of_double =
                        Some(compound::OneOfSomething::_enum(is.read_enum_strict()?));
                }
                11u32 => {
                    if true {
                        if !processed_field_indexes.insert(11u32) {
                            {
                                :: std :: rt :: begin_panic_fmt ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "Double processed field index found for matching field " , " (note that fields indexes start with 1, not 0)" ] , & match ( & 11u32 , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Display :: fmt ) ] , } ) )
                            }
                        };
                    };
                    one_of_double = Some(compound::OneOfSomething::message(
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
                    one_of_enum = Some(compound::OneOfSomething::double(is.read_double()?));
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
                    one_of_enum = Some(compound::OneOfSomething::_enum(is.read_enum_strict()?));
                }
                14u32 => {
                    if true {
                        if !processed_field_indexes.insert(14u32) {
                            {
                                :: std :: rt :: begin_panic_fmt ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "Double processed field index found for matching field " , " (note that fields indexes start with 1, not 0)" ] , & match ( & 14u32 , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Display :: fmt ) ] , } ) )
                            }
                        };
                    };
                    one_of_enum = Some(compound::OneOfSomething::message(
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
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(
                            ::protobuf::rt::unexpected_wire_type(wire_type),
                        );
                    }
                    one_of_message = Some(compound::OneOfSomething::double(is.read_double()?));
                }
                16u32 => {
                    if true {
                        if !processed_field_indexes.insert(16u32) {
                            {
                                :: std :: rt :: begin_panic_fmt ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "Double processed field index found for matching field " , " (note that fields indexes start with 1, not 0)" ] , & match ( & 16u32 , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Display :: fmt ) ] , } ) )
                            }
                        };
                    };
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(
                            ::protobuf::rt::unexpected_wire_type(wire_type),
                        );
                    }
                    one_of_message =
                        Some(compound::OneOfSomething::_enum(is.read_enum_strict()?));
                }
                17u32 => {
                    if true {
                        if !processed_field_indexes.insert(17u32) {
                            {
                                :: std :: rt :: begin_panic_fmt ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "Double processed field index found for matching field " , " (note that fields indexes start with 1, not 0)" ] , & match ( & 17u32 , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Display :: fmt ) ] , } ) )
                            }
                        };
                    };
                    one_of_message = Some(compound::OneOfSomething::message(
                        ::protobuf::rt::read_message::<_>(wire_type, is)?,
                    ));
                }
                18u32 => {
                    if true {
                        if !processed_field_indexes.insert(18u32) {
                            {
                                :: std :: rt :: begin_panic_fmt ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "Double processed field index found for matching field " , " (note that fields indexes start with 1, not 0)" ] , & match ( & 18u32 , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Display :: fmt ) ] , } ) )
                            }
                        };
                    };
                    bytes_default = ::protobuf::rt::read_singular_proto3_bytes(wire_type, is)?;
                }
                19u32 => {
                    if true {
                        if !processed_field_indexes.insert(19u32) {
                            {
                                :: std :: rt :: begin_panic_fmt ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "Double processed field index found for matching field " , " (note that fields indexes start with 1, not 0)" ] , & match ( & 19u32 , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Display :: fmt ) ] , } ) )
                            }
                        };
                    };
                    bytes_o_default =
                        ::protobuf::rt::read_singular_proto3_bytes(wire_type, is)?;
                }
                20u32 => {
                    if true {
                        if !processed_field_indexes.insert(20u32) {
                            {
                                :: std :: rt :: begin_panic_fmt ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "Double processed field index found for matching field " , " (note that fields indexes start with 1, not 0)" ] , & match ( & 20u32 , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Display :: fmt ) ] , } ) )
                            }
                        };
                    };
                    bytes_o_empty = ::protobuf::rt::read_singular_proto3_bytes(wire_type, is)?;
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
        let gen_struct = Compound {
            one_of_double: one_of_double.unwrap(),
            one_of_enum: one_of_enum.unwrap(),
            one_of_message: one_of_message.unwrap(),
            bytes_default: bytes_default,
            bytes_o_default: bytes_o_default,
            bytes_o_empty: bytes_o_empty,
        };
        ::std::result::Result::Ok(gen_struct)
    }
    fn write_to_os(
        &self,
        os: &mut ::protobuf::CodedOutputStream<'_>,
    ) -> ::protobuf::ProtobufResult<()> {
        match &self.one_of_double {
            &compound::OneOfSomething::double(dummy_ident) => {
                os.write_double(9u32, dummy_ident)?;
            }
            &compound::OneOfSomething::_enum(dummy_ident) => {
                os.write_enum(10u32, ::protobuf::ProtobufEnumStrict::value(dummy_ident))?;
            }
            &compound::OneOfSomething::message(ref dummy_ident) => {
                ::protobuf::rt::write_strict_message_field_with_cached_size(
                    11u32,
                    dummy_ident,
                    os,
                )?;
            }
        }
        match &self.one_of_enum {
            &compound::OneOfSomething::double(dummy_ident) => {
                os.write_double(12u32, dummy_ident)?;
            }
            &compound::OneOfSomething::_enum(dummy_ident) => {
                os.write_enum(13u32, ::protobuf::ProtobufEnumStrict::value(dummy_ident))?;
            }
            &compound::OneOfSomething::message(ref dummy_ident) => {
                ::protobuf::rt::write_strict_message_field_with_cached_size(
                    14u32,
                    dummy_ident,
                    os,
                )?;
            }
        }
        match &self.one_of_message {
            &compound::OneOfSomething::double(dummy_ident) => {
                os.write_double(15u32, dummy_ident)?;
            }
            &compound::OneOfSomething::_enum(dummy_ident) => {
                os.write_enum(16u32, ::protobuf::ProtobufEnumStrict::value(dummy_ident))?;
            }
            &compound::OneOfSomething::message(ref dummy_ident) => {
                ::protobuf::rt::write_strict_message_field_with_cached_size(
                    17u32,
                    dummy_ident,
                    os,
                )?;
            }
        }
        if !self.bytes_default.is_empty() {
            os.write_bytes(18u32, &self.bytes_default)?;
        }
        if !self.bytes_o_default.is_empty() {
            os.write_bytes(19u32, &self.bytes_o_default)?;
        }
        if !self.bytes_o_empty.is_empty() {
            os.write_bytes(20u32, &self.bytes_o_empty)?;
        }
        ::std::result::Result::Ok(())
    }
    fn compute_size(&self) -> usize {
        let mut size = 0;
        match &self.one_of_double {
            &compound::OneOfSomething::double(dummy_ident) => {
                size += 9;
            }
            &compound::OneOfSomething::_enum(dummy_ident) => {
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
            &compound::OneOfSomething::message(ref dummy_ident) => {
                let len = dummy_ident.compute_size() as u32;
                size += 1u32 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
            }
        }
        match &self.one_of_enum {
            &compound::OneOfSomething::double(dummy_ident) => {
                size += 9;
            }
            &compound::OneOfSomething::_enum(dummy_ident) => {
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
            &compound::OneOfSomething::message(ref dummy_ident) => {
                let len = dummy_ident.compute_size() as u32;
                size += 1u32 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
            }
        }
        match &self.one_of_message {
            &compound::OneOfSomething::double(dummy_ident) => {
                size += 9;
            }
            &compound::OneOfSomething::_enum(dummy_ident) => {
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
                size += ::protobuf::rt::enum_size_strict(16u32, dummy_ident);
            }
            &compound::OneOfSomething::message(ref dummy_ident) => {
                let len = dummy_ident.compute_size() as u32;
                size += 2u32 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
            }
        }
        if !self.bytes_default.is_empty() {
            size += ::protobuf::rt::bytes_size(18u32, &self.bytes_default);
        }
        if !self.bytes_o_default.is_empty() {
            size += ::protobuf::rt::bytes_size(19u32, &self.bytes_o_default);
        }
        if !self.bytes_o_empty.is_empty() {
            size += ::protobuf::rt::bytes_size(20u32, &self.bytes_o_empty);
        }
        size as usize
    }
}