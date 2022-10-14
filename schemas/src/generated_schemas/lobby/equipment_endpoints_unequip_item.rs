// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; # [derive (Debug , Clone , Serialize , Deserialize)] pub struct LobbyEquipmentEndpointsUnequipItemParams { # [serde (rename = "bindingId")] pub binding_id : i32 , # [serde (rename = "heroBindingId")] pub hero_binding_id : i32 } impl Schema for LobbyEquipmentEndpointsUnequipItemParams { fn schema () -> Value { serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"bindingId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"heroBindingId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}}},\"required\":[\"bindingId\",\"heroBindingId\"]}") . unwrap () } } impl Agent for LobbyEquipmentEndpointsUnequipItemParams { fn topic () -> & 'static str { "lobby_equipmentEndpoints_unequipItem" } fn method () -> & 'static str { "equipmentEndpoints_unequipItem" } fn agent () -> & 'static str { "lobby" } fn access_level () -> AccessLevel { AccessLevel :: Protected } } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct LobbyEquipmentEndpointsUnequipItemReturns { # [serde (rename = "slotId")] pub slot_id : i32 , # [serde (rename = "heroBindingId")] pub hero_binding_id : i32 , # [serde (rename = "bindingId")] pub binding_id : i32 } impl Schema for LobbyEquipmentEndpointsUnequipItemReturns { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"slotId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"heroBindingId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"bindingId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}}},\"required\":[\"bindingId\",\"heroBindingId\",\"slotId\"]}") } } impl Agent for LobbyEquipmentEndpointsUnequipItemReturns { fn topic () -> & 'static str { "lobby_equipmentEndpoints_unequipItem" } fn method () -> & 'static str { "equipmentEndpoints_unequipItem" } fn agent () -> & 'static str { "lobby" } fn access_level () -> AccessLevel { AccessLevel :: Protected } }