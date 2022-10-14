// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; # [derive (Debug , Clone , Serialize , Deserialize)] pub struct CatsLobbyResetDeviceIdParams { # [serde (rename = "userId" , deserialize_with = "deserialize_to_string")] pub user_id : String } impl Schema for CatsLobbyResetDeviceIdParams { fn schema () -> Value { serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"userId\":{\"type\":\"string\"}},\"required\":[\"userId\"]}") . unwrap () } } impl Agent for CatsLobbyResetDeviceIdParams { fn topic () -> & 'static str { "cats_lobby_resetDeviceId" } fn method () -> & 'static str { "lobby_resetDeviceId" } fn agent () -> & 'static str { "cats" } fn access_level () -> AccessLevel { AccessLevel :: Protected } } impl < 'de > Deserialize < 'de > for CatsLobbyResetDeviceIdReturns { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { serde_json :: Value :: deserialize (deserializer) ? ; Ok (CatsLobbyResetDeviceIdReturns) } } # [derive (Debug , Clone , Serialize)] pub struct CatsLobbyResetDeviceIdReturns ; impl Schema for CatsLobbyResetDeviceIdReturns { fn schema () -> Value { serde_json :: json ! ("{}") } } impl Agent for CatsLobbyResetDeviceIdReturns { fn topic () -> & 'static str { "cats_lobby_resetDeviceId" } fn method () -> & 'static str { "lobby_resetDeviceId" } fn agent () -> & 'static str { "cats" } fn access_level () -> AccessLevel { AccessLevel :: Protected } }