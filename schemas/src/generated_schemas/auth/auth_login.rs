// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; # [derive (Debug , Clone , Serialize , Deserialize)] pub struct AuthAuthLoginParams { # [serde (rename = "password")] pub password : String , # [serde (rename = "deviceId")] pub device_id : Option < String > , # [serde (rename = "appId")] pub app_id : Option < i32 > , # [serde (rename = "username")] pub username : String } impl Schema for AuthAuthLoginParams { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"password\":{\"type\":\"string\"},\"deviceId\":{\"type\":\"string\"},\"appId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"username\":{\"type\":\"string\"}},\"required\":[\"username\",\"password\"]}") } } impl Agent for AuthAuthLoginParams { fn topic () -> & 'static str { "auth_auth_login" } fn method () -> & 'static str { "auth_login" } fn agent () -> & 'static str { "auth" } } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct AuthAuthLoginReturns { # [serde (rename = "refresh_token")] pub refresh_token : String , # [serde (rename = "expires_in")] pub expires_in : i32 , # [serde (rename = "refresh_expires_in")] pub refresh_expires_in : i32 , # [serde (rename = "access_token")] pub access_token : String , # [serde (rename = "userId")] pub user_id : String } impl Schema for AuthAuthLoginReturns { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"refresh_token\":{\"type\":\"string\"},\"expires_in\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"refresh_expires_in\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"access_token\":{\"type\":\"string\"},\"userId\":{\"type\":\"string\"}},\"required\":[\"access_token\",\"expires_in\",\"refresh_expires_in\",\"refresh_token\",\"userId\"]}") } } impl Agent for AuthAuthLoginReturns { fn topic () -> & 'static str { "auth_auth_login" } fn method () -> & 'static str { "auth_login" } fn agent () -> & 'static str { "auth" } }