// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; # [derive (Debug , Clone , Serialize , Deserialize)] pub struct DragocatsLobbyUnitEndpointsGetEquipedUnitsParams { # [serde (rename = "userId")] pub user_id : String } impl Schema for DragocatsLobbyUnitEndpointsGetEquipedUnitsParams { fn schema () -> Value { serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"userId\":{\"type\":\"string\"}},\"required\":[\"userId\"]}") . unwrap () } } impl Agent for DragocatsLobbyUnitEndpointsGetEquipedUnitsParams { fn topic () -> & 'static str { "dragocats-lobby_unitEndpoints_getEquipedUnits" } fn method () -> & 'static str { "unitEndpoints_getEquipedUnits" } fn agent () -> & 'static str { "dragocats-lobby" } fn access_level () -> AccessLevel { } } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct DragocatsLobbyUnitEndpointsGetEquipedUnitsReturnsParams { # [serde (rename = "slotId")] pub slot_id : f64 , # [serde (rename = "unitId")] pub unit_id : f64 } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct DragocatsLobbyUnitEndpointsGetEquipedUnitsReturns (pub Vec < DragocatsLobbyUnitEndpointsGetEquipedUnitsReturnsParams >) ; impl Schema for DragocatsLobbyUnitEndpointsGetEquipedUnitsReturns { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"slotId\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"unitId\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"unitId\",\"slotId\"]}}") } } impl Agent for DragocatsLobbyUnitEndpointsGetEquipedUnitsReturns { fn topic () -> & 'static str { "dragocats-lobby_unitEndpoints_getEquipedUnits" } fn method () -> & 'static str { "unitEndpoints_getEquipedUnits" } fn agent () -> & 'static str { "dragocats-lobby" } fn access_level () -> AccessLevel { } }