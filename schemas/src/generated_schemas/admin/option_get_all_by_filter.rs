// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct AdminOptionGetAllByFilterParams {
    #[serde(rename = "type")]
    r#type: String,
    #[serde(rename = "tab")]
    pub tab: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AdminOptionGetAllByFilterReturnsParamsExtraDetailsParams {
    #[serde(rename = "type")]
    r#type: String,
    #[serde(rename = "tab")]
    pub tab: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AdminOptionGetAllByFilterReturnsParams {
    #[serde(rename = "scope")]
    pub scope: String,
    #[serde(rename = "key")]
    pub key: String,
    #[serde(rename = "value")]
    pub value: String,
    #[serde(rename = "previousValue")]
    pub previous_value: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "extraDetails")]
    pub extra_details: AdminOptionGetAllByFilterReturnsParamsExtraDetailsParams,
}
pub type AdminOptionGetAllByFilterReturns = Vec<AdminOptionGetAllByFilterReturnsParams>;
