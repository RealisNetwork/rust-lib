use crate::schema_declaration::SchemaDeclaration;
use quote::__private::{Ident, TokenStream};
use quote::quote;
use serde::{Deserialize, Serialize};
use syn::__private::Span;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct StringParams {
    pub format: Option<StringFormat>,
    pub pattern: Option<String>,
    pub min_length: Option<u64>,
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

    pub fn get_schema_declaration(&self, name: &str) -> SchemaDeclaration {
        let (prefix, declaration) = self.get_declaration(name);
        SchemaDeclaration {
            declaration,
            prefix,
            contains_struct: true,
        }
    }
}
