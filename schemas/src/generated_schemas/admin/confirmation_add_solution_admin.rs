// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminConfirmationAddSolutionAdminParams {
    #[serde(rename = "isConfirmed")]
    pub is_confirmed: bool,
    #[serde(rename = "id")]
    pub id: f64,
}
impl Schema for AdminConfirmationAddSolutionAdminParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"isConfirmed\":{\"type\":\"boolean\"},\"id\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"id\",\"isConfirmed\"]}")
    }
}
impl Agent for AdminConfirmationAddSolutionAdminParams {
    fn topic() -> &'static str {
        "admin_confirmation_addSolutionAdmin"
    }
    fn method() -> &'static str {
        "confirmation_addSolutionAdmin"
    }
    fn agent() -> &'static str {
        "admin"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminConfirmationAddSolutionAdminReturns(pub bool);
impl Schema for AdminConfirmationAddSolutionAdminReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"boolean\"}")
    }
}
impl Agent for AdminConfirmationAddSolutionAdminReturns {
    fn topic() -> &'static str {
        "admin_confirmation_addSolutionAdmin"
    }
    fn method() -> &'static str {
        "confirmation_addSolutionAdmin"
    }
    fn agent() -> &'static str {
        "admin"
    }
}