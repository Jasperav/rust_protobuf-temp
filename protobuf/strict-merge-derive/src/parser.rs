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
    pub tag_size: u32
}

fn find_next(s: &str) -> (String, String) {
    let (first, second) = s.split_at(s.find("|").unwrap() + 1);

    (first.replace("|", ""), second.to_string())
}

impl Parse for OneOfMapping {
    fn parse(input: &ParseBuffer) -> syn::Result<Self> {
        // Ignore the '='
        Punct::parse(&input)?;

        let info = parse_literal(input)?;
        let (name, info) = find_next(&info);
        let (prototype, info) = find_next(&info);
        let (field_number, tag_size) = find_next(&info);

        let proto_mapping = str_to_value_calculator(&prototype);

        Ok(OneOfMapping {
            enum_case: format_ident!("{}", name),
            proto_mapping,
            field_number: field_number.parse().unwrap(),
            tag_size: tag_size.parse().unwrap()
        })
    }
}