// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; # [derive (Debug , Clone , Serialize , Deserialize)] pub struct DragonsLobbyApplicationInitializationParams { # [serde (rename = "userId")] pub user_id : String } impl Schema for DragonsLobbyApplicationInitializationParams { fn schema () -> Value { serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"userId\":{\"type\":\"string\"}},\"required\":[\"userId\"]}") . unwrap () } } impl Agent for DragonsLobbyApplicationInitializationParams { fn topic () -> & 'static str { "dragons_lobby_applicationInitialization" } fn method () -> & 'static str { "lobby_applicationInitialization" } fn agent () -> & 'static str { "dragons" } fn access_level () -> AccessLevel { } } impl < 'de > Deserialize < 'de > for DragonsLobbyApplicationInitializationReturns { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { serde_json :: Value :: deserialize (deserializer) ? ; Ok (DragonsLobbyApplicationInitializationReturns) } } # [derive (Debug , Clone , Serialize)] pub struct DragonsLobbyApplicationInitializationReturns ; impl Schema for DragonsLobbyApplicationInitializationReturns { fn schema () -> Value { serde_json :: json ! ("{}") } } impl Agent for DragonsLobbyApplicationInitializationReturns { fn topic () -> & 'static str { "dragons_lobby_applicationInitialization" } fn method () -> & 'static str { "lobby_applicationInitialization" } fn agent () -> & 'static str { "dragons" } fn access_level () -> AccessLevel { } }