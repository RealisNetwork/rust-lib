// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; # [derive (Debug , Clone , Serialize , Deserialize)] pub struct SoulAdapterWalletGetBalanceByAddressParams { # [serde (rename = "address")] pub address : String } impl Schema for SoulAdapterWalletGetBalanceByAddressParams { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"address\":{\"type\":\"string\"}},\"required\":[\"address\"]}") } } impl Agent for SoulAdapterWalletGetBalanceByAddressParams { fn topic () -> & 'static str { "soul-adapter_wallet_getBalanceByAddress" } fn method () -> & 'static str { "wallet_getBalanceByAddress" } fn agent () -> & 'static str { "soul-adapter" } } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct SoulAdapterWalletGetBalanceByAddressReturns (pub String) ; impl Schema for SoulAdapterWalletGetBalanceByAddressReturns { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"string\"}") } } impl Agent for SoulAdapterWalletGetBalanceByAddressReturns { fn topic () -> & 'static str { "soul-adapter_wallet_getBalanceByAddress" } fn method () -> & 'static str { "wallet_getBalanceByAddress" } fn agent () -> & 'static str { "soul-adapter" } }