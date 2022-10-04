// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; # [derive (Debug , Clone , Serialize , Deserialize)] pub struct NearAdapterContractIsEnoughBalanceOnWithdrawWalletParams { # [serde (rename = "amount")] pub amount : String } impl Schema for NearAdapterContractIsEnoughBalanceOnWithdrawWalletParams { fn schema () -> Value { serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"amount\":{\"type\":\"string\"}},\"required\":[\"amount\"]}") . unwrap () } } impl Agent for NearAdapterContractIsEnoughBalanceOnWithdrawWalletParams { fn topic () -> & 'static str { "near-adapter_contract_isEnoughBalanceOnWithdrawWallet" } fn method () -> & 'static str { "contract_isEnoughBalanceOnWithdrawWallet" } fn agent () -> & 'static str { "near-adapter" } fn access_level () -> AccessLevel { } } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct NearAdapterContractIsEnoughBalanceOnWithdrawWalletReturns (pub bool) ; impl Schema for NearAdapterContractIsEnoughBalanceOnWithdrawWalletReturns { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"boolean\"}") } } impl Agent for NearAdapterContractIsEnoughBalanceOnWithdrawWalletReturns { fn topic () -> & 'static str { "near-adapter_contract_isEnoughBalanceOnWithdrawWallet" } fn method () -> & 'static str { "contract_isEnoughBalanceOnWithdrawWallet" } fn agent () -> & 'static str { "near-adapter" } fn access_level () -> AccessLevel { } }