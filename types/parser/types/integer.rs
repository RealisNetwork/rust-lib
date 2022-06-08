use crate::parser::traits::Parameterizable;
use quote::{__private::TokenStream, quote};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "numberType")]
pub enum AdditionalAttribute {
    #[serde(alias = "byte")]
    Byte,
    #[serde(alias = "short")]
    Short,
    #[serde(alias = "int")]
    Int,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Integer {
    minimum: i64,
    maximum: i64,
    #[serde(rename = "additionalAttributes")]
    additional_attributes: AdditionalAttribute,
}

impl Parameterizable for Integer {
    fn get_type(&self, _: &str) -> (TokenStream, TokenStream) {
        match self.additional_attributes {
            AdditionalAttribute::Byte => (quote! {}, quote! { i8 }),
            AdditionalAttribute::Short => (quote! {}, quote! { i16 }),
            AdditionalAttribute::Int => (quote! {}, quote! { i32 }),
        }
    }

    fn impl_read_from_bytes(&self, _: &str) -> TokenStream {
        match self.additional_attributes {
            AdditionalAttribute::Byte => quote! { let params = i8::decode(_byte_reader)?; },
            AdditionalAttribute::Short => quote! { let params = i16::decode(_byte_reader)?; },
            AdditionalAttribute::Int => quote! { let params = i32::decode(_byte_reader)?; },
        }
    }

    fn impl_write_to_bytes(&self) -> TokenStream {
        quote! { object.encode(&mut byte_writer).map_err(EncodeError::Byte)?; }
        // quote! { byte_writer.write_bytes(&object.to_ne_bytes()); } // TODO:
        // Check this
        // quote! { byte_writer.
        // write_type(object);
        // }
    }
}
