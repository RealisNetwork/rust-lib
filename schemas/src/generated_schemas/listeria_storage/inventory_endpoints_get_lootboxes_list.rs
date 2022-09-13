// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; impl < 'de > Deserialize < 'de > for ListeriaStorageInventoryEndpointsGetLootboxesListParams { fn deserialize < D > (_deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { Ok (ListeriaStorageInventoryEndpointsGetLootboxesListParams) } } # [derive (Debug , Clone , Serialize)] pub struct ListeriaStorageInventoryEndpointsGetLootboxesListParams ; impl Schema for ListeriaStorageInventoryEndpointsGetLootboxesListParams { fn schema () -> Value { serde_json :: json ! ("{}") } } impl Agent for ListeriaStorageInventoryEndpointsGetLootboxesListParams { fn topic () -> & 'static str { "listeria-storage_inventoryEndpoints_getLootboxesList" } fn method () -> & 'static str { "inventoryEndpoints_getLootboxesList" } fn agent () -> & 'static str { "listeria-storage" } } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct ListeriaStorageInventoryEndpointsGetLootboxesListReturnsLootboxesParamsParams { # [serde (rename = "bindingId")] pub binding_id : i32 , # [serde (rename = "lootboxId")] pub lootbox_id : i32 , # [serde (rename = "status")] pub status : i32 } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct ListeriaStorageInventoryEndpointsGetLootboxesListReturns { # [serde (rename = "lootboxes")] pub lootboxes : Vec < ListeriaStorageInventoryEndpointsGetLootboxesListReturnsLootboxesParamsParams > } impl Schema for ListeriaStorageInventoryEndpointsGetLootboxesListReturns { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"lootboxes\":{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"bindingId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"lootboxId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"status\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}}},\"required\":[\"lootboxId\",\"bindingId\",\"status\"]}}},\"required\":[\"lootboxes\"]}") } } impl Agent for ListeriaStorageInventoryEndpointsGetLootboxesListReturns { fn topic () -> & 'static str { "listeria-storage_inventoryEndpoints_getLootboxesList" } fn method () -> & 'static str { "inventoryEndpoints_getLootboxesList" } fn agent () -> & 'static str { "listeria-storage" } }