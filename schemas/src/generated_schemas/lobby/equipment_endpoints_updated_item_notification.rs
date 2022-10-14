// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; impl < 'de > Deserialize < 'de > for LobbyEquipmentEndpointsUpdatedItemNotificationParams { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { serde_json :: Value :: deserialize (deserializer) ? ; Ok (LobbyEquipmentEndpointsUpdatedItemNotificationParams) } } # [derive (Debug , Clone , Serialize)] pub struct LobbyEquipmentEndpointsUpdatedItemNotificationParams ; impl Schema for LobbyEquipmentEndpointsUpdatedItemNotificationParams { fn schema () -> Value { serde_json :: from_str ("{}") . unwrap () } } impl Agent for LobbyEquipmentEndpointsUpdatedItemNotificationParams { fn topic () -> & 'static str { "lobby_equipmentEndpoints_updatedItemNotification" } fn method () -> & 'static str { "equipmentEndpoints_updatedItemNotification" } fn agent () -> & 'static str { "lobby" } fn access_level () -> AccessLevel { AccessLevel :: Protected } } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct LobbyEquipmentEndpointsUpdatedItemNotificationReturnsEffectsParamsParams { # [serde (rename = "power" , deserialize_with = "deserialize_to_string")] pub power : String , # [serde (rename = "statName")] pub stat_name : i8 } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct LobbyEquipmentEndpointsUpdatedItemNotificationReturns { # [serde (rename = "linkToExplorer" , deserialize_with = "deserialize_to_string")] pub link_to_explorer : String , # [serde (rename = "transactionHash")] pub transaction_hash : Option < String > , # [serde (rename = "isPending")] pub is_pending : bool , # [serde (rename = "status")] pub status : i32 , # [serde (rename = "bindingId")] pub binding_id : i32 , # [serde (rename = "equipmentItemId")] pub equipment_item_id : i32 , # [serde (rename = "toNextLevelScrolls")] pub to_next_level_scrolls : i32 , # [serde (rename = "blockId")] pub block_id : Option < String > , # [serde (rename = "level")] pub level : i16 , # [serde (rename = "maxLevel")] pub max_level : i16 , # [serde (rename = "effects")] pub effects : Vec < LobbyEquipmentEndpointsUpdatedItemNotificationReturnsEffectsParamsParams > } impl Schema for LobbyEquipmentEndpointsUpdatedItemNotificationReturns { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"linkToExplorer\":{\"type\":\"string\"},\"transactionHash\":{\"type\":\"string\"},\"isPending\":{\"type\":\"boolean\"},\"status\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"bindingId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"equipmentItemId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"toNextLevelScrolls\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"blockId\":{\"type\":\"string\"},\"level\":{\"type\":\"integer\",\"minimum\":-32768,\"maximum\":32767,\"additionalAttributes\":{\"numberType\":\"Short\"}},\"maxLevel\":{\"type\":\"integer\",\"minimum\":-32768,\"maximum\":32767,\"additionalAttributes\":{\"numberType\":\"Short\"}},\"effects\":{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"power\":{\"type\":\"string\"},\"statName\":{\"type\":\"integer\",\"minimum\":-128,\"maximum\":127,\"additionalAttributes\":{\"numberType\":\"Byte\"}}},\"required\":[\"statName\",\"power\"]}}},\"required\":[\"equipmentItemId\",\"bindingId\",\"linkToExplorer\",\"effects\",\"isPending\",\"level\",\"maxLevel\",\"toNextLevelScrolls\",\"status\"]}") } } impl Agent for LobbyEquipmentEndpointsUpdatedItemNotificationReturns { fn topic () -> & 'static str { "lobby_equipmentEndpoints_updatedItemNotification" } fn method () -> & 'static str { "equipmentEndpoints_updatedItemNotification" } fn agent () -> & 'static str { "lobby" } fn access_level () -> AccessLevel { AccessLevel :: Protected } }