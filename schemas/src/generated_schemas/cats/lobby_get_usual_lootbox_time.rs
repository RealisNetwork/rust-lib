// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; # [derive (Debug , Clone , Serialize , Deserialize)] pub struct CatsLobbyGetUsualLootboxTimeParams { } impl Schema for CatsLobbyGetUsualLootboxTimeParams { fn schema () -> Value { serde_json :: from_str ("{\"type\":\"object\",\"properties\":{}}") . unwrap () } } impl Agent for CatsLobbyGetUsualLootboxTimeParams { fn topic () -> & 'static str { "cats_lobby_getUsualLootboxTime" } fn method () -> & 'static str { "lobby_getUsualLootboxTime" } fn agent () -> & 'static str { "cats" } fn access_level () -> AccessLevel { AccessLevel :: Public } } impl < 'de > Deserialize < 'de > for CatsLobbyGetUsualLootboxTimeReturns { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { serde_json :: Value :: deserialize (deserializer) ? ; Ok (CatsLobbyGetUsualLootboxTimeReturns) } } # [derive (Debug , Clone , Serialize)] pub struct CatsLobbyGetUsualLootboxTimeReturns ; impl Schema for CatsLobbyGetUsualLootboxTimeReturns { fn schema () -> Value { serde_json :: json ! ("{}") } } impl Agent for CatsLobbyGetUsualLootboxTimeReturns { fn topic () -> & 'static str { "cats_lobby_getUsualLootboxTime" } fn method () -> & 'static str { "lobby_getUsualLootboxTime" } fn agent () -> & 'static str { "cats" } fn access_level () -> AccessLevel { AccessLevel :: Public } }