use crate::agent_params::AgentParams;
use crate::schema_declaration::SchemaDeclaration;
use convert_case::{Case, Casing};
use quote::__private::{Ident, TokenStream};
use quote::quote;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use syn::__private::Span;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Object {
    properties: HashMap<String, AgentParams>,
    required: Option<Vec<String>>,
}

impl Object {
    fn processed_field_name(
        &self,
        name: &str,
        field_name: &str,
        field_type: AgentParams,
    ) -> (TokenStream, TokenStream) {
        let field_type_stream = self.get_field_type(name, field_name, &field_type);
        let field_declaration = match field_name.to_lowercase().as_str() {
            "type" => {
                let ident = Ident::new_raw(&field_name.to_lowercase(), Span::call_site());
                quote! {
                    #[serde(rename = #field_name)]
                    pub #ident: #field_type_stream
                }
            }
            _ => {
                let name_ident = Ident::new(&field_name.to_case(Case::Snake), Span::call_site());
                quote! {
                    #[serde(rename = #field_name)]
                    pub #name_ident: #field_type_stream
                }
            }
        };

        let prefix = match field_type {
            AgentParams::Array(array) => {
                let (prefix, _declaration) = array.get_declaration(name);
                prefix
            }
            AgentParams::Object(object) => {
                let (prefix, declaration) = object.get_declaration(name);
                quote! {
                    #prefix
                    #declaration
                }
            }
            _ => quote! {},
        };

        (prefix, field_declaration)
    }

    fn get_fields(&self, name: &str) -> Vec<(TokenStream, TokenStream)> {
        self.properties
            .clone()
            .into_iter()
            .map(|(native_field_name, field_type)| {
                self.processed_field_name(
                    &format!("{}_{}Params", name, native_field_name).to_case(Case::Pascal),
                    &native_field_name,
                    field_type,
                )
            })
            .collect()
    }

    pub fn get_declaration(&self, name: &str) -> (TokenStream, TokenStream) {
        let (prefix, fields): (Vec<_>, Vec<_>) = self.get_fields(name).into_iter().unzip();
        let ident = Ident::new(name, Span::call_site());
        (
            quote! {#(#prefix)*},
            quote! {
                #[derive(Debug, Clone, Serialize, Deserialize)]
                pub struct #ident { #(#fields),*}
            },
        )
    }

    pub fn get_type(&self, name: &str) -> TokenStream {
        let ident = Ident::new(name, Span::call_site());
        quote! { #ident }
    }

    pub fn get_schema_declaration(&self, name: &str) -> SchemaDeclaration {
        let (prefix, declaration) = self.get_declaration(name);
        SchemaDeclaration {
            declaration,
            prefix,
            contains_struct: true,
        }
    }

    fn get_field_type(
        &self,
        name: &str,
        field_name: &str,
        field_type: &AgentParams,
    ) -> TokenStream {
        match self
            .required
            .as_ref()
            .map(|fields| fields.contains(&field_name.to_owned()))
        {
            Some(true) | None => field_type.get_type(name),
            Some(false) => {
                let field_type = field_type.get_type(name);
                quote! {
                    Option<#field_type>
                }
            }
        }
    }
}
