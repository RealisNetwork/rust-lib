// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; # [derive (Debug , Clone , Serialize , Deserialize)] pub struct TransactionsBalanceGetBalanceByUserIdParams { # [serde (rename = "currency")] pub currency : String } impl Schema for TransactionsBalanceGetBalanceByUserIdParams { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"currency\":{\"type\":\"string\",\"pattern\":\"^(ETH)|(LIS)|(WLIS)$\"}},\"required\":[\"currency\"]}") } } impl Agent for TransactionsBalanceGetBalanceByUserIdParams { fn topic () -> & 'static str { "transactions_balance_getBalanceByUserId" } fn method () -> & 'static str { "balance_getBalanceByUserId" } fn agent () -> & 'static str { "transactions" } } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct TransactionsBalanceGetBalanceByUserIdReturns { # [serde (rename = "amount")] pub amount : String } impl Schema for TransactionsBalanceGetBalanceByUserIdReturns { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"amount\":{\"type\":\"string\"}},\"required\":[\"amount\"]}") } } impl Agent for TransactionsBalanceGetBalanceByUserIdReturns { fn topic () -> & 'static str { "transactions_balance_getBalanceByUserId" } fn method () -> & 'static str { "balance_getBalanceByUserId" } fn agent () -> & 'static str { "transactions" } }