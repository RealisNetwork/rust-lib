use crate::agent_params::AgentParams;
use convert_case::{Case, Casing};
use quote::__private::{Ident, TokenStream};
use quote::{quote, ToTokens};
use serde::Deserialize;
use syn::__private::Span;

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Agent {
    pub agent: String,
    pub method: String,
    pub topic: String,
    pub params: AgentParams,
    pub returns: AgentParams,
}

impl Agent {
    pub fn create_name(&self) -> (String, String) {
        (self.agent.clone(), self.method.clone())
    }

    pub fn create_file_name(&self) -> String {
        format!("{}", self.method).to_case(Case::Snake)
    }

    pub fn create_directory_name(&self) -> String {
        format!("{}", self.agent).to_case(Case::Snake)
    }

    pub fn create_name_case_pascal(&self) -> String {
        format!("{}_{}", self.agent, self.method).to_case(Case::Pascal)
    }

    #[must_use]
    pub fn create_ident(&self) -> Ident {
        let name = self.create_name_case_pascal();
        Ident::new(&name, Span::call_site())
    }

    #[must_use]
    pub fn create_name_params(&self) -> String {
        format!("{}Params", self.create_name_case_pascal())
    }

    #[must_use]
    pub fn create_name_returns(&self) -> String {
        format!("{}Returns", self.create_name_case_pascal())
    }
}

impl ToTokens for Agent {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name_params = &self.create_name_params();
        let name_returns = &self.create_name_returns();

        let params_declaration = self.params.get_schema_declaration(name_params);
        let returns_declaration = self.returns.get_schema_declaration(name_returns);

        let ident_name_params = Ident::new(name_params, Span::call_site());
        let ident_name_returns = Ident::new(name_returns, Span::call_site());

        let params_schema = serde_json::to_string(&self.params).unwrap();
        let returns_schema = serde_json::to_string(&self.returns).unwrap();

        let impl_schema_params = quote! {
            impl Schema for #ident_name_params {
                fn schema() -> Value {
                    serde_json::json!(#params_schema)
                }
            }
        };
        let impl_schema_returns = quote! {
            impl Schema for #ident_name_returns {
                fn schema() -> Value {
                    serde_json::json!(#returns_schema)
                }
            }
        };

        let declaration = quote! {
            #![allow(unknown_lints)]
            #![allow(clippy::all)]
            use crate::generated_schemas::prelude::*;

            #params_declaration
            #impl_schema_params

            #returns_declaration
            #impl_schema_returns
        };

        tokens.extend(declaration);
    }
}
