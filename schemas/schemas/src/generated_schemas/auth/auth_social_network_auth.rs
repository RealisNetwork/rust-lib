// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; # [derive (Debug , Clone , Serialize , Deserialize)] pub struct AuthAuthSocialNetworkAuthParams { # [serde (rename = "credential")] pub credential : String , # [serde (rename = "provider")] pub provider : String } impl Schema for AuthAuthSocialNetworkAuthParams { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"credential\":{\"type\":\"string\"},\"provider\":{\"type\":\"string\"}},\"required\":[\"credential\",\"provider\"]}") } } impl Agent for AuthAuthSocialNetworkAuthParams { fn topic () -> & 'static str { "auth_auth_socialNetworkAuth" } fn method () -> & 'static str { "auth_socialNetworkAuth" } fn agent () -> & 'static str { "auth" } } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct AuthAuthSocialNetworkAuthReturns { # [serde (rename = "refresh_token")] pub refresh_token : Option < String > , # [serde (rename = "expires_in")] pub expires_in : i32 , # [serde (rename = "refresh_expires_in")] pub refresh_expires_in : Option < i32 > , # [serde (rename = "userId")] pub user_id : String , # [serde (rename = "access_token")] pub access_token : String } impl Schema for AuthAuthSocialNetworkAuthReturns { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"refresh_token\":{\"type\":\"string\"},\"expires_in\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"refresh_expires_in\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"userId\":{\"type\":\"string\"},\"access_token\":{\"type\":\"string\"}},\"required\":[\"access_token\",\"expires_in\",\"userId\"]}") } } impl Agent for AuthAuthSocialNetworkAuthReturns { fn topic () -> & 'static str { "auth_auth_socialNetworkAuth" } fn method () -> & 'static str { "auth_socialNetworkAuth" } fn agent () -> & 'static str { "auth" } }