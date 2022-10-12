use crate::agent::Agent;
use quote::__private::TokenStream;
use quote::quote;
use std::io::Write;
use std::path::Path;

pub struct SchemaManagerGenerator {
    agents: Vec<Agent>,
    filename: String,
}

impl SchemaManagerGenerator {
    pub fn new(agents: Vec<Agent>, filename: String) -> Self {
        Self { agents, filename }
    }

    pub fn generate(self) {
        let mut file = std::fs::File::create(&Path::new(&self.filename))
            .unwrap_or_else(|_| panic!("Fail to create {:?} file", self.filename));

        let code = self.code();

        file.write_all(code.to_string().as_bytes())
            .unwrap_or_else(|_| panic!("Fail to create {:?} file", self.filename));
    }

    fn code(&self) -> TokenStream {
        let agent_params = self.agents.iter().map(|a| {
            let agent = &a.agent;
            let method = &a.method;
            let ident = a.create_ident_params();
            quote! {(#agent, #method) => Some(#ident::schema()),}
        });
        let agent_returns = self.agents.iter().map(|a| {
            let agent = &a.agent;
            let method = &a.method;
            let ident = a.create_ident_returns();
            quote! {(#agent, #method) => Some(#ident::schema()),}
        });
        let access_level = self.agents.iter().map(|a| {
            let agent = &a.agent;
            let method = &a.method;
            let ident = a.access_level;
            quote! {(#agent, #method) => #ident,}
        });

        quote! {
            use crate::generated_schemas::*;
            use crate::Schema;
            use error_registry::{generated_errors::{Common, Validation}, BaseError};
            use jsonschema::JSONSchema;
            use serde_json::{json, Value};
            use crate::common::*;

            pub struct SchemaManager;

            impl SchemaManager {
                pub fn get_params_schema(agent: &str, method: &str) -> Option<Value> {
                    match (agent, method) {
                        #(#agent_params)*
                        _ => None,
                    }
                }

                pub fn get_returns_schema(agent: &str, method: &str) -> Option<Value> {
                    match (agent, method) {
                        #(#agent_returns)*
                        _ => None,
                    }
                }

                pub fn get_access_level(agent: &str, method: &str) -> Option<AccessLevel> {
                    match (agent, method) {
                        #(#access_level)*
                        _ => None,
                    }
                }

                pub fn validate_params(
                    agent: &str,
                    method: &str,
                    json: &Value,
                ) -> Result<(), BaseError<Value>> {
                    let schema = Self::get_params_schema(agent, method)
                        .ok_or_else(|| BaseError::new("Unknown method", Common::InternalServerError))?;
                    Self::validate(&schema, json)
                }

                pub fn validate_returns(
                    agent: &str,
                    method: &str,
                    json: &Value,
                ) -> Result<(), BaseError<Value>> {
                    let schema = Self::get_returns_schema(agent, method)
                        .ok_or_else(|| BaseError::new("Unknown method", Common::InternalServerError))?;
                    Self::validate(&schema, json)
                }

                pub fn validate(schema: &Value, json: &Value) -> Result<(), BaseError<Value>> {
                    JSONSchema::compile(schema)
                        .map_err(|e| {
                            BaseError::new(
                                format!("Invalid schema: {}", e),
                                Common::InternalServerError,
                            )
                        })?
                        .validate(json)
                        .map_err(|e| BaseError {
                            msg: format!(
                                "Does not match pattern: {:?}",
                                e.map(|e| e.to_string()).collect::<Vec<_>>()
                            ),
                            error_type: Validation::DoesNotMatchPattern.into(),
                            data: Some(json!({
                                "schema": schema,
                                "json": json,
                            })),
                            ..Default::default()
                        })
                }

                pub fn is_valid_params(agent: &str, method: &str, json: &Value) -> Result<bool, BaseError<()>> {
                    let schema = Self::get_params_schema(agent, method)
                        .ok_or_else(|| BaseError::new("Unknown method", Common::InternalServerError))?;
                    Self::is_valid(&schema, json)
                }

                pub fn is_valid_returns(
                    agent: &str,
                    method: &str,
                    json: &Value,
                ) -> Result<bool, BaseError<()>> {
                    let schema = Self::get_returns_schema(agent, method)
                        .ok_or_else(|| BaseError::new("Unknown method", Common::InternalServerError))?;
                    Self::is_valid(&schema, json)
                }

                pub fn is_valid(schema: &Value, json: &Value) -> Result<bool, BaseError<()>> {
                    JSONSchema::compile(schema)
                        .map(|schema| schema.is_valid(json))
                        .map_err(|e| {
                            BaseError::new(
                                format!("Invalid schema: {}", e),
                                Common::InternalServerError,
                            )
                        })
                }
            }
        }
    }
}
