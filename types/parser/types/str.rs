use crate::parser::traits::Parameterizable;
use quote::{__private::TokenStream, quote};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Str {
    #[serde(default)]
    pattern: Option<String>,
}

impl Parameterizable for Str {
    fn get_type(&self, _: &str) -> (TokenStream, TokenStream) {
        (quote! {}, quote! { String })
    }

    fn impl_read_from_bytes(&self, _: &str) -> TokenStream {
        quote! { let params = String::decode(_byte_reader)?; }
    }

    fn impl_write_to_bytes(&self) -> TokenStream {
        quote! { object.encode(&mut byte_writer).map_err(EncodeError::Byte)?; }
    }
}
