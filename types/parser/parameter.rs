use crate::parser::{
    traits::Parameterizable,
    types::{array::Array, bool::Bool, integer::Integer, number::Number, object::Object, str::Str},
};
use quote::{__private::TokenStream, quote};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(from = "MaybeTaggedParameter")]
pub enum Parameter {
    Array(Array),
    Object(Object),
    String(Str),
    Integer(Integer),
    Number(Number),
    Bool,
    Empty,
}

impl Parameter {
    #[must_use]
    pub fn get_type(&self, name: &str) -> (TokenStream, TokenStream) {
        match self {
            Parameter::Empty => (quote! {}, quote! {}),
            Parameter::Bool => Bool.get_type(name),
            Parameter::Integer(integer) => integer.get_type(name),
            Parameter::Number(number) => number.get_type(name),
            Parameter::String(str) => str.get_type(name),
            Parameter::Array(array) => array.get_type(name),
            Parameter::Object(object) => object.get_type(name),
        }
    }

    #[must_use]
    pub fn impl_read_from_bytes(&self, name: &str) -> TokenStream {
        match self {
            Parameter::String(str) => str.impl_read_from_bytes(name),
            Parameter::Integer(integer) => integer.impl_read_from_bytes(name),
            Parameter::Number(number) => number.impl_read_from_bytes(name),
            Parameter::Bool => Bool.impl_read_from_bytes(name),
            Parameter::Empty => quote! { let params = (); },
            Parameter::Array(array) => array.impl_read_from_bytes(name),
            Parameter::Object(object) => object.impl_read_from_bytes(name),
        }
    }

    #[must_use]
    pub fn impl_write_to_bytes(&self) -> TokenStream {
        match self {
            Parameter::String(str) => str.impl_write_to_bytes(),
            Parameter::Integer(integer) => integer.impl_write_to_bytes(),
            Parameter::Number(number) => number.impl_write_to_bytes(),
            Parameter::Bool => Bool.impl_write_to_bytes(),
            Parameter::Empty => quote! {},
            Parameter::Array(array) => array.impl_write_to_bytes(),
            Parameter::Object(object) => object.impl_write_to_bytes(),
        }
    }
}

impl From<MaybeTaggedParameter> for Parameter {
    fn from(parameter: MaybeTaggedParameter) -> Self {
        match parameter {
            MaybeTaggedParameter::Tagged(TaggedParameter::Array { items }) => Parameter::Array(Array {
                parameter: Box::new(items),
            }),
            MaybeTaggedParameter::Tagged(TaggedParameter::Object(value)) => Parameter::Object(value),
            MaybeTaggedParameter::Tagged(TaggedParameter::String(value)) => Parameter::String(value),
            MaybeTaggedParameter::Tagged(TaggedParameter::Integer(value)) => Parameter::Integer(value),
            MaybeTaggedParameter::Tagged(TaggedParameter::Number(value)) => Parameter::Number(value),
            MaybeTaggedParameter::Tagged(TaggedParameter::Bool) => Parameter::Bool,
            MaybeTaggedParameter::Empty {} => Parameter::Empty,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
enum MaybeTaggedParameter {
    Tagged(TaggedParameter),
    Empty {},
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
enum TaggedParameter {
    #[serde(alias = "array")]
    Array { items: Parameter },
    #[serde(alias = "object")]
    Object(Object),
    #[serde(alias = "string")]
    String(Str),
    #[serde(alias = "integer")]
    Integer(Integer),
    #[serde(alias = "number")]
    Number(Number),
    #[serde(alias = "boolean")]
    Bool,
}
