// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; # [derive (Debug , Clone , Serialize , Deserialize)] pub struct AdminConfirmationGetAllActionsParams { # [serde (rename = "tab")] pub tab : String , # [serde (rename = "page")] pub page : f64 , # [serde (rename = "perPage")] pub per_page : f64 } impl Schema for AdminConfirmationGetAllActionsParams { fn schema () -> Value { serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"tab\":{\"type\":\"string\"},\"page\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"perPage\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"page\",\"perPage\",\"tab\"]}") . unwrap () } } impl Agent for AdminConfirmationGetAllActionsParams { fn topic () -> & 'static str { "admin_confirmation_getAllActions" } fn method () -> & 'static str { "confirmation_getAllActions" } fn agent () -> & 'static str { "admin" } fn access_level () -> AccessLevel { } } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct AdminConfirmationGetAllActionsReturnsDataParamsParamsInfoMethodParamsParamsParams { } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct AdminConfirmationGetAllActionsReturnsDataParamsParamsInfoMethodParams { # [serde (rename = "agent")] pub agent : String , # [serde (rename = "method")] pub method : String , # [serde (rename = "params")] pub params : AdminConfirmationGetAllActionsReturnsDataParamsParamsInfoMethodParamsParamsParams } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct AdminConfirmationGetAllActionsReturnsDataParamsParams { # [serde (rename = "id")] pub id : f64 , # [serde (rename = "isSuccess")] pub is_success : bool , # [serde (rename = "updatedAt")] pub updated_at : String , # [serde (rename = "userId")] pub user_id : String , # [serde (rename = "infoMethod")] pub info_method : AdminConfirmationGetAllActionsReturnsDataParamsParamsInfoMethodParams , # [serde (rename = "nickname")] pub nickname : String , # [serde (rename = "tab")] pub tab : String , # [serde (rename = "createdAt")] pub created_at : String } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct AdminConfirmationGetAllActionsReturns { # [serde (rename = "totalCount")] pub total_count : f64 , # [serde (rename = "data")] pub data : Vec < AdminConfirmationGetAllActionsReturnsDataParamsParams > } impl Schema for AdminConfirmationGetAllActionsReturns { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"totalCount\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"data\":{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"id\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"isSuccess\":{\"type\":\"boolean\"},\"updatedAt\":{\"type\":\"string\"},\"userId\":{\"type\":\"string\"},\"infoMethod\":{\"type\":\"object\",\"properties\":{\"agent\":{\"type\":\"string\"},\"method\":{\"type\":\"string\"},\"params\":{\"type\":\"object\",\"properties\":{}}},\"required\":[\"agent\",\"method\",\"params\"]},\"nickname\":{\"type\":\"string\"},\"tab\":{\"type\":\"string\"},\"createdAt\":{\"type\":\"string\"}},\"required\":[\"id\",\"userId\",\"isSuccess\",\"infoMethod\",\"tab\",\"nickname\",\"createdAt\",\"updatedAt\"]}}},\"required\":[\"data\",\"totalCount\"]}") } } impl Agent for AdminConfirmationGetAllActionsReturns { fn topic () -> & 'static str { "admin_confirmation_getAllActions" } fn method () -> & 'static str { "confirmation_getAllActions" } fn agent () -> & 'static str { "admin" } fn access_level () -> AccessLevel { } }