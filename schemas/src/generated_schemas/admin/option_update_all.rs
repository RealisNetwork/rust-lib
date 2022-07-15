// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct AdminOptionUpdateAllParamsClientKeysParamsParamsExtraDetailsParams {
    #[serde(rename = "type")]
    r#type: String,
    #[serde(rename = "tab")]
    pub tab: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AdminOptionUpdateAllParamsClientKeysParamsParams {
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "value")]
    pub value: String,
    #[serde(rename = "key")]
    pub key: String,
    #[serde(rename = "extraDetails")]
    pub extra_details: AdminOptionUpdateAllParamsClientKeysParamsParamsExtraDetailsParams,
    #[serde(rename = "scope")]
    pub scope: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AdminOptionUpdateAllParams {
    #[serde(rename = "clientKeys")]
    pub client_keys: Vec<AdminOptionUpdateAllParamsClientKeysParamsParams>,
}
pub type AdminOptionUpdateAllReturns = bool;
