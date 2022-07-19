// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; # [derive (Debug , Clone , Serialize , Deserialize)] pub struct TransactionsBalanceGetWithFilterParamsFilterListParams { # [serde (rename = "TypeTransaction")] pub type_transaction : String , # [serde (rename = "firstDate")] pub first_date : String , # [serde (rename = "reason")] pub reason : String , # [serde (rename = "creator")] pub creator : String , # [serde (rename = "page")] pub page : i64 , # [serde (rename = "userId")] pub user_id : String , # [serde (rename = "lastDate")] pub last_date : String , # [serde (rename = "perPage")] pub per_page : i64 } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct TransactionsBalanceGetWithFilterParams { # [serde (rename = "filterList")] pub filter_list : TransactionsBalanceGetWithFilterParamsFilterListParams } impl Schema for TransactionsBalanceGetWithFilterParams { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"filterList\":{\"type\":\"object\",\"properties\":{\"TypeTransaction\":{\"type\":\"string\"},\"firstDate\":{\"type\":\"string\"},\"reason\":{\"type\":\"string\"},\"creator\":{\"type\":\"string\"},\"page\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"userId\":{\"type\":\"string\"},\"lastDate\":{\"type\":\"string\"},\"perPage\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":null}},\"required\":[\"filterList\"]}") } } impl Agent for TransactionsBalanceGetWithFilterParams { fn topic () -> & 'static str { "transactions_balance_getWithFilter" } fn method () -> & 'static str { "balance_getWithFilter" } fn agent () -> & 'static str { "transactions" } } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct TransactionsBalanceGetWithFilterReturnsDataParamsParamsExtraDetailsParams { } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct TransactionsBalanceGetWithFilterReturnsDataParamsParams { # [serde (rename = "updatedAt")] pub updated_at : String , # [serde (rename = "creator")] pub creator : String , # [serde (rename = "txId")] pub tx_id : String , # [serde (rename = "id")] pub id : i64 , # [serde (rename = "debit")] pub debit : String , # [serde (rename = "createdAt")] pub created_at : String , # [serde (rename = "credit")] pub credit : String , # [serde (rename = "userId")] pub user_id : String , # [serde (rename = "reason")] pub reason : String , # [serde (rename = "currency")] pub currency : String , # [serde (rename = "extraDetails")] pub extra_details : Option < TransactionsBalanceGetWithFilterReturnsDataParamsParamsExtraDetailsParams > } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct TransactionsBalanceGetWithFilterReturns { # [serde (rename = "data")] pub data : Vec < TransactionsBalanceGetWithFilterReturnsDataParamsParams > , # [serde (rename = "totalCount")] pub total_count : i64 } impl Schema for TransactionsBalanceGetWithFilterReturns { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"data\":{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"updatedAt\":{\"type\":\"string\"},\"creator\":{\"type\":\"string\"},\"txId\":{\"type\":\"string\"},\"id\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"debit\":{\"type\":\"string\"},\"createdAt\":{\"type\":\"string\"},\"credit\":{\"type\":\"string\"},\"userId\":{\"type\":\"string\"},\"reason\":{\"type\":\"string\"},\"currency\":{\"type\":\"string\",\"pattern\":\"^(ETH)|(LIS)$\"},\"extraDetails\":{\"type\":\"object\",\"properties\":{},\"required\":null}},\"required\":[\"id\",\"debit\",\"credit\",\"reason\",\"currency\",\"txId\",\"userId\",\"creator\",\"createdAt\",\"updatedAt\"]}},\"totalCount\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"totalCount\",\"data\"]}") } } impl Agent for TransactionsBalanceGetWithFilterReturns { fn topic () -> & 'static str { "transactions_balance_getWithFilter" } fn method () -> & 'static str { "balance_getWithFilter" } fn agent () -> & 'static str { "transactions" } }