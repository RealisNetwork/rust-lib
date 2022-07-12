use crate::traits::Parameterizable;
use crate::types::array::Array;
use crate::types::integer::{AdditionalAttribute, Integer};
use crate::types::object::Object;
use crate::types::str::StringParams;
use convert_case::{Case, Casing};
use quote::__private::{Ident, TokenStream};
use quote::{quote, ToTokens};
use serde::{Deserialize, Serialize};
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
    pub fn create_ident_params(&self) -> Ident {
        let name = format!("{}Params", self.create_name_case_pascal());
        Ident::new(&name, Span::call_site())
    }

    #[must_use]
    pub fn create_ident_returns(&self) -> Ident {
        let name = format!("{}Returns", self.create_name_case_pascal());
        Ident::new(&name, Span::call_site())
    }

    fn get_declaration(&self) -> TokenStream {
        let (params_prefix, params_declaration) = self.get_params_declaration();
        let (returns_prefix, returns_declaration) = self.get_returns_declaration();

        // TODO: Check this

        quote! {
            #params_prefix
            #returns_prefix

            #[derive(Debug, Serialize, Deserialize)]
            #params_declaration

            #[derive(Debug, Serialize, Deserialize)]
            #returns_declaration
        }
    }

    fn get_params_declaration(&self) -> (TokenStream, TokenStream) {
        let name = self.create_ident_params();
        match &self.params {
            // _ => {
            //     let declaration = quote! { #name };
            //     let prefix = quote! {};
            //     (prefix, declaration)
            // }
            AgentParams::Array(_array) => (quote! {}, quote! {pub struct #name;}), // pub struct ..Params(Vec<...>)
            AgentParams::Object(_object) => (quote! {}, quote! {pub struct #name;}), // pub struct ..Params{...}
            AgentParams::String(_string) => (quote! {}, quote! {pub struct #name(String);}), // pub struct ..Params(String)
            AgentParams::Integer(_integer) => (quote! {}, quote! {pub struct #name;}), // pub struct ..Params(u8|u16...)
            // pub struct ..Params(bool)
            AgentParams::Bool => (quote! {}, quote! { pub struct #name(bool); }),
            AgentParams::Empty => (quote! {}, quote! {pub struct #name;}), // pub struct ..Params or ..Params {}
        }
    }

    fn get_returns_declaration(&self) -> (TokenStream, TokenStream) {
        let name = self.create_ident_returns();
        match &self.returns {
            // _ => {
            //     let declaration = quote! { #name };
            //     let prefix = quote! {};
            //     (prefix, declaration)
            // }
            AgentParams::Array(_array) => (quote! {}, quote! {pub struct #name;}), // pub struct ..Returns(Vec<...>)
            AgentParams::Object(_object) => (quote! {}, quote! {pub struct #name;}), // pub struct ..Returns{...}
            AgentParams::String(_string) => (quote! {}, quote! {pub struct #name(String);}), // pub struct ..Returns(String)
            AgentParams::Integer(_integer) => (quote! {}, quote! {pub struct #name;}), // pub struct ..Returns(u8|u16...)
            // pub struct ..Returns(bool)
            AgentParams::Bool => (quote! {}, quote! { pub struct #name(bool); }),
            AgentParams::Empty => (quote! {}, quote! {pub struct #name;}), // pub struct ..Returns or ..Returns {}
        }
    }
}

impl ToTokens for Agent {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let declaration = self.get_declaration();

        tokens.extend(declaration);
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
#[serde(from = "MaybeTaggedAgentsParams")]
pub enum AgentParams {
    Array(Array),
    Object(Object),
    String(StringParams),
    Integer(Option<Integer>),
    Bool,
    Empty,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
enum MaybeTaggedAgentsParams {
    Tagged(TaggedParams),
    Empty {},
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
enum TaggedParams {
    #[serde(rename = "array")]
    Array { items: AgentParams },
    #[serde(rename = "object")]
    Object(Object),
    #[serde(rename = "string")]
    String(StringParams),
    #[serde(rename = "integer", alias = "number")]
    Integer {
        minimum: Option<i64>,
        maximum: Option<i64>,
        #[serde(rename = "additionalAttributes")]
        additional_attributes: Option<AdditionalAttribute>,
    },
    #[serde(rename = "boolean")]
    Bool,
}

impl AgentParams {
    #[must_use]
    pub fn get_type(&self, name: &str) -> (TokenStream, TokenStream) {
        match self {
            AgentParams::Array(array) => array.get_type(name),
            _ => (quote! {}, quote! {}),
            // AgentParams::Bool => Bool.get_type(name),
            // AgentParams::Integer(integer) => integer.get_type(name),
            // AgentParams::Number(number) => number.get_type(name),
            // AgentParams::String(str) => str.get_type(name),
            // AgentParams::Object(object) => object.get_type(name),
        }
    }

    #[must_use]
    pub fn impl_read_from_bytes(&self, name: &str) -> TokenStream {
        match self {
            // AgentParams::String(str) => str.impl_read_from_bytes(name),
            // AgentParams::Integer(integer) => integer.impl_read_from_bytes(name),
            // AgentParams::Number(number) => number.impl_read_from_bytes(name),
            // AgentParams::Bool => Bool.impl_read_from_bytes(name),
            AgentParams::Array(array) => array.impl_read_from_bytes(name),
            _ => quote! { let params = (); },
            // AgentParams::Object(object) => object.impl_read_from_bytes(name),
        }
    }

    #[must_use]
    pub fn impl_write_to_bytes(&self) -> TokenStream {
        match self {
            // AgentParams::String(str) => str.impl_write_to_bytes(),
            // AgentParams::Integer(integer) => integer.impl_write_to_bytes(),
            // AgentParams::Number(number) => number.impl_write_to_bytes(),
            // AgentParams::Bool => Bool.impl_write_to_bytes(),
            AgentParams::Array(array) => array.impl_write_to_bytes(),
            _ => quote! {},
            // AgentParams::Object(object) => object.impl_write_to_bytes(),
        }
    }
}

impl From<MaybeTaggedAgentsParams> for AgentParams {
    fn from(parameter: MaybeTaggedAgentsParams) -> Self {
        match parameter {
            MaybeTaggedAgentsParams::Tagged(TaggedParams::Array { items }) => {
                AgentParams::Array(Array {
                    parameter: Box::new(items),
                })
            }
            MaybeTaggedAgentsParams::Tagged(TaggedParams::Object(value)) => {
                AgentParams::Object(value)
            }
            MaybeTaggedAgentsParams::Tagged(TaggedParams::String(value)) => {
                AgentParams::String(value)
            }
            MaybeTaggedAgentsParams::Tagged(TaggedParams::Integer {
                minimum,
                maximum,
                additional_attributes,
            }) => match (minimum, maximum, additional_attributes) {
                (Some(minimum), Some(maximum), Some(additional_attributes)) => {
                    AgentParams::Integer(Some(Integer {
                        minimum,
                        maximum,
                        additional_attributes,
                    }))
                }
                _ => AgentParams::Integer(None),
            },
            MaybeTaggedAgentsParams::Tagged(TaggedParams::Bool) => AgentParams::Bool,
            MaybeTaggedAgentsParams::Empty {} => AgentParams::Empty,
        }
    }
}
