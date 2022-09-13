// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; # [derive (Debug , Clone , Serialize , Deserialize)] pub struct TransactionsBalanceGetAllMyParams { # [serde (rename = "perPage")] pub per_page : f64 , # [serde (rename = "page")] pub page : f64 } impl Schema for TransactionsBalanceGetAllMyParams { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"perPage\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"page\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"page\",\"perPage\"]}") } } impl Agent for TransactionsBalanceGetAllMyParams { fn topic () -> & 'static str { "transactions_balance_getAllMy" } fn method () -> & 'static str { "balance_getAllMy" } fn agent () -> & 'static str { "transactions" } } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct TransactionsBalanceGetAllMyReturnsParamsExtraDetailsParams { } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct TransactionsBalanceGetAllMyReturnsParams { # [serde (rename = "updatedAt")] pub updated_at : String , # [serde (rename = "totalCount")] pub total_count : f64 , # [serde (rename = "id")] pub id : f64 , # [serde (rename = "creator")] pub creator : String , # [serde (rename = "txId")] pub tx_id : String , # [serde (rename = "extraDetails")] pub extra_details : Option < TransactionsBalanceGetAllMyReturnsParamsExtraDetailsParams > , # [serde (rename = "userId")] pub user_id : String , # [serde (rename = "reason")] pub reason : String , # [serde (rename = "credit")] pub credit : String , # [serde (rename = "currency")] pub currency : String , # [serde (rename = "debit")] pub debit : String , # [serde (rename = "createdAt")] pub created_at : String } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct TransactionsBalanceGetAllMyReturns (pub Vec < TransactionsBalanceGetAllMyReturnsParams >) ; impl Schema for TransactionsBalanceGetAllMyReturns { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"updatedAt\":{\"type\":\"string\"},\"totalCount\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"id\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"creator\":{\"type\":\"string\"},\"txId\":{\"type\":\"string\"},\"extraDetails\":{\"type\":\"object\",\"properties\":{},\"required\":null},\"userId\":{\"type\":\"string\"},\"reason\":{\"type\":\"string\"},\"credit\":{\"type\":\"string\"},\"currency\":{\"type\":\"string\",\"pattern\":\"^(ETH)|(LIS)|(WLIS)$\"},\"debit\":{\"type\":\"string\"},\"createdAt\":{\"type\":\"string\"}},\"required\":[\"totalCount\",\"id\",\"debit\",\"credit\",\"reason\",\"currency\",\"txId\",\"userId\",\"creator\",\"createdAt\",\"updatedAt\"]}}") } } impl Agent for TransactionsBalanceGetAllMyReturns { fn topic () -> & 'static str { "transactions_balance_getAllMy" } fn method () -> & 'static str { "balance_getAllMy" } fn agent () -> & 'static str { "transactions" } }