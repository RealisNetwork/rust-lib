// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; # [derive (Debug , Clone , Serialize , Deserialize)] pub struct AuthAuthGetAllMyActiveSessionsParams { # [serde (rename = "token")] pub token : String } impl Schema for AuthAuthGetAllMyActiveSessionsParams { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"token\":{\"type\":\"string\"}},\"required\":[\"token\"]}") } } impl Agent for AuthAuthGetAllMyActiveSessionsParams { fn topic () -> & 'static str { "auth_auth_getAllMyActiveSessions" } fn method () -> & 'static str { "auth_getAllMyActiveSessions" } fn agent () -> & 'static str { "auth" } } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct AuthAuthGetAllMyActiveSessionsReturns { # [serde (rename = "lastAccess")] pub last_access : i64 , # [serde (rename = "clients")] pub clients : String , # [serde (rename = "username")] pub username : String , # [serde (rename = "id")] pub id : String , # [serde (rename = "ipAddress")] pub ip_address : String , # [serde (rename = "userId")] pub user_id : String , # [serde (rename = "start")] pub start : i64 } impl Schema for AuthAuthGetAllMyActiveSessionsReturns { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"lastAccess\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"clients\":{\"type\":\"string\"},\"username\":{\"type\":\"string\"},\"id\":{\"type\":\"string\"},\"ipAddress\":{\"type\":\"string\"},\"userId\":{\"type\":\"string\"},\"start\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"id\",\"username\",\"userId\",\"ipAddress\",\"start\",\"lastAccess\",\"clients\"]}") } } impl Agent for AuthAuthGetAllMyActiveSessionsReturns { fn topic () -> & 'static str { "auth_auth_getAllMyActiveSessions" } fn method () -> & 'static str { "auth_getAllMyActiveSessions" } fn agent () -> & 'static str { "auth" } }