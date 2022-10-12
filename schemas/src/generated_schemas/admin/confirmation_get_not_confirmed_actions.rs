// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
impl<'de> Deserialize<'de> for AdminConfirmationGetNotConfirmedActionsParams {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde_json::Value::deserialize(deserializer)?;
        Ok(AdminConfirmationGetNotConfirmedActionsParams)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct AdminConfirmationGetNotConfirmedActionsParams;
impl Schema for AdminConfirmationGetNotConfirmedActionsParams {
    fn schema() -> Value {
        serde_json::from_str("{}").unwrap()
    }
}
impl Agent for AdminConfirmationGetNotConfirmedActionsParams {
    fn topic() -> &'static str {
        "admin_confirmation_getNotConfirmedActions"
    }
    fn method() -> &'static str {
        "confirmation_getNotConfirmedActions"
    }
    fn agent() -> &'static str {
        "admin"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Private
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminConfirmationGetNotConfirmedActionsReturns(pub f64);
impl Schema for AdminConfirmationGetNotConfirmedActionsReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}")
    }
}
impl Agent for AdminConfirmationGetNotConfirmedActionsReturns {
    fn topic() -> &'static str {
        "admin_confirmation_getNotConfirmedActions"
    }
    fn method() -> &'static str {
        "confirmation_getNotConfirmedActions"
    }
    fn agent() -> &'static str {
        "admin"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Private
    }
}
