// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; # [derive (Debug , Clone , Serialize , Deserialize)] pub struct TransactionsBalanceGetWithFilterParamsFilterListParams { # [serde (rename = "TypeTransaction")] pub type_transaction : Option < String > , # [serde (rename = "lastDate")] pub last_date : Option < String > , # [serde (rename = "perPage")] pub per_page : Option < f64 > , # [serde (rename = "page")] pub page : Option < f64 > , # [serde (rename = "creator")] pub creator : Option < String > , # [serde (rename = "userId")] pub user_id : Option < String > , # [serde (rename = "reason")] pub reason : Option < String > , # [serde (rename = "firstDate")] pub first_date : Option < String > } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct TransactionsBalanceGetWithFilterParams { # [serde (rename = "filterList")] pub filter_list : TransactionsBalanceGetWithFilterParamsFilterListParams } impl Schema for TransactionsBalanceGetWithFilterParams { fn schema () -> Value { serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"filterList\":{\"type\":\"object\",\"properties\":{\"TypeTransaction\":{\"type\":\"string\"},\"lastDate\":{\"type\":\"string\"},\"perPage\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"page\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"creator\":{\"type\":\"string\"},\"userId\":{\"type\":\"string\"},\"reason\":{\"type\":\"string\"},\"firstDate\":{\"type\":\"string\"}}}},\"required\":[\"filterList\"]}") . unwrap () } } impl Agent for TransactionsBalanceGetWithFilterParams { fn topic () -> & 'static str { "transactions_balance_getWithFilter" } fn method () -> & 'static str { "balance_getWithFilter" } fn agent () -> & 'static str { "transactions" } fn access_level () -> AccessLevel { } } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct TransactionsBalanceGetWithFilterReturnsDataParamsParamsExtraDetailParams { } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct TransactionsBalanceGetWithFilterReturnsDataParamsParams { # [serde (rename = "createdAt")] pub created_at : String , # [serde (rename = "userId")] pub user_id : String , # [serde (rename = "debit")] pub debit : String , # [serde (rename = "currency")] pub currency : String , # [serde (rename = "id")] pub id : f64 , # [serde (rename = "reason")] pub reason : String , # [serde (rename = "extraDetail")] pub extra_detail : Option < TransactionsBalanceGetWithFilterReturnsDataParamsParamsExtraDetailParams > , # [serde (rename = "updatedAt")] pub updated_at : String , # [serde (rename = "credit")] pub credit : String , # [serde (rename = "txId")] pub tx_id : String , # [serde (rename = "creator")] pub creator : String } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct TransactionsBalanceGetWithFilterReturns { # [serde (rename = "data")] pub data : Vec < TransactionsBalanceGetWithFilterReturnsDataParamsParams > , # [serde (rename = "totalCount")] pub total_count : f64 } impl Schema for TransactionsBalanceGetWithFilterReturns { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"data\":{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"createdAt\":{\"type\":\"string\"},\"userId\":{\"type\":\"string\"},\"debit\":{\"type\":\"string\"},\"currency\":{\"type\":\"string\",\"pattern\":\"^(ETH)|(LIS)|(WLIS)|(TLIS)$\"},\"id\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"reason\":{\"type\":\"string\"},\"extraDetail\":{\"type\":\"object\",\"properties\":{}},\"updatedAt\":{\"type\":\"string\"},\"credit\":{\"type\":\"string\"},\"txId\":{\"type\":\"string\"},\"creator\":{\"type\":\"string\"}},\"required\":[\"id\",\"debit\",\"credit\",\"reason\",\"currency\",\"txId\",\"userId\",\"creator\",\"createdAt\",\"updatedAt\"]}},\"totalCount\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"totalCount\",\"data\"]}") } } impl Agent for TransactionsBalanceGetWithFilterReturns { fn topic () -> & 'static str { "transactions_balance_getWithFilter" } fn method () -> & 'static str { "balance_getWithFilter" } fn agent () -> & 'static str { "transactions" } fn access_level () -> AccessLevel { } }