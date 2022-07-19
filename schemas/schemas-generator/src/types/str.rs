use quote::__private::{Ident, TokenStream};
use quote::quote;
use serde::{Deserialize, Serialize};
use syn::__private::Span;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct StringParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<StringFormat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_length: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_length: Option<u64>,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub enum StringFormat {
    #[serde(rename = "email")]
    Email,
    #[serde(rename = "date")]
    Date,
}

impl StringParams {
    pub fn get_declaration(&self, name: &str) -> (TokenStream, TokenStream) {
        let ident = Ident::new(name, Span::call_site());
        (
            quote! {},
            quote! {
                #[derive(Debug, Clone, Serialize, Deserialize)]
                pub struct #ident(String);
            },
        )
    }

    pub fn get_type(&self, _name: &str) -> TokenStream {
        quote! { String }
    }

    pub fn get_schema_declaration(&self, name: &str) -> TokenStream {
        let (mut prefix, declaration) = self.get_declaration(name);
        prefix.extend(declaration);
        prefix
    }
}
