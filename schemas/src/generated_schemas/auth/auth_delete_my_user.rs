// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::Schema;
use serde::de::Deserializer;
use serde::{Deserialize, Serialize};
use serde_json::Value;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthAuthDeleteMyUserParams {
    #[serde(rename = "token")]
    pub token: String,
}
impl Schema for AuthAuthDeleteMyUserParams {
    fn schema() -> Value {
        todo!()
    }
}
pub type AuthAuthDeleteMyUserReturns = bool;
