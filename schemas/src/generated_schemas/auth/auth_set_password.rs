// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; # [derive (Debug , Clone , Serialize , Deserialize)] pub struct AuthAuthSetPasswordParams { # [serde (rename = "providerId")] pub provider_id : Option < String > , # [serde (rename = "password")] pub password : String , # [serde (rename = "appId")] pub app_id : Option < i32 > , # [serde (rename = "passwordHash")] pub password_hash : String } impl Schema for AuthAuthSetPasswordParams { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"providerId\":{\"type\":\"string\"},\"password\":{\"type\":\"string\"},\"appId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"passwordHash\":{\"type\":\"string\"}},\"required\":[\"passwordHash\",\"password\"]}") } } impl Agent for AuthAuthSetPasswordParams { fn topic () -> & 'static str { "auth_auth_setPassword" } fn method () -> & 'static str { "auth_setPassword" } fn agent () -> & 'static str { "auth" } } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct AuthAuthSetPasswordReturns { # [serde (rename = "refresh_expires_in")] pub refresh_expires_in : i32 , # [serde (rename = "access_token")] pub access_token : String , # [serde (rename = "id_token")] pub id_token : String , # [serde (rename = "refresh_token")] pub refresh_token : String , # [serde (rename = "userId")] pub user_id : String , # [serde (rename = "expires_in")] pub expires_in : i32 } impl Schema for AuthAuthSetPasswordReturns { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"refresh_expires_in\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"access_token\":{\"type\":\"string\"},\"id_token\":{\"type\":\"string\"},\"refresh_token\":{\"type\":\"string\"},\"userId\":{\"type\":\"string\"},\"expires_in\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}}},\"required\":[\"access_token\",\"expires_in\",\"id_token\",\"refresh_expires_in\",\"refresh_token\",\"userId\"]}") } } impl Agent for AuthAuthSetPasswordReturns { fn topic () -> & 'static str { "auth_auth_setPassword" } fn method () -> & 'static str { "auth_setPassword" } fn agent () -> & 'static str { "auth" } }