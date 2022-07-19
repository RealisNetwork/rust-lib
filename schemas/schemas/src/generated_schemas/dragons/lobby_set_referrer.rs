// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; # [derive (Debug , Clone , Serialize , Deserialize)] pub struct DragonsLobbySetReferrerParams { # [serde (rename = "siteReferrerId")] pub site_referrer_id : i64 , # [serde (rename = "userId")] pub user_id : String } impl Schema for DragonsLobbySetReferrerParams { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"siteReferrerId\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"userId\":{\"type\":\"string\"}},\"required\":[\"userId\",\"siteReferrerId\"]}") } } impl Agent for DragonsLobbySetReferrerParams { fn topic () -> & 'static str { "dragons_lobby_setReferrer" } fn method () -> & 'static str { "lobby_setReferrer" } fn agent () -> & 'static str { "dragons" } } impl < 'de > Deserialize < 'de > for DragonsLobbySetReferrerReturns { fn deserialize < D > (_deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { Ok (DragonsLobbySetReferrerReturns) } } # [derive (Debug , Clone , Serialize)] pub struct DragonsLobbySetReferrerReturns ; impl Schema for DragonsLobbySetReferrerReturns { fn schema () -> Value { serde_json :: json ! ("{}") } } impl Agent for DragonsLobbySetReferrerReturns { fn topic () -> & 'static str { "dragons_lobby_setReferrer" } fn method () -> & 'static str { "lobby_setReferrer" } fn agent () -> & 'static str { "dragons" } }