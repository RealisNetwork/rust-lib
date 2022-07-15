// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct AdminOptionUpdateParamsExtraDetailsParams {
    #[serde(rename = "type")]
    r#type: String,
    #[serde(rename = "tab")]
    pub tab: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AdminOptionUpdateParams {
    #[serde(rename = "clientKey")]
    pub client_key: String,
    #[serde(rename = "value")]
    pub value: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "extraDetails")]
    pub extra_details: AdminOptionUpdateParamsExtraDetailsParams,
}
pub type AdminOptionUpdateReturns = bool;
