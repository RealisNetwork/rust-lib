// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; # [derive (Debug , Clone , Serialize , Deserialize)] pub struct TransactionsBalanceIncreaseBalanceByUserIdParamsExtraDetailsParams { } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct TransactionsBalanceIncreaseBalanceByUserIdParams { # [serde (rename = "creator")] pub creator : String , # [serde (rename = "reason")] pub reason : String , # [serde (rename = "currency")] pub currency : String , # [serde (rename = "extraDetails")] pub extra_details : Option < TransactionsBalanceIncreaseBalanceByUserIdParamsExtraDetailsParams > , # [serde (rename = "amount")] pub amount : String , # [serde (rename = "txId")] pub tx_id : String , # [serde (rename = "userId")] pub user_id : String } impl Schema for TransactionsBalanceIncreaseBalanceByUserIdParams { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"creator\":{\"type\":\"string\"},\"reason\":{\"type\":\"string\"},\"currency\":{\"type\":\"string\",\"pattern\":\"^(ETH)|(LIS)|(WLIS)$\"},\"extraDetails\":{\"type\":\"object\",\"properties\":{},\"required\":null},\"amount\":{\"type\":\"string\"},\"txId\":{\"type\":\"string\"},\"userId\":{\"type\":\"string\"}},\"required\":[\"creator\",\"reason\",\"currency\",\"amount\",\"txId\",\"userId\"]}") } } impl Agent for TransactionsBalanceIncreaseBalanceByUserIdParams { fn topic () -> & 'static str { "transactions_balance_increaseBalanceByUserId" } fn method () -> & 'static str { "balance_increaseBalanceByUserId" } fn agent () -> & 'static str { "transactions" } } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct TransactionsBalanceIncreaseBalanceByUserIdReturns { } impl Schema for TransactionsBalanceIncreaseBalanceByUserIdReturns { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"object\",\"properties\":{},\"required\":null}") } } impl Agent for TransactionsBalanceIncreaseBalanceByUserIdReturns { fn topic () -> & 'static str { "transactions_balance_increaseBalanceByUserId" } fn method () -> & 'static str { "balance_increaseBalanceByUserId" } fn agent () -> & 'static str { "transactions" } }