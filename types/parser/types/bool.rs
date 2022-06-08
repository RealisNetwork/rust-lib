use crate::parser::traits::Parameterizable;
use quote::{__private::TokenStream, quote};

pub struct Bool;

impl Parameterizable for Bool {
    fn get_type(&self, _: &str) -> (TokenStream, TokenStream) {
        (quote! {}, quote! { bool })
    }

    fn impl_read_from_bytes(&self, _: &str) -> TokenStream {
        quote! { let params = bool::decode(_byte_reader)?; }
    }

    fn impl_write_to_bytes(&self) -> TokenStream {
        quote! { object.encode(&mut byte_writer).map_err(EncodeError::Byte)?; }
    }
}
