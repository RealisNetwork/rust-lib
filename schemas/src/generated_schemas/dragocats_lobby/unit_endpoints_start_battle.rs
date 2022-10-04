// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; impl < 'de > Deserialize < 'de > for DragocatsLobbyUnitEndpointsStartBattleParams { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { serde_json :: Value :: deserialize (deserializer) ? ; Ok (DragocatsLobbyUnitEndpointsStartBattleParams) } } # [derive (Debug , Clone , Serialize)] pub struct DragocatsLobbyUnitEndpointsStartBattleParams ; impl Schema for DragocatsLobbyUnitEndpointsStartBattleParams { fn schema () -> Value { serde_json :: from_str ("{}") . unwrap () } } impl Agent for DragocatsLobbyUnitEndpointsStartBattleParams { fn topic () -> & 'static str { "dragocats-lobby_unitEndpoints_startBattle" } fn method () -> & 'static str { "unitEndpoints_startBattle" } fn agent () -> & 'static str { "dragocats-lobby" } fn access_level () -> AccessLevel { } } impl < 'de > Deserialize < 'de > for DragocatsLobbyUnitEndpointsStartBattleReturns { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { serde_json :: Value :: deserialize (deserializer) ? ; Ok (DragocatsLobbyUnitEndpointsStartBattleReturns) } } # [derive (Debug , Clone , Serialize)] pub struct DragocatsLobbyUnitEndpointsStartBattleReturns ; impl Schema for DragocatsLobbyUnitEndpointsStartBattleReturns { fn schema () -> Value { serde_json :: json ! ("{}") } } impl Agent for DragocatsLobbyUnitEndpointsStartBattleReturns { fn topic () -> & 'static str { "dragocats-lobby_unitEndpoints_startBattle" } fn method () -> & 'static str { "unitEndpoints_startBattle" } fn agent () -> & 'static str { "dragocats-lobby" } fn access_level () -> AccessLevel { } }