// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; impl < 'de > Deserialize < 'de > for LobbyEquipmentEndpointsUpdatedItemNotificationParams { fn deserialize < D > (_deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { Ok (LobbyEquipmentEndpointsUpdatedItemNotificationParams) } } # [derive (Debug , Clone , Serialize)] pub struct LobbyEquipmentEndpointsUpdatedItemNotificationParams ; impl Schema for LobbyEquipmentEndpointsUpdatedItemNotificationParams { fn schema () -> Value { serde_json :: json ! ("{}") } } impl Agent for LobbyEquipmentEndpointsUpdatedItemNotificationParams { fn topic () -> & 'static str { "lobby_equipmentEndpoints_updatedItemNotification" } fn method () -> & 'static str { "equipmentEndpoints_updatedItemNotification" } fn agent () -> & 'static str { "lobby" } } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct LobbyEquipmentEndpointsUpdatedItemNotificationReturnsEffectsParamsParams { # [serde (rename = "power")] pub power : String , # [serde (rename = "statName")] pub stat_name : i8 } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct LobbyEquipmentEndpointsUpdatedItemNotificationReturns { # [serde (rename = "status")] pub status : i32 , # [serde (rename = "isPending")] pub is_pending : bool , # [serde (rename = "linkToExplorer")] pub link_to_explorer : String , # [serde (rename = "equipmentItemId")] pub equipment_item_id : i32 , # [serde (rename = "transactionHash")] pub transaction_hash : Option < String > , # [serde (rename = "level")] pub level : i16 , # [serde (rename = "blockId")] pub block_id : Option < String > , # [serde (rename = "maxLevel")] pub max_level : i16 , # [serde (rename = "toNextLevelScrolls")] pub to_next_level_scrolls : i32 , # [serde (rename = "effects")] pub effects : Vec < LobbyEquipmentEndpointsUpdatedItemNotificationReturnsEffectsParamsParams > , # [serde (rename = "bindingId")] pub binding_id : i32 } impl Schema for LobbyEquipmentEndpointsUpdatedItemNotificationReturns { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"status\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"isPending\":{\"type\":\"boolean\"},\"linkToExplorer\":{\"type\":\"string\"},\"equipmentItemId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"transactionHash\":{\"type\":\"string\"},\"level\":{\"type\":\"integer\",\"minimum\":-32768,\"maximum\":32767,\"additionalAttributes\":{\"numberType\":\"Short\"}},\"blockId\":{\"type\":\"string\"},\"maxLevel\":{\"type\":\"integer\",\"minimum\":-32768,\"maximum\":32767,\"additionalAttributes\":{\"numberType\":\"Short\"}},\"toNextLevelScrolls\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"effects\":{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"power\":{\"type\":\"string\"},\"statName\":{\"type\":\"integer\",\"minimum\":-128,\"maximum\":127,\"additionalAttributes\":{\"numberType\":\"Byte\"}}},\"required\":[\"statName\",\"power\"]}},\"bindingId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}}},\"required\":[\"equipmentItemId\",\"bindingId\",\"linkToExplorer\",\"effects\",\"isPending\",\"level\",\"maxLevel\",\"toNextLevelScrolls\",\"status\"]}") } } impl Agent for LobbyEquipmentEndpointsUpdatedItemNotificationReturns { fn topic () -> & 'static str { "lobby_equipmentEndpoints_updatedItemNotification" } fn method () -> & 'static str { "equipmentEndpoints_updatedItemNotification" } fn agent () -> & 'static str { "lobby" } }