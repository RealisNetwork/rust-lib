// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; impl < 'de > Deserialize < 'de > for AdminUserRoleGetAllWithNicknameParams { fn deserialize < D > (_deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { Ok (AdminUserRoleGetAllWithNicknameParams) } } # [derive (Debug , Clone , Serialize)] pub struct AdminUserRoleGetAllWithNicknameParams ; impl Schema for AdminUserRoleGetAllWithNicknameParams { fn schema () -> Value { serde_json :: json ! ("{}") } } impl Agent for AdminUserRoleGetAllWithNicknameParams { fn topic () -> & 'static str { "admin_userRole_getAllWithNickname" } fn method () -> & 'static str { "userRole_getAllWithNickname" } fn agent () -> & 'static str { "admin" } } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct AdminUserRoleGetAllWithNicknameReturnsParams { # [serde (rename = "userId")] pub user_id : String , # [serde (rename = "role")] pub role : String , # [serde (rename = "nickname")] pub nickname : String } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct AdminUserRoleGetAllWithNicknameReturns (Vec < AdminUserRoleGetAllWithNicknameReturnsParams >) ; impl Schema for AdminUserRoleGetAllWithNicknameReturns { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"userId\":{\"type\":\"string\"},\"role\":{\"type\":\"string\"},\"nickname\":{\"type\":\"string\"}},\"required\":[\"userId\",\"nickname\",\"role\"]}}") } } impl Agent for AdminUserRoleGetAllWithNicknameReturns { fn topic () -> & 'static str { "admin_userRole_getAllWithNickname" } fn method () -> & 'static str { "userRole_getAllWithNickname" } fn agent () -> & 'static str { "admin" } }