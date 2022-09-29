// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrchestratorBalanceUserBlockchainDepositParams {
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "creator")]
    pub creator: String,
    #[serde(rename = "amount")]
    pub amount: String,
    #[serde(rename = "txId")]
    pub tx_id: String,
    #[serde(rename = "currency")]
    pub currency: String,
}
impl Schema for OrchestratorBalanceUserBlockchainDepositParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"userId\":{\"type\":\"string\"},\"creator\":{\"type\":\"string\"},\"amount\":{\"type\":\"string\"},\"txId\":{\"type\":\"string\"},\"currency\":{\"type\":\"string\",\"pattern\":\"^(ETH)|(LIS)|(WLIS)|(TLIS)$\"}},\"required\":[\"userId\",\"currency\",\"amount\",\"creator\",\"txId\"]}") . unwrap ()
    }
}
impl Agent for OrchestratorBalanceUserBlockchainDepositParams {
    fn topic() -> &'static str {
        "orchestrator_balance_userBlockchainDeposit"
    }
    fn method() -> &'static str {
        "balance_userBlockchainDeposit"
    }
    fn agent() -> &'static str {
        "orchestrator"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrchestratorBalanceUserBlockchainDepositReturns(pub bool);
impl Schema for OrchestratorBalanceUserBlockchainDepositReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"boolean\"}")
    }
}
impl Agent for OrchestratorBalanceUserBlockchainDepositReturns {
    fn topic() -> &'static str {
        "orchestrator_balance_userBlockchainDeposit"
    }
    fn method() -> &'static str {
        "balance_userBlockchainDeposit"
    }
    fn agent() -> &'static str {
        "orchestrator"
    }
}
