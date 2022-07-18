// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::Schema;
use serde::de::Deserializer;
use serde::{Deserialize, Serialize};
use serde_json::Value;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminActionUndoParams {
    #[serde(rename = "actionId")]
    pub action_id: String,
}
impl Schema for AdminActionUndoParams {
    fn schema() -> Value {
        todo!()
    }
}
pub type AdminActionUndoReturns = bool;
