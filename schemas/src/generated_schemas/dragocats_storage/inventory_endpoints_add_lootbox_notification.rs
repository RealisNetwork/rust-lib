// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; impl < 'de > Deserialize < 'de > for DragocatsStorageInventoryEndpointsAddLootboxNotificationParams { fn deserialize < D > (_deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { Ok (DragocatsStorageInventoryEndpointsAddLootboxNotificationParams) } } # [derive (Debug , Clone , Serialize)] pub struct DragocatsStorageInventoryEndpointsAddLootboxNotificationParams ; impl Schema for DragocatsStorageInventoryEndpointsAddLootboxNotificationParams { fn schema () -> Value { serde_json :: json ! ("{}") } } impl Agent for DragocatsStorageInventoryEndpointsAddLootboxNotificationParams { fn topic () -> & 'static str { "dragocats-storage_inventoryEndpoints_addLootboxNotification" } fn method () -> & 'static str { "inventoryEndpoints_addLootboxNotification" } fn agent () -> & 'static str { "dragocats-storage" } } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct DragocatsStorageInventoryEndpointsAddLootboxNotificationReturns { # [serde (rename = "status")] pub status : i32 , # [serde (rename = "lootboxId")] pub lootbox_id : i32 , # [serde (rename = "bindingId")] pub binding_id : i32 } impl Schema for DragocatsStorageInventoryEndpointsAddLootboxNotificationReturns { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"status\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"lootboxId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"bindingId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}}},\"required\":[\"bindingId\",\"lootboxId\",\"status\"]}") } } impl Agent for DragocatsStorageInventoryEndpointsAddLootboxNotificationReturns { fn topic () -> & 'static str { "dragocats-storage_inventoryEndpoints_addLootboxNotification" } fn method () -> & 'static str { "inventoryEndpoints_addLootboxNotification" } fn agent () -> & 'static str { "dragocats-storage" } }