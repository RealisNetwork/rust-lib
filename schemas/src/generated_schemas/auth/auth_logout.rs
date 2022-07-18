// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::Schema;
use serde::de::Deserializer;
use serde::{Deserialize, Serialize};
use serde_json::Value;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthAuthLogoutParams {
    #[serde(rename = "appId")]
    pub app_id: i32,
}
impl Schema for AuthAuthLogoutParams {
    fn schema() -> Value {
        todo!()
    }
}
pub type AuthAuthLogoutReturns = bool;
