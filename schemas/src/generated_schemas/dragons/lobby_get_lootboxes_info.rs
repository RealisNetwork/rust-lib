// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; # [derive (Debug , Clone , Serialize , Deserialize)] pub struct DragonsLobbyGetLootboxesInfoParams { # [serde (rename = "userId")] pub user_id : String } impl Schema for DragonsLobbyGetLootboxesInfoParams { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"userId\":{\"type\":\"string\"}},\"required\":[\"userId\"]}") } } impl Agent for DragonsLobbyGetLootboxesInfoParams { fn topic () -> & 'static str { "dragons_lobby_getLootboxesInfo" } fn method () -> & 'static str { "lobby_getLootboxesInfo" } fn agent () -> & 'static str { "dragons" } } impl < 'de > Deserialize < 'de > for DragonsLobbyGetLootboxesInfoReturns { fn deserialize < D > (_deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { Ok (DragonsLobbyGetLootboxesInfoReturns) } } # [derive (Debug , Clone , Serialize)] pub struct DragonsLobbyGetLootboxesInfoReturns ; impl Schema for DragonsLobbyGetLootboxesInfoReturns { fn schema () -> Value { serde_json :: json ! ("{}") } } impl Agent for DragonsLobbyGetLootboxesInfoReturns { fn topic () -> & 'static str { "dragons_lobby_getLootboxesInfo" } fn method () -> & 'static str { "lobby_getLootboxesInfo" } fn agent () -> & 'static str { "dragons" } }