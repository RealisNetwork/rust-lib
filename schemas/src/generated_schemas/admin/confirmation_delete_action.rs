// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminConfirmationDeleteActionParams {
    #[serde(rename = "id")]
    pub id: f64,
}
impl Schema for AdminConfirmationDeleteActionParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"id\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"id\"]}")
    }
}
impl Agent for AdminConfirmationDeleteActionParams {
    fn topic() -> &'static str {
        "admin_confirmation_deleteAction"
    }
    fn method() -> &'static str {
        "confirmation_deleteAction"
    }
    fn agent() -> &'static str {
        "admin"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminConfirmationDeleteActionReturns(pub bool);
impl Schema for AdminConfirmationDeleteActionReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"boolean\"}")
    }
}
impl Agent for AdminConfirmationDeleteActionReturns {
    fn topic() -> &'static str {
        "admin_confirmation_deleteAction"
    }
    fn method() -> &'static str {
        "confirmation_deleteAction"
    }
    fn agent() -> &'static str {
        "admin"
    }
}