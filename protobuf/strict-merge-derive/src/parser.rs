use syn::{DeriveInput, Data, Fields, Field, Type};
use quote::{quote, format_ident};
use proc_macro2::{TokenStream, Punct, Literal, Ident};
use syn::parse::{Parse, ParseBuffer};
use syn::parse_macro_input;
use crate::value_calculator::ValueCalculator;
use crate::matcher::str_to_value_calculator;

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

pub struct OneOfMapper {
    pub mapping: Vec<OneOfMapping>,
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
        let prototype = prototype.replace("|", "");

        Ok(OneOfMapping {
            enum_case: format_ident!("{}", name.replace("|", "")),
            proto_mapping: str_to_value_calculator(&prototype),
            field_number: field_number.parse().unwrap(),
        })
    }
}