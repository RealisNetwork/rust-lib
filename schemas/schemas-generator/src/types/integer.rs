use quote::{
    quote, ToTokens,
    __private::{Ident, TokenStream},
};
use serde::{Deserialize, Serialize};
use syn::__private::Span;

use crate::schema_declaration::SchemaDeclaration;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "numberType")]
pub enum AdditionalAttribute {
    #[serde(alias = "byte")]
    Byte,
    #[serde(alias = "short")]
    Short,
    #[serde(alias = "int")]
    Int,
    #[serde(alias = "number")]
    Number,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Integer {
    pub minimum: i64,
    pub maximum: i64,
    #[serde(rename = "additionalAttributes")]
    pub additional_attributes: AdditionalAttribute,
}

impl Integer {
    pub fn get_declaration(&self, name: &str) -> (TokenStream, TokenStream) {
        let integer_type = self.get_type();
        let ident = Ident::new(name, Span::call_site());
        (quote! {}, quote! { pub type #ident = #integer_type; })
    }

    pub fn get_type(&self) -> TokenStream {
        match self.additional_attributes {
            AdditionalAttribute::Byte => quote! { i8 },
            AdditionalAttribute::Short => quote! { i16 },
            AdditionalAttribute::Int => quote! { i32 },
            AdditionalAttribute::Number => quote! { i64 },
        }
    }

    pub fn get_schema_declaration(&self, name: &str) -> SchemaDeclaration {
        let (prefix, declaration) = self.get_declaration(name);
        SchemaDeclaration {
            declaration,
            prefix,
            contains_struct: false,
        }
    }
}

impl Default for Integer {
    fn default() -> Self {
        Self {
            minimum: i64::MIN,
            maximum: i64::MAX,
            additional_attributes: AdditionalAttribute::Number,
        }
    }
}
