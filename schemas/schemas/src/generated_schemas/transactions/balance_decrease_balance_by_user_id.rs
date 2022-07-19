// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; # [derive (Debug , Clone , Serialize , Deserialize)] pub struct TransactionsBalanceDecreaseBalanceByUserIdParamsExtraDetailsParams { } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct TransactionsBalanceDecreaseBalanceByUserIdParams { # [serde (rename = "currency")] pub currency : String , # [serde (rename = "extraDetails")] pub extra_details : Option < TransactionsBalanceDecreaseBalanceByUserIdParamsExtraDetailsParams > , # [serde (rename = "reason")] pub reason : String , # [serde (rename = "txId")] pub tx_id : String , # [serde (rename = "creator")] pub creator : String , # [serde (rename = "amount")] pub amount : String , # [serde (rename = "userId")] pub user_id : String } impl Schema for TransactionsBalanceDecreaseBalanceByUserIdParams { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"currency\":{\"type\":\"string\",\"pattern\":\"^(ETH)|(LIS)$\"},\"extraDetails\":{\"type\":\"object\",\"properties\":{},\"required\":null},\"reason\":{\"type\":\"string\"},\"txId\":{\"type\":\"string\"},\"creator\":{\"type\":\"string\"},\"amount\":{\"type\":\"string\"},\"userId\":{\"type\":\"string\"}},\"required\":[\"creator\",\"reason\",\"currency\",\"amount\",\"txId\",\"userId\"]}") } } impl Agent for TransactionsBalanceDecreaseBalanceByUserIdParams { fn topic () -> & 'static str { "transactions_balance_decreaseBalanceByUserId" } fn method () -> & 'static str { "balance_decreaseBalanceByUserId" } fn agent () -> & 'static str { "transactions" } } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct TransactionsBalanceDecreaseBalanceByUserIdReturns { } impl Schema for TransactionsBalanceDecreaseBalanceByUserIdReturns { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"object\",\"properties\":{},\"required\":null}") } } impl Agent for TransactionsBalanceDecreaseBalanceByUserIdReturns { fn topic () -> & 'static str { "transactions_balance_decreaseBalanceByUserId" } fn method () -> & 'static str { "balance_decreaseBalanceByUserId" } fn agent () -> & 'static str { "transactions" } }