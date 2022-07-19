// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
impl<'de> Deserialize<'de> for DragocatsStorageInventoryEndpointsGetUserInventoryParams {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(DragocatsStorageInventoryEndpointsGetUserInventoryParams)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct DragocatsStorageInventoryEndpointsGetUserInventoryParams;
impl Schema for DragocatsStorageInventoryEndpointsGetUserInventoryParams {
    fn schema() -> Value {
        serde_json::json!("{}")
    }
}
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
pub struct DragocatsStorageInventoryEndpointsGetUserInventoryReturnsUnitsParamsParamsPartsParams {
    #[serde(rename = "body")]
    pub body: i32,
    #[serde(rename = "tail")]
    pub tail: i32,
    #[serde(rename = "head")]
    pub head: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DragocatsStorageInventoryEndpointsGetUserInventoryReturnsUnitsParamsParamsUnitCharacteristicDtoParamsParams
{
    #[serde(rename = "unitStat")]
    pub unit_stat: i32,
    #[serde(rename = "value")]
    pub value: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DragocatsStorageInventoryEndpointsGetUserInventoryReturnsUnitsParamsParams { # [serde (rename = "unitType")] pub unit_type : String , # [serde (rename = "bindingId")] pub binding_id : i32 , # [serde (rename = "parts")] pub parts : DragocatsStorageInventoryEndpointsGetUserInventoryReturnsUnitsParamsParamsPartsParams , # [serde (rename = "combatPower")] pub combat_power : i32 , # [serde (rename = "level")] pub level : i32 , # [serde (rename = "status")] pub status : i32 , # [serde (rename = "experience")] pub experience : i32 , # [serde (rename = "unitCharacteristicDto")] pub unit_characteristic_dto : Vec < DragocatsStorageInventoryEndpointsGetUserInventoryReturnsUnitsParamsParamsUnitCharacteristicDtoParamsParams > , # [serde (rename = "rarity")] pub rarity : i32 , # [serde (rename = "maxLevel")] pub max_level : i32 , # [serde (rename = "experienceForLevelup")] pub experience_for_levelup : i32 , # [serde (rename = "isNft")] pub is_nft : bool }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DragocatsStorageInventoryEndpointsGetUserInventoryReturns {
    #[serde(rename = "lootboxes")]
    pub lootboxes:
        Vec<DragocatsStorageInventoryEndpointsGetUserInventoryReturnsLootboxesParamsParams>,
    #[serde(rename = "units")]
    pub units: Vec<DragocatsStorageInventoryEndpointsGetUserInventoryReturnsUnitsParamsParams>,
}
impl Schema for DragocatsStorageInventoryEndpointsGetUserInventoryReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"lootboxes\":{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"bindingId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"lootboxId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"status\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}}},\"required\":[\"bindingId\",\"lootboxId\",\"status\"]}},\"units\":{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"unitType\":{\"type\":\"string\"},\"bindingId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"parts\":{\"type\":\"object\",\"properties\":{\"body\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"tail\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"head\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}}},\"required\":[\"head\",\"body\",\"tail\"]},\"combatPower\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"level\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"status\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"experience\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"unitCharacteristicDto\":{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"unitStat\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"value\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}}},\"required\":[\"unitStat\",\"value\"]}},\"rarity\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"maxLevel\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"experienceForLevelup\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"isNft\":{\"type\":\"boolean\"}},\"required\":[\"bindingId\",\"isNft\",\"unitType\",\"rarity\",\"parts\",\"unitCharacteristicDto\",\"combatPower\",\"level\",\"maxLevel\",\"experience\",\"experienceForLevelup\",\"status\"]}}},\"required\":[\"lootboxes\",\"units\"]}")
    }
}