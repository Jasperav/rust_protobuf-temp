use syn::{DeriveInput, Data, Fields, Field};
use quote::{quote, format_ident};
use proc_macro2::{TokenStream, Punct, Literal};
use syn::parse::{Parse, ParseBuffer};
use syn::parse_macro_input;

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

#[derive(Debug)]
pub struct OneOfMapping {
    pub name: String,
    pub prototype: String,
    pub field_number: i32,
}

impl Parse for OneOfMapping {
    fn parse(input: &ParseBuffer) -> syn::Result<Self> {
        // Ignore the '='
        Punct::parse(&input)?;

        let info = parse_literal(input)?;
        let (name, info) = info.split_at(info.find("|").unwrap() + 1);
        let (prototype, field_number) = info.split_at(info.find("|").unwrap() + 1);

        Ok(OneOfMapping {
            name: name.replace("|", ""),
            prototype: prototype.replace("|", ""),
            field_number: field_number.parse().unwrap(),
        })
    }
}