use crate::parser::parameter::Parameter;
use convert_case::{Case, Casing};
use quote::{__private::TokenStream, quote};
use std::collections::HashMap;

use crate::parser::traits::Parameterizable;
use serde::{Deserialize, Serialize};
use syn::{Ident, __private::Span};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Object {
    properties: HashMap<String, Parameter>,
    required: Vec<String>,
}

impl Object {
    fn preprocess_field_name(field_name: &str) -> String {
        match field_name {
            "type" => String::from("rust_keyword_type").to_case(Case::Snake),
            _ => field_name.to_case(Case::Snake),
        }
    }

    /// Returns for each field in object (`native_name`, name, parameter)
    /// `native_name` - original field name
    /// name - original field name cast to snake case
    /// parameter - `Parameter` of a field
    fn get_fields(&self) -> Vec<(String, String, Parameter)> {
        self.properties
            .iter()
            .map(|(native_field_name, field_type)| {
                (
                    native_field_name.clone(),
                    Self::preprocess_field_name(native_field_name),
                    field_type.clone(),
                )
            })
            .collect()
    }
}

impl Parameterizable for Object {
    fn get_type(&self, name: &str) -> (TokenStream, TokenStream) {
        let (prefix, fields): (Vec<TokenStream>, Vec<TokenStream>) = self
            .get_fields()
            .into_iter()
            .enumerate()
            .map(|(index, (native_field_name, field_name, field_type))| {
                let (prefix, field_type) = field_type.get_type(&format!("{}{}", name, index));
                let field_name = Ident::new(&field_name, Span::call_site());

                let field_type = if self.required.contains(&native_field_name) {
                    field_type
                } else {
                    quote! { Option<#field_type> }
                };

                let field = quote! {
                    #[serde(rename=#native_field_name)]
                    pub #field_name: #field_type,
                };

                (prefix, field)
            })
            .unzip();

        let name = Ident::new(name, Span::call_site());
        let declaration = quote! {
            #(#prefix)*
            #[derive(Debug, Clone, Serialize, Deserialize)] //  ByteSerialize, ByteDeserialize
            pub struct #name {
                #(#fields)*
            }
        };

        (quote! { #declaration }, quote! { #name })
    }

    fn impl_read_from_bytes(&self, name: &str) -> TokenStream {
        let name = Ident::new(name, Span::call_site());

        quote! {
            let params = <#name>::decode(_byte_reader)?;
        }
    }

    fn impl_write_to_bytes(&self) -> TokenStream {
        quote! {
            object.encode(&mut byte_writer).map_err(EncodeError::Byte)?;
        }
    }
}
