// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; # [derive (Debug , Clone , Serialize , Deserialize)] pub struct CatsLobbyGetConfigByIdParams { # [serde (rename = "configKey" , deserialize_with = "deserialize_to_string")] pub config_key : String } impl Schema for CatsLobbyGetConfigByIdParams { fn schema () -> Value { serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"configKey\":{\"type\":\"string\"}},\"required\":[\"configKey\"]}") . unwrap () } } impl Agent for CatsLobbyGetConfigByIdParams { fn topic () -> & 'static str { "cats_lobby_getConfigById" } fn method () -> & 'static str { "lobby_getConfigById" } fn agent () -> & 'static str { "cats" } fn access_level () -> AccessLevel { AccessLevel :: Public } } impl < 'de > Deserialize < 'de > for CatsLobbyGetConfigByIdReturns { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { serde_json :: Value :: deserialize (deserializer) ? ; Ok (CatsLobbyGetConfigByIdReturns) } } # [derive (Debug , Clone , Serialize)] pub struct CatsLobbyGetConfigByIdReturns ; impl Schema for CatsLobbyGetConfigByIdReturns { fn schema () -> Value { serde_json :: json ! ("{}") } } impl Agent for CatsLobbyGetConfigByIdReturns { fn topic () -> & 'static str { "cats_lobby_getConfigById" } fn method () -> & 'static str { "lobby_getConfigById" } fn agent () -> & 'static str { "cats" } fn access_level () -> AccessLevel { AccessLevel :: Public } }