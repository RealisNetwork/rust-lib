use quote::{quote, ToTokens, __private::TokenStream};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "numberType")]
pub enum AdditionalAttribute {
    #[serde(alias = "byte")]
    Byte,
    #[serde(alias = "short")]
    Short,
    #[serde(alias = "int")]
    Int,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Integer {
    pub minimum: i64,
    pub maximum: i64,
    #[serde(rename = "additionalAttributes")]
    pub additional_attributes: AdditionalAttribute,
}

impl Integer {
    pub fn get_declaration(&self) -> TokenStream {
        match self.additional_attributes {
            AdditionalAttribute::Byte => quote! { i8 },
            AdditionalAttribute::Short => quote! { i16 },
            AdditionalAttribute::Int => quote! { i32 },
        }
    }
}

impl ToTokens for Integer {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let declaration = self.get_declaration();

        tokens.extend(declaration);
    }
}
