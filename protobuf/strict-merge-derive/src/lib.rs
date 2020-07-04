use syn::{DeriveInput, Data, Fields, Field};
use quote::{quote, format_ident};
use proc_macro2::TokenStream;

#[proc_macro_derive(StrictMerge, attributes(t_double, t_float, t_int32, t_int64, t_uint32, t_uint64, t_sint32, t_sint64, t_fixed32, t_fixed64, t_sfixed32, t_sfixed64, t_bool, t_string, t_bytes , t_enum , t_message, t_group))]
pub fn strict_merge(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derive_input: DeriveInput = syn::parse(input).unwrap();
    let name = derive_input.ident;

    let q = match derive_input.data {
        Data::Struct(s) => {
            let named_fields: Vec<Field> = if let Fields::Named(n) = s.fields {
                n.named.into_iter().collect()
            } else {
                unreachable!("All fields should be named");
            };

            let names = named_fields
                .iter()
                .map(|f| f.ident.clone().unwrap())
                .collect::<Vec<_>>();
            let names_opt = named_fields
                .iter()
                .map(|f| f.ident.clone().unwrap())
                .map(|f| f.to_string() + "_opt")
                .map(|f| format_ident!("{}", f))
                .collect::<Vec<_>>();
            let (names_match, default_values): (Vec<_>, Vec<Option<TokenStream>>) = named_fields
                .iter()
                .enumerate()
                .map(|(index, field)| {
                    let index = index as u32 + 1;
                    let p = || panic!("{:#?}", field);
                    let field_opt = format_ident!("{}", field.ident.clone().unwrap().to_string() + "_opt");

                    let attr = field.attrs.last().unwrap().path.segments.first().unwrap().ident.to_string().replace("t_", "");

                    let (deserialize, default_value) = match attr.as_str() {
                        "sfixed64" => (quote! {
                            if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                                return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                            }
                            #field_opt = Some(is.read_double()?);
                        }, Some(quote! {
                            0 as f64
                        })),
                        _ => unreachable!("Unexpected attributed found: {}", attr)
                    };

                    (quote! {
                        #index => {
                            debug_assert_eq!(None, #field_opt);
                            #deserialize
                        }
                    }, default_value)
                })
                .unzip();
            let default_values = default_values
                .into_iter()
                .map(|f| match f {
                    None => quote! {
                        None
                    },
                    Some(s) => quote! {
                        Some(#s)
                    }
                })
                .collect::<Vec<_>>();

            let mut v = vec![];

            v.push(quote! {
                #(let mut #names_opt = #default_values;)*
            });
            v.push(quote! {
                while !is.eof()? {
                    let (field_number, wire_type) = is.read_tag_unpack()?;
                    match field_number {
                        #(#names_match),*
                        _ => {
                            debug_assert!(false, "number: {:#?}, wire_type: {:#?}", field_number, wire_type);
                            return ::protobuf::ProtobufResult::Err(::protobuf::ProtobufError::WireError(::protobuf::error::WireError::IncorrectVarint));
                        }
                    }
                }
            });

            // TODO: Support actual optional messages
            v.push(quote! {
               #(if #names_opt.is_none() {
                    debug_assert!(false, "Unexpected not filled variant for property {}", stringify!(#names_opt));
                    return ::protobuf::ProtobufResult::Err(::protobuf::ProtobufError::WireError(::protobuf::error::WireError::IncorrectVarint));
               })*
            });
            v.push(quote! {
                let gen_struct = #name {
                    #(#names: #names_opt.unwrap()),*
                };
            });
            v.push(quote! {
                ::std::result::Result::Ok(gen_struct)
            });

            v
        },
        Data::Enum(e) => {
            panic!();
        },
        Data::Union(_) => unreachable!("Unions in protobuf are not possible"),
    };

    let tokens = quote! {
        impl protobuf::StrictMerge<#name> for #name {
            fn strict_merge(is: &mut protobuf::CodedInputStream<'_>) -> protobuf::ProtobufResult<Self> {
                #(#q)*
            }
        }
    };

    tokens.into()
}