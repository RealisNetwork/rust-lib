// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; # [derive (Debug , Clone , Serialize , Deserialize)] pub struct OrchestratorBalanceUserBlockchainDepositParams { # [serde (rename = "userId" , deserialize_with = "deserialize_to_string")] pub user_id : String , # [serde (rename = "creator" , deserialize_with = "deserialize_to_string")] pub creator : String , # [serde (rename = "txId" , deserialize_with = "deserialize_to_string")] pub tx_id : String , # [serde (rename = "currency" , deserialize_with = "deserialize_to_string")] pub currency : String , # [serde (rename = "amount" , deserialize_with = "deserialize_to_string")] pub amount : String } impl Schema for OrchestratorBalanceUserBlockchainDepositParams { fn schema () -> Value { serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"userId\":{\"type\":\"string\"},\"creator\":{\"type\":\"string\"},\"txId\":{\"type\":\"string\"},\"currency\":{\"type\":\"string\",\"pattern\":\"^(ETH)|(LIS)|(WLIS)|(TLIS)$\"},\"amount\":{\"type\":\"string\"}},\"required\":[\"userId\",\"currency\",\"amount\",\"creator\",\"txId\"]}") . unwrap () } } impl Agent for OrchestratorBalanceUserBlockchainDepositParams { fn topic () -> & 'static str { "orchestrator_balance_userBlockchainDeposit" } fn method () -> & 'static str { "balance_userBlockchainDeposit" } fn agent () -> & 'static str { "orchestrator" } fn access_level () -> AccessLevel { AccessLevel :: Internal } } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct OrchestratorBalanceUserBlockchainDepositReturns (pub bool) ; impl Schema for OrchestratorBalanceUserBlockchainDepositReturns { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"boolean\"}") } } impl Agent for OrchestratorBalanceUserBlockchainDepositReturns { fn topic () -> & 'static str { "orchestrator_balance_userBlockchainDeposit" } fn method () -> & 'static str { "balance_userBlockchainDeposit" } fn agent () -> & 'static str { "orchestrator" } fn access_level () -> AccessLevel { AccessLevel :: Internal } }