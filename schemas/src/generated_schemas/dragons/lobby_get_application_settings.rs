// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; impl < 'de > Deserialize < 'de > for DragonsLobbyGetApplicationSettingsParams { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { serde_json :: Value :: deserialize (deserializer) ? ; Ok (DragonsLobbyGetApplicationSettingsParams) } } # [derive (Debug , Clone , Serialize)] pub struct DragonsLobbyGetApplicationSettingsParams ; impl Schema for DragonsLobbyGetApplicationSettingsParams { fn schema () -> Value { serde_json :: from_str ("{}") . unwrap () } } impl Agent for DragonsLobbyGetApplicationSettingsParams { fn topic () -> & 'static str { "dragons_lobby_getApplicationSettings" } fn method () -> & 'static str { "lobby_getApplicationSettings" } fn agent () -> & 'static str { "dragons" } fn access_level () -> AccessLevel { } } impl < 'de > Deserialize < 'de > for DragonsLobbyGetApplicationSettingsReturns { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { serde_json :: Value :: deserialize (deserializer) ? ; Ok (DragonsLobbyGetApplicationSettingsReturns) } } # [derive (Debug , Clone , Serialize)] pub struct DragonsLobbyGetApplicationSettingsReturns ; impl Schema for DragonsLobbyGetApplicationSettingsReturns { fn schema () -> Value { serde_json :: json ! ("{}") } } impl Agent for DragonsLobbyGetApplicationSettingsReturns { fn topic () -> & 'static str { "dragons_lobby_getApplicationSettings" } fn method () -> & 'static str { "lobby_getApplicationSettings" } fn agent () -> & 'static str { "dragons" } fn access_level () -> AccessLevel { } }