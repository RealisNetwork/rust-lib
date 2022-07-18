use crate::schema_declaration::SchemaDeclaration;
use crate::types::array::Array;
use crate::types::empty::Empty;
use crate::types::integer::{AdditionalAttribute, Integer};
use crate::types::object::Object;
use crate::types::str::StringParams;

use quote::__private::{Ident, TokenStream};
use quote::quote;
use serde::{Deserialize, Serialize};
use syn::__private::Span;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
#[serde(from = "MaybeTaggedAgentsParams")]
pub enum AgentParams {
    Array(Array),
    Object(Object),
    String(StringParams),
    Integer(Integer),
    Bool,
    Empty,
}

impl AgentParams {
    #[must_use]
    pub fn get_declaration(&self, name: &str) -> (TokenStream, TokenStream) {
        let ident = Ident::new(name, Span::call_site());
        match self {
            AgentParams::Array(array) => array.get_declaration(name),
            AgentParams::Object(object) => object.get_declaration(name),
            AgentParams::String(string) => string.get_declaration(name),
            AgentParams::Integer(integer) => integer.get_declaration(name),
            AgentParams::Bool => (quote! {}, quote! {pub type #ident = bool;}),
            AgentParams::Empty => Empty::get_declaration(name),
        }
    }

    pub fn get_type(&self, name: &str) -> TokenStream {
        match self {
            AgentParams::Array(array) => array.get_type(name),
            AgentParams::Object(object) => object.get_type(name),
            AgentParams::String(string) => string.get_type(name),
            AgentParams::Integer(integer) => integer.get_type(),
            AgentParams::Bool => quote! { bool },
            AgentParams::Empty => Empty::get_type(),
        }
    }

    pub fn get_schema_declaration(&self, name: &str) -> SchemaDeclaration {
        match self {
            AgentParams::Array(array) => array.get_schema_declaration(name),
            AgentParams::Object(object) => object.get_schema_declaration(name),
            AgentParams::String(string) => string.get_schema_declaration(name),
            AgentParams::Integer(integer) => integer.get_schema_declaration(name),
            AgentParams::Bool => {
                let (prefix, declaration) = self.get_declaration(name);
                SchemaDeclaration {
                    declaration,
                    prefix,
                    ..Default::default()
                }
            }
            AgentParams::Empty => Empty::get_schema_declaration(name),
        }
    }
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
                    AgentParams::Integer(Integer {
                        minimum,
                        maximum,
                        additional_attributes,
                    })
                }
                _ => AgentParams::Integer(Integer::default()),
            },
            MaybeTaggedAgentsParams::Tagged(TaggedParams::Bool) => AgentParams::Bool,
            MaybeTaggedAgentsParams::Empty {} => AgentParams::Empty,
        }
    }
}
