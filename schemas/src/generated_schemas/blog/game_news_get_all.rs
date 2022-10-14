// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; # [derive (Debug , Clone , Serialize , Deserialize)] pub struct BlogGameNewsGetAllParams { # [serde (rename = "page")] pub page : f64 , # [serde (rename = "perPage")] pub per_page : f64 } impl Schema for BlogGameNewsGetAllParams { fn schema () -> Value { serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"page\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"perPage\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"page\",\"perPage\"]}") . unwrap () } } impl Agent for BlogGameNewsGetAllParams { fn topic () -> & 'static str { "blog_gameNews_getAll" } fn method () -> & 'static str { "gameNews_getAll" } fn agent () -> & 'static str { "blog" } fn access_level () -> AccessLevel { AccessLevel :: Public } } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct BlogGameNewsGetAllReturnsParamsDataParamsParams { # [serde (rename = "content" , deserialize_with = "deserialize_to_string")] pub content : String , # [serde (rename = "image" , deserialize_with = "deserialize_to_string")] pub image : String , # [serde (rename = "appId")] pub app_id : f64 , # [serde (rename = "createdAt" , deserialize_with = "deserialize_to_string")] pub created_at : String , # [serde (rename = "updatedAt" , deserialize_with = "deserialize_to_string")] pub updated_at : String , # [serde (rename = "id")] pub id : f64 , # [serde (rename = "title" , deserialize_with = "deserialize_to_string")] pub title : String } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct BlogGameNewsGetAllReturnsParams { # [serde (rename = "data")] pub data : Vec < BlogGameNewsGetAllReturnsParamsDataParamsParams > , # [serde (rename = "totalCount")] pub total_count : f64 } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct BlogGameNewsGetAllReturns (pub Vec < BlogGameNewsGetAllReturnsParams >) ; impl Schema for BlogGameNewsGetAllReturns { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"data\":{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"content\":{\"type\":\"string\"},\"image\":{\"type\":\"string\"},\"appId\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"createdAt\":{\"type\":\"string\"},\"updatedAt\":{\"type\":\"string\"},\"id\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"title\":{\"type\":\"string\"}},\"required\":[\"id\",\"title\",\"content\",\"image\",\"appId\",\"createdAt\",\"updatedAt\"]}},\"totalCount\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"data\",\"totalCount\"]}}") } } impl Agent for BlogGameNewsGetAllReturns { fn topic () -> & 'static str { "blog_gameNews_getAll" } fn method () -> & 'static str { "gameNews_getAll" } fn agent () -> & 'static str { "blog" } fn access_level () -> AccessLevel { AccessLevel :: Public } }