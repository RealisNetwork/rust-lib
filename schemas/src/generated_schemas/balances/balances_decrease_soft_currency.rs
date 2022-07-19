// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; # [derive (Debug , Clone , Serialize , Deserialize)] pub struct BalancesBalancesDecreaseSoftCurrencyParams { # [serde (rename = "amount")] pub amount : i32 , # [serde (rename = "txId")] pub tx_id : String } impl Schema for BalancesBalancesDecreaseSoftCurrencyParams { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"amount\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"txId\":{\"type\":\"string\"}},\"required\":[\"amount\",\"txId\"]}") } } impl Agent for BalancesBalancesDecreaseSoftCurrencyParams { fn topic () -> & 'static str { "balances_balances_decreaseSoftCurrency" } fn method () -> & 'static str { "balances_decreaseSoftCurrency" } fn agent () -> & 'static str { "balances" } } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct BalancesBalancesDecreaseSoftCurrencyReturns { # [serde (rename = "balance")] pub balance : i32 } impl Schema for BalancesBalancesDecreaseSoftCurrencyReturns { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"balance\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}}},\"required\":[\"balance\"]}") } } impl Agent for BalancesBalancesDecreaseSoftCurrencyReturns { fn topic () -> & 'static str { "balances_balances_decreaseSoftCurrency" } fn method () -> & 'static str { "balances_decreaseSoftCurrency" } fn agent () -> & 'static str { "balances" } }