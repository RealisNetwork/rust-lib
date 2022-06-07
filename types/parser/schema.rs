use crate::parser::parameter::Parameter;

use convert_case::{Case, Casing};
use quote::{__private::TokenStream, quote, ToTokens};
use serde::{de, Deserialize, Deserializer, Serialize};
use syn::{Ident, __private::Span};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Schema {
    pub agent: String,
    pub method: String,

    #[serde(deserialize_with = "deserialize_u8")]
    pub byte_agent: u8,
    #[serde(deserialize_with = "deserialize_u8")]
    pub byte_method: u8,

    pub params: Parameter,
    pub returns: Parameter,
}

impl Schema {
    #[must_use]
    pub fn create_name(&self) -> String {
        format!("{}_{}", self.agent, self.method).to_case(Case::Pascal)
    }

    #[must_use]
    pub fn create_ident(&self) -> Ident {
        let name = self.create_name();
        Ident::new(&name, Span::call_site())
    }

    fn get_declaration(&self) -> TokenStream {
        let name = self.create_ident();

        let (params_prefix, params_declaration) = self.get_params_declaration();
        let (returns_prefix, returns_declaration) = self.get_returns_declaration();

        // TODO: Check this

        quote! {
            #params_prefix
            #returns_prefix

            #[derive(Debug, Serialize, Deserialize)]
            #[serde(untagged)]
            pub enum #name {
                #params_declaration,
                #returns_declaration
            }
        }
    }

    fn get_params_declaration(&self) -> (TokenStream, TokenStream) {
        match self.params {
            Parameter::Object(_) | Parameter::String(_) | Parameter::Integer(_) | Parameter::Bool | Parameter::Array(_) => {
                let (prefix, variant_type) = self.params.get_type(&format!("{}ParamsObject", self.create_name()));
                let declaration = quote! { Params(#variant_type) };
                (prefix, declaration)
            }
            Parameter::Empty => {
                let prefix = quote! {};
                let declaration = quote! { Params };
                (prefix, declaration)
            }
        }
    }

    fn get_returns_declaration(&self) -> (TokenStream, TokenStream) {
        match self.returns {
            Parameter::Object(_) | Parameter::String(_) | Parameter::Integer(_) | Parameter::Bool | Parameter::Array(_) => {
                let (prefix, variant_type) = self.returns.get_type(&format!("{}ReturnsObject", self.create_name()));
                let declaration = quote! { Returns(#variant_type), };
                (prefix, declaration)
            }
            Parameter::Empty => {
                let prefix = quote! {};
                let declaration = quote! { Returns };
                (prefix, declaration)
            }
        }
    }

    fn get_impl_from_bytes_to_json(&self) -> TokenStream {
        let from_bytes_to_params = self.params.impl_read_from_bytes(&format!("{}ParamsObject", self.create_name()));
        quote! {
            #from_bytes_to_params
            Ok(serde_json::to_value(params).unwrap())
        }
    }

    fn get_impl_from_value_to_bytes(&self) -> TokenStream {
        // TODO: Check this
        let name = self.create_ident();

        let from_returns_to_bytes = self.returns.impl_write_to_bytes();

        match self.returns {
            Parameter::Empty => {
                quote! {
                    Ok(vec![])
                }
            }
            _ => {
                quote! {
                    let object = serde_json::from_value::<#name>(json).map_err(EncodeError::Parse)?;
                    if let #name::Returns(object) = object {
                        let mut byte_writer = byte_formatter::ByteWriter::default();
                        #from_returns_to_bytes
                        Ok(byte_writer.as_vec())
                    } else {
                        Ok(vec![])
                    }
                }
            }
        }
    }

    fn get_impl_convertable(&self) -> TokenStream {
        let name = self.create_ident();

        let topic_to_send = format!("{}_{}", self.agent, self.method);
        let topic_to_response = format!("{}_response", topic_to_send);

        let from_bytes_to_params = self.get_impl_from_bytes_to_json();
        let from_value_to_bytes = self.get_impl_from_value_to_bytes();

        quote! {
            // impl Convertable for #name {
            //     fn from_bytes_to_json(_byte_reader: &mut ByteReader) -> Result<Value, byte_formatter::Error> {
            //         #from_bytes_to_params
            //     }
            //
            //     fn topic_to_send() -> String {
            //         String::from(#topic_to_send)
            //     }
            //
            //     fn topic_to_response() -> String {
            //         String::from(#topic_to_response)
            //     }
            //
            //     fn from_value_to_bytes(json: Value) -> Result<Vec<u8>, EncodeError> {
            //         #from_value_to_bytes
            //     }
            // }
        }
    }
}

impl ToTokens for Schema {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let declaration = self.get_declaration();
        let implementation = self.get_impl_convertable();

        tokens.extend(declaration);
        tokens.extend(implementation);
    }
}

fn deserialize_u8<'de, D>(deserializer: D) -> Result<u8, D::Error>
where
    D: Deserializer<'de>,
{
    String::deserialize(deserializer)?
        .parse::<u8>()
        .map_err(|error| de::Error::custom(format!("Cannot deserialize byte_agent or byte_method with error: {:?}", error)))
}
