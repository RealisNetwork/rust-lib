// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; # [derive (Debug , Clone , Serialize , Deserialize)] pub struct TransactionsBalanceGetListWithPaginationParams { # [serde (rename = "page")] pub page : i32 } impl Schema for TransactionsBalanceGetListWithPaginationParams { fn schema () -> Value { serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"page\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}}},\"required\":[\"page\"]}") . unwrap () } } impl Agent for TransactionsBalanceGetListWithPaginationParams { fn topic () -> & 'static str { "transactions_balance_getListWithPagination" } fn method () -> & 'static str { "balance_getListWithPagination" } fn agent () -> & 'static str { "transactions" } fn access_level () -> AccessLevel { AccessLevel :: Protected } } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct TransactionsBalanceGetListWithPaginationReturnsListParamsParams { # [serde (rename = "dateTime" , deserialize_with = "deserialize_to_string")] pub date_time : String , # [serde (rename = "blockId" , deserialize_with = "deserialize_to_string")] pub block_id : String , # [serde (rename = "balanceChange" , deserialize_with = "deserialize_to_string")] pub balance_change : String } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct TransactionsBalanceGetListWithPaginationReturns { # [serde (rename = "pages")] pub pages : i32 , # [serde (rename = "page")] pub page : i32 , # [serde (rename = "list")] pub list : Vec < TransactionsBalanceGetListWithPaginationReturnsListParamsParams > } impl Schema for TransactionsBalanceGetListWithPaginationReturns { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"pages\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"page\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"list\":{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"dateTime\":{\"type\":\"string\"},\"blockId\":{\"type\":\"string\"},\"balanceChange\":{\"type\":\"string\"}},\"required\":[\"blockId\",\"dateTime\",\"balanceChange\"]}}},\"required\":[\"page\",\"pages\",\"list\"]}") } } impl Agent for TransactionsBalanceGetListWithPaginationReturns { fn topic () -> & 'static str { "transactions_balance_getListWithPagination" } fn method () -> & 'static str { "balance_getListWithPagination" } fn agent () -> & 'static str { "transactions" } fn access_level () -> AccessLevel { AccessLevel :: Protected } }