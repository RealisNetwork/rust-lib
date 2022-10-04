// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; # [derive (Debug , Clone , Serialize , Deserialize)] pub struct DragonsLobbyGetJackpotWinnersInfoParams { # [serde (rename = "userId")] pub user_id : String } impl Schema for DragonsLobbyGetJackpotWinnersInfoParams { fn schema () -> Value { serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"userId\":{\"type\":\"string\"}},\"required\":[\"userId\"]}") . unwrap () } } impl Agent for DragonsLobbyGetJackpotWinnersInfoParams { fn topic () -> & 'static str { "dragons_lobby_getJackpotWinnersInfo" } fn method () -> & 'static str { "lobby_getJackpotWinnersInfo" } fn agent () -> & 'static str { "dragons" } fn access_level () -> AccessLevel { } } impl < 'de > Deserialize < 'de > for DragonsLobbyGetJackpotWinnersInfoReturns { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { serde_json :: Value :: deserialize (deserializer) ? ; Ok (DragonsLobbyGetJackpotWinnersInfoReturns) } } # [derive (Debug , Clone , Serialize)] pub struct DragonsLobbyGetJackpotWinnersInfoReturns ; impl Schema for DragonsLobbyGetJackpotWinnersInfoReturns { fn schema () -> Value { serde_json :: json ! ("{}") } } impl Agent for DragonsLobbyGetJackpotWinnersInfoReturns { fn topic () -> & 'static str { "dragons_lobby_getJackpotWinnersInfo" } fn method () -> & 'static str { "lobby_getJackpotWinnersInfo" } fn agent () -> & 'static str { "dragons" } fn access_level () -> AccessLevel { } }