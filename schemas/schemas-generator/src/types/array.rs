use crate::agents::AgentParams;
use crate::traits::Parameterizable;
use quote::__private::TokenStream;
use quote::quote;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Array {
    pub parameter: Box<AgentParams>,
}

impl Parameterizable for Array {
    fn get_type(&self, name: &str) -> (TokenStream, TokenStream) {
        let (prefix, parameter_type) = self.parameter.get_type(name);
        (prefix, quote! { Vec<#parameter_type> })
    }

    fn impl_read_from_bytes(&self, name: &str) -> TokenStream {
        let (_, parameter_type) = self.parameter.get_type(name);
        quote! {
            let params = <#parameter_type>::decode(_byte_reader)?;
        }
    }

    fn impl_write_to_bytes(&self) -> TokenStream {
        quote! {
            object.encode(&mut byte_writer).map_err(EncodeError::Byte)?;
        }
    }
}
