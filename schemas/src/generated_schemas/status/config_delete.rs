// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::Schema;
use serde::de::Deserializer;
use serde::{Deserialize, Serialize};
use serde_json::Value;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusConfigDeleteParams {
    #[serde(rename = "id")]
    pub id: i64,
}
impl Schema for StatusConfigDeleteParams {
    fn schema() -> Value {
        todo!()
    }
}
pub type StatusConfigDeleteReturns = bool;
