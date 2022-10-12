// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
impl<'de> Deserialize<'de> for DragocatsStorageInventoryEndpointsGetUserInventoryParams {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde_json::Value::deserialize(deserializer)?;
        Ok(DragocatsStorageInventoryEndpointsGetUserInventoryParams)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct DragocatsStorageInventoryEndpointsGetUserInventoryParams;
impl Schema for DragocatsStorageInventoryEndpointsGetUserInventoryParams {
    fn schema() -> Value {
        serde_json::from_str("{}").unwrap()
    }
}
impl Agent for DragocatsStorageInventoryEndpointsGetUserInventoryParams {
    fn topic() -> &'static str {
        "dragocats-storage_inventoryEndpoints_getUserInventory"
    }
    fn method() -> &'static str {
        "inventoryEndpoints_getUserInventory"
    }
    fn agent() -> &'static str {
        "dragocats-storage"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Protected
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DragocatsStorageInventoryEndpointsGetUserInventoryReturnsUnitsParamsParamsPartsParams {
    #[serde(rename = "head")]
    pub head: i32,
    #[serde(rename = "tail")]
    pub tail: i32,
    #[serde(rename = "body")]
    pub body: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DragocatsStorageInventoryEndpointsGetUserInventoryReturnsUnitsParamsParamsUnitCharacteristicDtoParamsParams
{
    #[serde(rename = "value")]
    pub value: i32,
    #[serde(rename = "unitStat")]
    pub unit_stat: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DragocatsStorageInventoryEndpointsGetUserInventoryReturnsUnitsParamsParams { # [serde (rename = "maxLevel")] pub max_level : i32 , # [serde (rename = "experienceForLevelup")] pub experience_for_levelup : i32 , # [serde (rename = "level")] pub level : i32 , # [serde (rename = "experience")] pub experience : i32 , # [serde (rename = "unitType")] pub unit_type : String , # [serde (rename = "parts")] pub parts : DragocatsStorageInventoryEndpointsGetUserInventoryReturnsUnitsParamsParamsPartsParams , # [serde (rename = "unitCharacteristicDto")] pub unit_characteristic_dto : Vec < DragocatsStorageInventoryEndpointsGetUserInventoryReturnsUnitsParamsParamsUnitCharacteristicDtoParamsParams > , # [serde (rename = "combatPower")] pub combat_power : i32 , # [serde (rename = "bindingId")] pub binding_id : i32 , # [serde (rename = "rarity")] pub rarity : i32 , # [serde (rename = "isNft")] pub is_nft : bool , # [serde (rename = "status")] pub status : i32 }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DragocatsStorageInventoryEndpointsGetUserInventoryReturnsLootboxesParamsParams {
    #[serde(rename = "bindingId")]
    pub binding_id: i32,
    #[serde(rename = "lootboxId")]
    pub lootbox_id: i32,
    #[serde(rename = "status")]
    pub status: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DragocatsStorageInventoryEndpointsGetUserInventoryReturns {
    #[serde(rename = "units")]
    pub units: Vec<DragocatsStorageInventoryEndpointsGetUserInventoryReturnsUnitsParamsParams>,
    #[serde(rename = "lootboxes")]
    pub lootboxes:
        Vec<DragocatsStorageInventoryEndpointsGetUserInventoryReturnsLootboxesParamsParams>,
}
impl Schema for DragocatsStorageInventoryEndpointsGetUserInventoryReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"units\":{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"maxLevel\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"experienceForLevelup\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"level\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"experience\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"unitType\":{\"type\":\"string\"},\"parts\":{\"type\":\"object\",\"properties\":{\"head\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"tail\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"body\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}}},\"required\":[\"head\",\"body\",\"tail\"]},\"unitCharacteristicDto\":{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"value\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"unitStat\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}}},\"required\":[\"unitStat\",\"value\"]}},\"combatPower\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"bindingId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"rarity\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"isNft\":{\"type\":\"boolean\"},\"status\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}}},\"required\":[\"bindingId\",\"isNft\",\"unitType\",\"rarity\",\"parts\",\"unitCharacteristicDto\",\"combatPower\",\"level\",\"maxLevel\",\"experience\",\"experienceForLevelup\",\"status\"]}},\"lootboxes\":{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"bindingId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"lootboxId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"status\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}}},\"required\":[\"bindingId\",\"lootboxId\",\"status\"]}}},\"required\":[\"lootboxes\",\"units\"]}")
    }
}
impl Agent for DragocatsStorageInventoryEndpointsGetUserInventoryReturns {
    fn topic() -> &'static str {
        "dragocats-storage_inventoryEndpoints_getUserInventory"
    }
    fn method() -> &'static str {
        "inventoryEndpoints_getUserInventory"
    }
    fn agent() -> &'static str {
        "dragocats-storage"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Protected
    }
}
