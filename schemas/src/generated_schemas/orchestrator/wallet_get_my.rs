// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
impl<'de> Deserialize<'de> for OrchestratorWalletGetMyParams {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde_json::Value::deserialize(deserializer)?;
        Ok(OrchestratorWalletGetMyParams)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct OrchestratorWalletGetMyParams;
impl Schema for OrchestratorWalletGetMyParams {
    fn schema() -> Value {
        serde_json::from_str("{}").unwrap()
    }
}
impl Agent for OrchestratorWalletGetMyParams {
    fn topic() -> &'static str {
        "orchestrator_wallet_getMy"
    }
    fn method() -> &'static str {
        "wallet_getMy"
    }
    fn agent() -> &'static str {
        "orchestrator"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Protected
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrchestratorWalletGetMyReturns {
    #[serde(rename = "address")]
    pub address: String,
}
impl Schema for OrchestratorWalletGetMyReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"address\":{\"type\":\"string\"}},\"required\":[\"address\"]}")
    }
}
impl Agent for OrchestratorWalletGetMyReturns {
    fn topic() -> &'static str {
        "orchestrator_wallet_getMy"
    }
    fn method() -> &'static str {
        "wallet_getMy"
    }
    fn agent() -> &'static str {
        "orchestrator"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Protected
    }
}
