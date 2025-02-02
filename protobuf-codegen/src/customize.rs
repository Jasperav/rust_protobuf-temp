use protobuf::descriptor::FieldOptions;
use protobuf::descriptor::FileOptions;
use protobuf::descriptor::MessageOptions;
use protobuf::rustproto;
use std::collections::{HashSet, HashMap};
use crate::DeriveMap;

/// Specifies style of generated code.
/// Generated files can be customized using this proto
/// or using `rustproto.proto` options.
#[derive(Default, Debug, Clone)]
pub struct Customize {
    /// Make oneof enum public.
    pub expose_oneof: Option<bool>,
    /// When true all fields are public, and accessors are not generated
    pub expose_fields: Option<bool>,
    /// When false, `get_`, `set_`, `mut_` etc. accessors are not generated
    pub generate_accessors: Option<bool>,
    /// When false, `get_` is not generated even if `syntax = "proto2"`
    pub generate_getter: Option<bool>,
    /// Use `bytes::Bytes` for `bytes` fields
    pub carllerche_bytes_for_bytes: Option<bool>,
    /// Use `bytes::Bytes` for `string` fields
    pub carllerche_bytes_for_string: Option<bool>,
    /// Use `std::Vec<T>` to store repeated messages fields
    pub repeated_field_vec: Option<bool>,
    /// Use `std::Option<std::Box<T>>` to store singular messages fields
    pub singular_field_option_box: Option<bool>,
    /// Use `std::Option<T>` to store singular messages fields.
    /// Note, it's not possible to have recursive messages with this option enabled.
    pub singular_field_option: Option<bool>,
    /// Implement serde_derive for messages
    pub serde_derive: Option<bool>,
    /// When `serde_derive` is set, serde annotations will be guarded with `#[cfg(cfg, ...)]`.
    pub serde_derive_cfg: Option<String>,
    pub strict_enums: Option<bool>,
    pub skip_unknown_fields: Option<bool>,
    pub skip_cached_size: Option<bool>,
    pub skip_initialized_check: Option<bool>,
    pub skip_descriptor_static: Option<bool>,
    pub use_derive_debug: Option<bool>,
    /// Enable lite runtime
    pub lite_runtime: Option<bool>,
    /// Used internally to generate protos bundled in protobuf crate
    /// like `descriptor.proto`
    pub inside_protobuf: Option<bool>,
    pub derives: DeriveMap,

    // When adding more options please keep in sync with `parse_from_parameter` below.
    /// Make sure `Customize` is always used with `..Default::default()`
    /// for future compatibility.
    pub _future_options: (),
}

#[derive(Debug)]
pub enum CustomizeParseParameterError {
    EqNotFound,
    CannotParseBool,
    UnknownOptionName(String),
}

pub type CustomizeParseParameterResult<T> = Result<T, CustomizeParseParameterError>;

impl Customize {
    /// Update fields of self with fields defined in other customize
    pub fn update_with(&mut self, that: &Customize) {
        if let Some(v) = that.expose_oneof {
            self.expose_oneof = Some(v);
        }
        if let Some(v) = that.expose_fields {
            self.expose_fields = Some(v);
        }
        if let Some(v) = that.generate_accessors {
            self.generate_accessors = Some(v);
        }
        if let Some(v) = that.generate_getter {
            self.generate_getter = Some(v);
        }
        if let Some(v) = that.carllerche_bytes_for_bytes {
            self.carllerche_bytes_for_bytes = Some(v);
        }
        if let Some(v) = that.carllerche_bytes_for_string {
            self.carllerche_bytes_for_string = Some(v);
        }
        if let Some(v) = that.repeated_field_vec {
            self.repeated_field_vec = Some(v);
        }
        if let Some(v) = that.singular_field_option_box {
            self.singular_field_option_box = Some(v);
        }
        if let Some(v) = that.singular_field_option {
            self.singular_field_option = Some(v);
        }
        if let Some(v) = that.serde_derive {
            self.serde_derive = Some(v);
        }
        if let Some(ref v) = that.serde_derive_cfg {
            self.serde_derive_cfg = Some(v.clone());
        }
        if let Some(v) = that.lite_runtime {
            self.lite_runtime = Some(v);
        }
        if let Some(v) = that.inside_protobuf {
            self.inside_protobuf = Some(v);
        }
        if let Some(ref v) = that.derives {
            self.derives = Some(v.clone());
        }
    }

    /// Update unset fields of self with fields from other customize
    pub fn set_defaults_from(&mut self, other: &Customize) {
        let mut tmp = other.clone();
        tmp.update_with(self);
        *self = tmp;
    }

    /// Parse customize options from a string passed via protoc flag.
    pub fn parse_from_parameter(parameter: &str) -> CustomizeParseParameterResult<Customize> {
        fn parse_bool(v: &str) -> CustomizeParseParameterResult<bool> {
            v.parse()
                .map_err(|_| CustomizeParseParameterError::CannotParseBool)
        }

        let mut r = Customize::default();
        for nv in parameter.split_whitespace() {
            let eq = match nv.find('=') {
                Some(eq) => eq,
                None => return Err(CustomizeParseParameterError::EqNotFound),
            };

            let n = &nv[..eq];
            let v = &nv[eq + 1..];

            if n == "expose_oneof" {
                r.expose_oneof = Some(parse_bool(v)?);
            } else if n == "expose_fields" {
                r.expose_fields = Some(parse_bool(v)?);
            } else if n == "generate_accessors" {
                r.generate_accessors = Some(parse_bool(v)?);
            } else if n == "generate_getter" {
                r.generate_getter = Some(parse_bool(v)?);
            } else if n == "carllerche_bytes_for_bytes" {
                r.carllerche_bytes_for_bytes = Some(parse_bool(v)?);
            } else if n == "carllerche_bytes_for_string" {
                r.carllerche_bytes_for_string = Some(parse_bool(v)?);
            } else if n == "repeated_field_vec" {
                r.repeated_field_vec = Some(parse_bool(v)?);
            } else if n == "singular_field_option_box" {
                r.singular_field_option_box = Some(parse_bool(v)?);
            } else if n == "singular_field_option" {
                r.singular_field_option = Some(parse_bool(v)?);
            } else if n == "serde_derive" {
                r.serde_derive = Some(parse_bool(v)?);
            } else if n == "serde_derive_cfg" {
                r.serde_derive_cfg = Some(v.to_owned());
            } else if n == "lite_runtime" {
                r.lite_runtime = Some(parse_bool(v)?);
            } else if n == "inside_protobuf" {
                r.inside_protobuf = Some(parse_bool(v)?);
            } else if n == "derives" {
                // TODO: Not sure how to parse this
            } else {
                return Err(CustomizeParseParameterError::UnknownOptionName(
                    n.to_owned(),
                ));
            }
        }
        Ok(r)
    }
}

pub fn customize_from_rustproto_for_message(source: &MessageOptions) -> Customize {
    let expose_oneof = rustproto::exts::expose_oneof.get(source);
    let expose_fields = rustproto::exts::expose_fields.get(source);
    let generate_accessors = rustproto::exts::generate_accessors.get(source);
    let generate_getter = rustproto::exts::generate_getter.get(source);
    let carllerche_bytes_for_bytes = rustproto::exts::carllerche_bytes_for_bytes.get(source);
    let carllerche_bytes_for_string = rustproto::exts::carllerche_bytes_for_string.get(source);
    let repeated_field_vec = rustproto::exts::repeated_field_vec.get(source);
    let singular_field_option_box = rustproto::exts::singular_field_option_box.get(source);
    let singular_field_option = rustproto::exts::singular_field_option.get(source);
    let serde_derive = rustproto::exts::serde_derive.get(source);
    let serde_derive_cfg = rustproto::exts::serde_derive_cfg.get(source);
    let lite_runtime = None;
    let inside_protobuf = None;
    Customize {
        expose_oneof,
        expose_fields,
        generate_accessors,
        generate_getter,
        carllerche_bytes_for_bytes,
        carllerche_bytes_for_string,
        repeated_field_vec,
        singular_field_option_box,
        singular_field_option,
        serde_derive,
        serde_derive_cfg,
        strict_enums: None, // TODO
        skip_unknown_fields: None, // TODO
        skip_cached_size: None, // TODO
        skip_initialized_check: None, // TODO
        skip_descriptor_static: None, // TODO
        use_derive_debug: None, // TODO
        lite_runtime,
        inside_protobuf,
        derives: None, // TODO
        _future_options: (),
    }
}

pub fn customize_from_rustproto_for_field(source: &FieldOptions) -> Customize {
    let expose_oneof = None;
    let expose_fields = rustproto::exts::expose_fields_field.get(source);
    let generate_accessors = rustproto::exts::generate_accessors_field.get(source);
    let generate_getter = rustproto::exts::generate_getter_field.get(source);
    let carllerche_bytes_for_bytes = rustproto::exts::carllerche_bytes_for_bytes_field.get(source);
    let carllerche_bytes_for_string =
        rustproto::exts::carllerche_bytes_for_string_field.get(source);
    let repeated_field_vec = rustproto::exts::repeated_field_vec_field.get(source);
    let singular_field_option_box = rustproto::exts::singular_field_option_box_field.get(source);
    let singular_field_option = rustproto::exts::singular_field_option_field.get(source);
    let serde_derive = None;
    let serde_derive_cfg = None;
    let lite_runtime = None;
    let inside_protobuf = None;
    Customize {
        expose_oneof,
        expose_fields,
        generate_accessors,
        generate_getter,
        carllerche_bytes_for_bytes,
        carllerche_bytes_for_string,
        repeated_field_vec,
        singular_field_option_box,
        singular_field_option,
        serde_derive,
        serde_derive_cfg,
        strict_enums: None, // TODO
        skip_unknown_fields: None, // TODO
        skip_cached_size: None, // TODO
        skip_initialized_check: None, // TODO
        skip_descriptor_static: None, // TODO
        use_derive_debug: None, // TODO
        lite_runtime,
        inside_protobuf,
        derives: None, // TODO
        _future_options: (),
    }
}

pub fn customize_from_rustproto_for_file(source: &FileOptions) -> Customize {
    let expose_oneof = rustproto::exts::expose_oneof_all.get(source);
    let expose_fields = rustproto::exts::expose_fields_all.get(source);
    let generate_accessors = rustproto::exts::generate_accessors_all.get(source);
    let generate_getter = rustproto::exts::generate_getter_all.get(source);
    let carllerche_bytes_for_bytes = rustproto::exts::carllerche_bytes_for_bytes_all.get(source);
    let carllerche_bytes_for_string = rustproto::exts::carllerche_bytes_for_string_all.get(source);
    let repeated_field_vec = rustproto::exts::repeated_field_vec_all.get(source);
    let singular_field_option_box = rustproto::exts::singular_field_option_box_all.get(source);
    let singular_field_option = rustproto::exts::singular_field_option_all.get(source);
    let serde_derive = rustproto::exts::serde_derive_all.get(source);
    let serde_derive_cfg = rustproto::exts::serde_derive_cfg_all.get(source);
    let lite_runtime = rustproto::exts::lite_runtime_all.get(source);
    let inside_protobuf = None;
    Customize {
        expose_oneof,
        expose_fields,
        generate_accessors,
        generate_getter,
        carllerche_bytes_for_bytes,
        carllerche_bytes_for_string,
        repeated_field_vec,
        singular_field_option_box,
        singular_field_option,
        serde_derive,
        serde_derive_cfg,
        strict_enums: None, // TODO
        skip_unknown_fields: None, // TODO
        skip_cached_size: None, // TODO
        skip_initialized_check: None, // TODO
        skip_descriptor_static: None, // TODO
        use_derive_debug: None, // TODO
        lite_runtime,
        inside_protobuf,
        derives: None, // TODO
        _future_options: (),
    }
}
