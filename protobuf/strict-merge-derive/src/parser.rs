use syn::{DeriveInput, Data, Fields, Field, Type};
use quote::{quote, format_ident};
use proc_macro2::{TokenStream, Punct, Literal, Ident};
use syn::parse::{Parse, ParseBuffer};
use syn::parse_macro_input;
use crate::matcher::str_to_value_calculator;
use crate::calculators::ValueCalculator;
use crate::calculators::message::ProtobufMessage;

fn parse_literal(input: &ParseBuffer) -> syn::Result<String> {
    Ok(Literal::parse(input)?.to_string().replace("\"", ""))
}

pub struct Prototype(pub String);

impl Parse for Prototype {
    fn parse(input: &ParseBuffer) -> syn::Result<Self> {
        // Ignore the '='
        Punct::parse(input)?;

        Ok(Prototype(parse_literal(input)?))
    }
}

pub struct FieldNumber(pub u32);

impl Parse for FieldNumber {
    fn parse(input: &ParseBuffer) -> syn::Result<Self> {
        // Ignore the '='
        Punct::parse(&input)?;

        let val = Literal::parse(input)?;

        Ok(FieldNumber(val.to_string().parse().unwrap()))
    }
}
pub struct OneOfMapping {
    pub enum_case: Ident,
    pub proto_mapping: Box<dyn ValueCalculator>,
    pub field_number: u32,
}

impl Parse for OneOfMapping {
    fn parse(input: &ParseBuffer) -> syn::Result<Self> {
        // Ignore the '='
        Punct::parse(&input)?;

        let info = parse_literal(input)?;
        let (name, info) = info.split_at(info.find("|").unwrap() + 1);
        let (prototype, field_number) = info.split_at(info.find("|").unwrap() + 1);
        let mut field_number = field_number.to_string();
        let prototype = prototype.replace("|", "");

        let proto_mapping = if prototype.as_str() == "message" {
            let (field_nr, tag_size) = field_number.split_at(field_number.find("|").unwrap() + 1);
            let tag_size: u32 = tag_size.parse().unwrap();

            field_number = field_nr.replace("|", "");

            Box::new(ProtobufMessage { tag_size })
        } else {
            str_to_value_calculator(&prototype)
        };

        Ok(OneOfMapping {
            enum_case: format_ident!("{}", name.replace("|", "")),
            proto_mapping,
            field_number: field_number.parse().unwrap(),
        })
    }
}