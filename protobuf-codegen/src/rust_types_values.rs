use std::cmp;

use crate::customize::Customize;
use crate::file_and_mod::FileAndMod;
use crate::inside::protobuf_crate_path;
use crate::protobuf_name::ProtobufAbsolutePath;
use crate::rust_name::RustIdent;
use crate::rust_name::RustIdentWithPath;
use crate::rust_name::RustPath;
use crate::scope::RootScope;
use crate::scope::WithScope;
use crate::strx::capitalize;
use crate::well_known_types::is_well_known_type_full;
use protobuf::descriptor::*;

// Represent subset of rust types used in generated code
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum RustType {
    // integer: signed?, size in bits
    Int(bool, u32),
    // param is size in bits
    Float(u32),
    Bool,
    Vec(Box<RustType>),
    HashMap(Box<RustType>, Box<RustType>),
    String,
    // [T], not &[T]
    Slice(Box<RustType>),
    // str, not &str
    Str,
    Option(Box<RustType>),
    SingularField(Box<RustType>),
    SingularPtrField(Box<RustType>),
    RepeatedField(Box<RustType>),
    // Box<T>
    Uniq(Box<RustType>),
    // &T
    Ref(Box<RustType>),
    // protobuf message
    Message(RustIdentWithPath),
    // protobuf enum, not any enum
    Enum(RustIdentWithPath, RustIdent),
    // protobuf enum or unknown
    EnumOrUnknown(RustIdentWithPath, RustIdent),
    // oneof enum
    Oneof(RustIdentWithPath),
    // bytes::Bytes
    Bytes,
    // chars::Chars
    Chars,
    // group
    Group,
}

impl RustType {
    #[inline]
    pub(crate) fn to_code(&self, customize: &Customize) -> String {
        match *self {
            RustType::Int(true, bits) => format!("i{}", bits),
            RustType::Int(false, bits) => format!("u{}", bits),
            RustType::Float(bits) => format!("f{}", bits),
            RustType::Bool => format!("bool"),
            RustType::Vec(ref param) => format!("::std::vec::Vec<{}>", param.to_code(customize)),
            RustType::HashMap(ref key, ref value) => format!(
                "::std::collections::HashMap<{}, {}>",
                key.to_code(customize),
                value.to_code(customize)
            ),
            RustType::String => format!("::std::string::String"),
            RustType::Slice(ref param) => format!("[{}]", param.to_code(customize)),
            RustType::Str => format!("str"),
            RustType::Option(ref param) => {
                format!("::std::option::Option<{}>", param.to_code(customize))
            }
            RustType::SingularField(ref param) => format!(
                "{}::SingularField<{}>",
                protobuf_crate_path(customize),
                param.to_code(customize)
            ),
            RustType::SingularPtrField(ref param) => format!(
                "{}::SingularPtrField<{}>",
                protobuf_crate_path(customize),
                param.to_code(customize)
            ),
            RustType::RepeatedField(ref param) => format!(
                "{}::RepeatedField<{}>",
                protobuf_crate_path(customize),
                param.to_code(customize)
            ),
            RustType::Uniq(ref param) => format!("::std::boxed::Box<{}>", param.to_code(customize)),
            RustType::Ref(ref param) => format!("&{}", param.to_code(customize)),
            RustType::Message(ref name)
            | RustType::Enum(ref name, _)
            | RustType::Oneof(ref name) => format!("{}", name),
            RustType::EnumOrUnknown(ref name, _) => {
                if let Some(s) = customize.strict_enums {
                    if s {
                        return format!(
                            "{}",
                            name
                        )
                    }
                }
                format!(
                    "{}::ProtobufEnumOrUnknown<{}>",
                    protobuf_crate_path(customize),
                    name
                )
            },
            RustType::Group => format!("<group>"),
            RustType::Bytes => format!("::bytes::Bytes"),
            RustType::Chars => format!("{}::Chars", protobuf_crate_path(customize)),
        }
    }
}

impl RustType {
    pub fn u8() -> RustType {
        RustType::Int(false, 8)
    }

    /// Type is rust primitive?
    pub fn is_primitive(&self) -> bool {
        match *self {
            RustType::Int(..) | RustType::Float(..) | RustType::Bool => true,
            _ => false,
        }
    }

    pub fn is_u8(&self) -> bool {
        match *self {
            RustType::Int(false, 8) => true,
            _ => false,
        }
    }

    pub fn is_copy(&self) -> bool {
        if self.is_primitive() {
            true
        } else if let RustType::Enum(..) = *self {
            true
        } else if let RustType::EnumOrUnknown(..) = *self {
            true
        } else {
            false
        }
    }

    fn is_str(&self) -> bool {
        match *self {
            RustType::Str => true,
            _ => false,
        }
    }

    fn is_string(&self) -> bool {
        match *self {
            RustType::String => true,
            _ => false,
        }
    }

    fn is_slice(&self) -> Option<&RustType> {
        match *self {
            RustType::Slice(ref v) => Some(&**v),
            _ => None,
        }
    }

    fn is_slice_u8(&self) -> bool {
        match self.is_slice() {
            Some(t) => t.is_u8(),
            None => false,
        }
    }

    fn is_message(&self) -> bool {
        match *self {
            RustType::Message(..) => true,
            _ => false,
        }
    }

    fn is_enum(&self) -> bool {
        match *self {
            RustType::Enum(..) => true,
            _ => false,
        }
    }

    fn is_enum_or_unknown(&self) -> bool {
        match *self {
            RustType::EnumOrUnknown(..) => true,
            _ => false,
        }
    }

    pub fn is_ref(&self) -> Option<&RustType> {
        match *self {
            RustType::Ref(ref v) => Some(&**v),
            _ => None,
        }
    }

    pub fn is_box(&self) -> Option<&RustType> {
        match *self {
            RustType::Uniq(ref v) => Some(&**v),
            _ => None,
        }
    }

    // default value for type
    pub fn default_value(&self, customize: &Customize) -> String {
        match *self {
            RustType::Ref(ref t) if t.is_str() => "\"\"".to_string(),
            RustType::Ref(ref t) if t.is_slice().is_some() => "&[]".to_string(),
            RustType::Int(..) => "0".to_string(),
            RustType::Float(..) => "0.".to_string(),
            RustType::Bool => "false".to_string(),
            RustType::Vec(..) => "::std::vec::Vec::new()".to_string(),
            RustType::HashMap(..) => "::std::collections::HashMap::new()".to_string(),
            RustType::String => "::std::string::String::new()".to_string(),
            RustType::Bytes => "::bytes::Bytes::new()".to_string(),
            RustType::Chars => format!("{}::Chars::new()", protobuf_crate_path(customize)),
            RustType::Option(..) => "::std::option::Option::None".to_string(),
            RustType::SingularField(..) => {
                format!("{}::SingularField::none()", protobuf_crate_path(customize))
            }
            RustType::SingularPtrField(..) => format!(
                "{}::SingularPtrField::none()",
                protobuf_crate_path(customize)
            ),
            RustType::RepeatedField(..) => {
                format!("{}::RepeatedField::new()", protobuf_crate_path(customize))
            }
            RustType::Message(ref name) => format!("{}::new()", name),
            RustType::Ref(ref m) if m.is_message() => match **m {
                RustType::Message(ref name) => format!(
                    "<{} as {}::Message>::default_instance()",
                    name,
                    protobuf_crate_path(customize)
                ),
                _ => unreachable!(),
            },
            // Note: default value of enum type may not be equal to default value of field
            RustType::Enum(ref name, ref default) => format!("{}::{}", name, default),
            RustType::EnumOrUnknown(ref name, ref default) => {
                if let Some(s) = customize.strict_enums {
                    if s {
                        return format!(
                            "{}::{}",
                            name,
                            default
                        )
                    }
                }
                format!(
                    "{}::ProtobufEnumOrUnknown::new({}::{})",
                    protobuf_crate_path(customize),
                    name,
                    default
                )
            },
            _ => panic!("cannot create default value for: {:?}", self),
        }
    }

    pub fn default_value_typed(self, customize: &Customize) -> RustValueTyped {
        RustValueTyped {
            value: self.default_value(customize),
            rust_type: self,
        }
    }

    /// Emit a code to clear a variable `v`
    pub fn clear(&self, v: &str, customize: &Customize) -> String {
        match *self {
            RustType::Option(..) => format!("{} = ::std::option::Option::None", v),
            RustType::Vec(..)
            | RustType::Bytes
            | RustType::String
            | RustType::RepeatedField(..)
            | RustType::SingularField(..)
            | RustType::SingularPtrField(..)
            | RustType::HashMap(..) => format!("{}.clear()", v),
            RustType::Chars => format!(
                "{}::Clear::clear(&mut {})",
                protobuf_crate_path(customize),
                v
            ),
            RustType::Bool
            | RustType::Float(..)
            | RustType::Int(..)
            | RustType::Enum(..)
            | RustType::EnumOrUnknown(..) => format!("{} = {}", v, self.default_value(customize)),
            ref ty => panic!("cannot clear type: {:?}", ty),
        }
    }

    // expression to convert `v` of type `self` to type `target`
    pub fn into_target(&self, target: &RustType, v: &str, customize: &Customize) -> String {
        self.try_into_target(target, v, customize)
            .expect(&format!("failed to convert {:?} into {:?}", self, target))
    }

    // https://github.com/rust-lang-nursery/rustfmt/issues/3131
    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn try_into_target(&self, target: &RustType, v: &str, customize: &Customize) -> Result<String, ()> {
        {
            if let Some(t1) = self.is_ref().and_then(|t| t.is_box()) {
                if let Some(t2) = target.is_ref() {
                    if t1 == t2 {
                        return Ok(format!("&**{}", v));
                    }
                }
            }
        }

        match (self, target) {
            (x, y) if x == y => return Ok(format!("{}", v)),
            (&RustType::Ref(ref x), y) if **x == *y => return Ok(format!("*{}", v)),
            (x, &RustType::Uniq(ref y)) if *x == **y => {
                return Ok(format!("::std::boxed::Box::new({})", v))
            }
            (&RustType::Uniq(ref x), y) if **x == *y => return Ok(format!("*{}", v)),
            (&RustType::String, &RustType::Ref(ref t)) if **t == RustType::Str => {
                return Ok(format!("&{}", v))
            }
            (&RustType::Chars, &RustType::Ref(ref t)) if **t == RustType::Str => {
                return Ok(format!("&{}", v))
            }
            (&RustType::Ref(ref t1), &RustType::Ref(ref t2)) if t1.is_string() && t2.is_str() => {
                return Ok(format!("&{}", v))
            }
            (&RustType::Ref(ref t1), &RustType::String)
                if match **t1 {
                       RustType::Str => true,
                       _ => false,
                   } => return Ok(format!("{}.to_owned()", v)),
            (&RustType::Ref(ref t1), &RustType::Chars)
                if match **t1 {
                       RustType::Str => true,
                       _ => false,
                    // TODO: from_static
                   } => {
                return Ok(format!("<{}::Chars as ::std::convert::From<_>>::from({}.to_owned())",
                    protobuf_crate_path(customize), v))
            },
            (&RustType::Ref(ref t1), &RustType::Vec(ref t2))
                if match (&**t1, &**t2) {
                       (&RustType::Slice(ref x), ref y) => **x == **y,
                       _ => false,
                   } => return Ok(format!("{}.to_vec()", v)),
            (&RustType::Ref(ref t1), &RustType::Bytes)
                if t1.is_slice_u8() =>
                    return Ok(format!("<::bytes::Bytes as ::std::convert::From<_>>::from({}.to_vec())", v)),
            (&RustType::Vec(ref x), &RustType::Ref(ref t))
                if match **t {
                       RustType::Slice(ref y) => x == y,
                       _ => false,
                   } => return Ok(format!("&{}", v)),
            (&RustType::Bytes, &RustType::Ref(ref t))
                if match **t {
                       RustType::Slice(ref y) => **y == RustType::u8(),
                       _ => false,
                   } => return Ok(format!("&{}", v)),
            (&RustType::Ref(ref t1), &RustType::Ref(ref t2))
                if match (&**t1, &**t2) {
                       (&RustType::Vec(ref x), &RustType::Slice(ref y)) => x == y,
                       _ => false,
                   } => return Ok(format!("&{}", v)),
            (&RustType::Enum(..), &RustType::Int(true, 32)) => {
                return Ok(format!("{}::ProtobufEnum::value(&{})", protobuf_crate_path(customize), v))
            },
            (&RustType::EnumOrUnknown(..), &RustType::Int(true, 32)) => {
                return if customize.strict_enums.unwrap_or(false) {
                    Ok(format!("{}::ProtobufEnum::value(&{})", protobuf_crate_path(customize), v))
                } else {
                    Ok(format!("{}::ProtobufEnumOrUnknown::value(&{})", protobuf_crate_path(customize), v))
                }
            },
            (&RustType::Ref(ref t), &RustType::Int(true, 32)) if t.is_enum() => {
                return Ok(format!("{}::ProtobufEnum::value({})", protobuf_crate_path(customize), v))
            }
            (&RustType::Ref(ref t), &RustType::Int(true, 32)) if t.is_enum_or_unknown() => {
                return if customize.strict_enums.unwrap_or(false) {
                    Ok(format!("{}::ProtobufEnum::value({})", protobuf_crate_path(customize), v))
                } else {
                    Ok(format!("{}::ProtobufEnumOrUnknown::value({})", protobuf_crate_path(customize), v))
                }
            },
            (&RustType::EnumOrUnknown(ref f, ..), &RustType::Enum(ref t, ..)) if f == t => {
                // TODO: ignores default value
                return if customize.strict_enums.unwrap_or(false) {
                    Ok(format!("{}", v))
                } else {
                    Ok(format!("{}::ProtobufEnumOrUnknown::enum_value_or_default(&{})", protobuf_crate_path(customize), v))
                }
            }
            (&RustType::Enum(ref f, ..), &RustType::EnumOrUnknown(ref t, ..)) if f == t => {
                return if customize.strict_enums.unwrap_or(false) {
                    Ok(format!("{}", v))
                } else {
                    Ok(format!("{}::ProtobufEnumOrUnknown::new({})", protobuf_crate_path(customize), v))
                }
            }
            _ => (),
        };

        if let &RustType::Ref(ref s) = self {
            if let Ok(conv) = s.try_into_target(target, v, customize) {
                return Ok(conv);
            }
        }

        Err(())
    }

    /// Type to view data of this type
    pub fn ref_type(&self) -> RustType {
        RustType::Ref(Box::new(match self {
            &RustType::String | &RustType::Chars => RustType::Str,
            &RustType::Vec(ref p) | &RustType::RepeatedField(ref p) => RustType::Slice(p.clone()),
            &RustType::Bytes => RustType::Slice(Box::new(RustType::u8())),
            &RustType::Message(ref p) => RustType::Message(p.clone()),
            &RustType::Uniq(ref p) => RustType::Uniq(p.clone()),
            x => panic!("no ref type for {:?}", x),
        }))
    }

    pub fn elem_type(&self) -> RustType {
        match self {
            &RustType::Option(ref ty) => (**ty).clone(),
            &RustType::SingularField(ref ty) => (**ty).clone(),
            &RustType::SingularPtrField(ref ty) => (**ty).clone(),
            x => panic!("cannot get elem type of {:?}", x),
        }
    }

    // type of `v` in `for v in xxx`
    pub fn iter_elem_type(&self) -> RustType {
        match self {
            &RustType::Vec(ref ty)
            | &RustType::Option(ref ty)
            | &RustType::RepeatedField(ref ty)
            | &RustType::SingularField(ref ty)
            | &RustType::SingularPtrField(ref ty) => RustType::Ref(ty.clone()),
            x => panic!("cannot iterate {:?}", x),
        }
    }

    pub fn value(self, value: String) -> RustValueTyped {
        RustValueTyped {
            value: value,
            rust_type: self,
        }
    }
}

/// Representation of an expression in code generator: text and type
pub(crate) struct RustValueTyped {
    pub value: String,
    pub rust_type: RustType,
}

impl RustValueTyped {
    pub fn into_type(&self, target: RustType, customize: &Customize) -> RustValueTyped {
        let target_value = self.rust_type.into_target(&target, &self.value, customize);
        RustValueTyped {
            value: target_value,
            rust_type: target,
        }
    }

    pub fn boxed(self, customize: &Customize) -> RustValueTyped {
        self.into_type(RustType::Uniq(Box::new(self.rust_type.clone())), customize)
    }
}

// protobuf type name for protobuf base type
pub fn protobuf_name(field_type: field_descriptor_proto::Type) -> &'static str {
    use field_descriptor_proto::Type;
    match field_type {
        Type::TYPE_DOUBLE => "double",
        Type::TYPE_FLOAT => "float",
        Type::TYPE_INT32 => "int32",
        Type::TYPE_INT64 => "int64",
        Type::TYPE_UINT32 => "uint32",
        Type::TYPE_UINT64 => "uint64",
        Type::TYPE_SINT32 => "sint32",
        Type::TYPE_SINT64 => "sint64",
        Type::TYPE_FIXED32 => "fixed32",
        Type::TYPE_FIXED64 => "fixed64",
        Type::TYPE_SFIXED32 => "sfixed32",
        Type::TYPE_SFIXED64 => "sfixed64",
        Type::TYPE_BOOL => "bool",
        Type::TYPE_STRING => "string",
        Type::TYPE_BYTES => "bytes",
        Type::TYPE_ENUM => "enum",
        Type::TYPE_MESSAGE => "message",
        Type::TYPE_GROUP => "group",
    }
}

// rust type for protobuf base type
pub(crate) fn rust_name(field_type: field_descriptor_proto::Type) -> RustType {
    use field_descriptor_proto::Type;
    match field_type {
        Type::TYPE_DOUBLE => RustType::Float(64),
        Type::TYPE_FLOAT => RustType::Float(32),
        Type::TYPE_INT32 => RustType::Int(true, 32),
        Type::TYPE_INT64 => RustType::Int(true, 64),
        Type::TYPE_UINT32 => RustType::Int(false, 32),
        Type::TYPE_UINT64 => RustType::Int(false, 64),
        Type::TYPE_SINT32 => RustType::Int(true, 32),
        Type::TYPE_SINT64 => RustType::Int(true, 64),
        Type::TYPE_FIXED32 => RustType::Int(false, 32),
        Type::TYPE_FIXED64 => RustType::Int(false, 64),
        Type::TYPE_SFIXED32 => RustType::Int(true, 32),
        Type::TYPE_SFIXED64 => RustType::Int(true, 64),
        Type::TYPE_BOOL => RustType::Bool,
        Type::TYPE_STRING => RustType::String,
        Type::TYPE_BYTES => RustType::Vec(Box::new(RustType::Int(false, 8))),
        Type::TYPE_ENUM | Type::TYPE_GROUP | Type::TYPE_MESSAGE => {
            panic!("there is no rust name for {:?}", field_type)
        }
    }
}

fn file_last_component(file: &str) -> &str {
    let bs = file.rfind('\\').map(|i| i + 1).unwrap_or(0);
    let fs = file.rfind('/').map(|i| i + 1).unwrap_or(0);
    &file[cmp::max(fs, bs)..]
}

#[cfg(test)]
#[test]
fn test_file_last_component() {
    assert_eq!("ab.proto", file_last_component("ab.proto"));
    assert_eq!("ab.proto", file_last_component("xx/ab.proto"));
    assert_eq!("ab.proto", file_last_component("xx\\ab.proto"));
    assert_eq!("ab.proto", file_last_component("yy\\xx\\ab.proto"));
}

fn is_descriptor_proto(file: &FileDescriptorProto) -> bool {
    file.get_package() == "google.protobuf"
        && file_last_component(file.get_name()) == "descriptor.proto"
}

fn make_path_to_path(source: &RustPath, dest: &RustPath) -> RustPath {
    if dest.is_absolute() {
        return dest.clone();
    }

    assert!(!source.is_absolute());

    let mut source = source.clone();
    let mut dest = dest.clone();
    while !source.is_empty() && source.first() == dest.first() {
        source.remove_first().unwrap();
        dest.remove_first().unwrap();
    }
    source.to_reverse().append(dest)
}

pub(crate) fn make_path(source: &RustPath, dest: &RustIdentWithPath) -> RustIdentWithPath {
    make_path_to_path(source, &dest.path).with_ident(dest.ident.clone())
}

pub(crate) fn message_or_enum_to_rust_relative(
    message_or_enum: &dyn WithScope,
    current: &FileAndMod,
) -> RustIdentWithPath {
    let same_file = message_or_enum.get_scope().get_file_descriptor().get_name() == current.file;
    if same_file {
        // field type is a message or enum declared in the same file
        make_path(
            &current.relative_mod.clone().into_path(),
            &message_or_enum.rust_name_to_file(),
        )
    } else if let Some(name) = is_well_known_type_full(&message_or_enum.name_absolute()) {
        // Well-known types are included in rust-protobuf library
        // https://developers.google.com/protocol-buffers/docs/reference/google.protobuf
        RustIdentWithPath::from(format!(
            "{}::well_known_types::{}",
            protobuf_crate_path(&current.customize),
            name
        ))
    } else if is_descriptor_proto(message_or_enum.get_file_descriptor()) {
        // Messages defined in descriptor.proto
        RustIdentWithPath::from(format!(
            "{}::descriptor::{}",
            protobuf_crate_path(&current.customize),
            message_or_enum.rust_name_to_file()
        ))
    } else {
        current
            .relative_mod
            .to_reverse()
            .into_path()
            .append_ident(RustIdent::super_ident())
            .append_with_ident(message_or_enum.rust_name_with_file())
    }
}

pub(crate) fn type_name_to_rust_relative(
    type_name: &ProtobufAbsolutePath,
    current: &FileAndMod,
    root_scope: &RootScope,
) -> RustIdentWithPath {
    assert!(!type_name.is_empty());
    let message_or_enum = root_scope.find_message_or_enum(type_name);
    message_or_enum_to_rust_relative(&message_or_enum, current)
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PrimitiveTypeVariant {
    Default,
    Carllerche,
}

pub enum _CarllercheBytesType {
    Bytes,
    Chars,
}

// ProtobufType trait name
pub(crate) enum ProtobufTypeGen {
    Primitive(field_descriptor_proto::Type, PrimitiveTypeVariant),
    Message(RustIdentWithPath),
    EnumOrUnknown(RustIdentWithPath),
    Enum(RustIdentWithPath),
}

impl ProtobufTypeGen {
    pub fn rust_type(&self, customize: &Customize) -> String {
        match self {
            &ProtobufTypeGen::Primitive(t, PrimitiveTypeVariant::Default) => format!(
                "{}::reflect::types::ProtobufType{}",
                protobuf_crate_path(customize),
                capitalize(protobuf_name(t))
            ),
            &ProtobufTypeGen::Primitive(
                field_descriptor_proto::Type::TYPE_BYTES,
                PrimitiveTypeVariant::Carllerche,
            ) => format!(
                "{}::reflect::types::ProtobufTypeCarllercheBytes",
                protobuf_crate_path(customize)
            ),
            &ProtobufTypeGen::Primitive(
                field_descriptor_proto::Type::TYPE_STRING,
                PrimitiveTypeVariant::Carllerche,
            ) => format!(
                "{}::reflect::types::ProtobufTypeCarllercheChars",
                protobuf_crate_path(customize)
            ),
            &ProtobufTypeGen::Primitive(.., PrimitiveTypeVariant::Carllerche) => unreachable!(),
            &ProtobufTypeGen::Message(ref name) => format!(
                "{}::reflect::types::ProtobufTypeMessage<{}>",
                protobuf_crate_path(customize),
                name
            ),
            &ProtobufTypeGen::EnumOrUnknown(ref name) => format!(
                "{}::reflect::types::ProtobufTypeEnumOrUnknown<{}>",
                protobuf_crate_path(customize),
                name
            ),
            &ProtobufTypeGen::Enum(ref name) => format!(
                "{}::reflect::types::ProtobufTypeEnum<{}>",
                protobuf_crate_path(customize),
                name
            ),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn into_target_ref_box_to_ref() {
        let t1 = RustType::Ref(Box::new(RustType::Uniq(Box::new(RustType::Message(
            RustIdentWithPath::from("Ab"),
        )))));
        let t2 = RustType::Ref(Box::new(RustType::Message(RustIdentWithPath::from("Ab"))));

        assert_eq!("&**v", t1.into_target(&t2, "v", &Customize::default()));
    }
}
