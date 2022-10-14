// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; impl < 'de > Deserialize < 'de > for CatsLobbyGetServerTimeParams { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { serde_json :: Value :: deserialize (deserializer) ? ; Ok (CatsLobbyGetServerTimeParams) } } # [derive (Debug , Clone , Serialize)] pub struct CatsLobbyGetServerTimeParams ; impl Schema for CatsLobbyGetServerTimeParams { fn schema () -> Value { serde_json :: from_str ("{}") . unwrap () } } impl Agent for CatsLobbyGetServerTimeParams { fn topic () -> & 'static str { "cats_lobby_getServerTime" } fn method () -> & 'static str { "lobby_getServerTime" } fn agent () -> & 'static str { "cats" } fn access_level () -> AccessLevel { AccessLevel :: Public } } impl < 'de > Deserialize < 'de > for CatsLobbyGetServerTimeReturns { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { serde_json :: Value :: deserialize (deserializer) ? ; Ok (CatsLobbyGetServerTimeReturns) } } # [derive (Debug , Clone , Serialize)] pub struct CatsLobbyGetServerTimeReturns ; impl Schema for CatsLobbyGetServerTimeReturns { fn schema () -> Value { serde_json :: json ! ("{}") } } impl Agent for CatsLobbyGetServerTimeReturns { fn topic () -> & 'static str { "cats_lobby_getServerTime" } fn method () -> & 'static str { "lobby_getServerTime" } fn agent () -> & 'static str { "cats" } fn access_level () -> AccessLevel { AccessLevel :: Public } }