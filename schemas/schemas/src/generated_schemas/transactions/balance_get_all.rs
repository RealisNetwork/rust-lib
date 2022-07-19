// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; # [derive (Debug , Clone , Serialize , Deserialize)] pub struct TransactionsBalanceGetAllParams { # [serde (rename = "userId")] pub user_id : String , # [serde (rename = "page")] pub page : i64 , # [serde (rename = "perPage")] pub per_page : i64 } impl Schema for TransactionsBalanceGetAllParams { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"userId\":{\"type\":\"string\"},\"page\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"perPage\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"userId\",\"page\",\"perPage\"]}") } } impl Agent for TransactionsBalanceGetAllParams { fn topic () -> & 'static str { "transactions_balance_getAll" } fn method () -> & 'static str { "balance_getAll" } fn agent () -> & 'static str { "transactions" } } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct TransactionsBalanceGetAllReturnsDataParamsParamsExtraDetailsParams { } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct TransactionsBalanceGetAllReturnsDataParamsParams { # [serde (rename = "currency")] pub currency : String , # [serde (rename = "createdAt")] pub created_at : String , # [serde (rename = "credit")] pub credit : String , # [serde (rename = "updatedAt")] pub updated_at : String , # [serde (rename = "debit")] pub debit : String , # [serde (rename = "id")] pub id : i64 , # [serde (rename = "reason")] pub reason : String , # [serde (rename = "txId")] pub tx_id : String , # [serde (rename = "userId")] pub user_id : String , # [serde (rename = "creator")] pub creator : String , # [serde (rename = "extraDetails")] pub extra_details : Option < TransactionsBalanceGetAllReturnsDataParamsParamsExtraDetailsParams > } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct TransactionsBalanceGetAllReturns { # [serde (rename = "data")] pub data : Vec < TransactionsBalanceGetAllReturnsDataParamsParams > , # [serde (rename = "totalCount")] pub total_count : i64 } impl Schema for TransactionsBalanceGetAllReturns { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"data\":{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"currency\":{\"type\":\"string\",\"pattern\":\"^(ETH)|(LIS)$\"},\"createdAt\":{\"type\":\"string\"},\"credit\":{\"type\":\"string\"},\"updatedAt\":{\"type\":\"string\"},\"debit\":{\"type\":\"string\"},\"id\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"reason\":{\"type\":\"string\"},\"txId\":{\"type\":\"string\"},\"userId\":{\"type\":\"string\"},\"creator\":{\"type\":\"string\"},\"extraDetails\":{\"type\":\"object\",\"properties\":{},\"required\":null}},\"required\":[\"id\",\"debit\",\"credit\",\"reason\",\"currency\",\"txId\",\"userId\",\"creator\",\"createdAt\",\"updatedAt\"]}},\"totalCount\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"totalCount\",\"data\"]}") } } impl Agent for TransactionsBalanceGetAllReturns { fn topic () -> & 'static str { "transactions_balance_getAll" } fn method () -> & 'static str { "balance_getAll" } fn agent () -> & 'static str { "transactions" } }