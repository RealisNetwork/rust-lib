// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::Schema;
use serde::de::Deserializer;
use serde::{Deserialize, Serialize};
use serde_json::Value;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminOptionUpdateAllParamsClientKeysParamsParamsExtraDetailsParams {
    #[serde(rename = "tab")]
    pub tab: String,
    #[serde(rename = "type")]
    pub r#type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminOptionUpdateAllParamsClientKeysParamsParams {
    #[serde(rename = "value")]
    pub value: String,
    #[serde(rename = "key")]
    pub key: String,
    #[serde(rename = "description")]
    pub description: Option<String>,
    #[serde(rename = "extraDetails")]
    pub extra_details: Option<AdminOptionUpdateAllParamsClientKeysParamsParamsExtraDetailsParams>,
    #[serde(rename = "scope")]
    pub scope: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminOptionUpdateAllParams {
    #[serde(rename = "clientKeys")]
    pub client_keys: Vec<AdminOptionUpdateAllParamsClientKeysParamsParams>,
}
impl Schema for AdminOptionUpdateAllParams {
    fn schema() -> Value {
        todo!()
    }
}
pub type AdminOptionUpdateAllReturns = bool;
