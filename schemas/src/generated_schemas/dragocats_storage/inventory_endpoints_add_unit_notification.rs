// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
impl<'de> Deserialize<'de> for DragocatsStorageInventoryEndpointsAddUnitNotificationParams {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(DragocatsStorageInventoryEndpointsAddUnitNotificationParams)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct DragocatsStorageInventoryEndpointsAddUnitNotificationParams;
impl Schema for DragocatsStorageInventoryEndpointsAddUnitNotificationParams {
    fn schema() -> Value {
        serde_json::json!("{}")
    }
}
impl Agent for DragocatsStorageInventoryEndpointsAddUnitNotificationParams {
    fn topic() -> &'static str {
        "dragocats-storage_inventoryEndpoints_addUnitNotification"
    }
    fn method() -> &'static str {
        "inventoryEndpoints_addUnitNotification"
    }
    fn agent() -> &'static str {
        "dragocats-storage"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DragocatsStorageInventoryEndpointsAddUnitNotificationReturnsUnitCharacteristicDtoParamsParams
{
    #[serde(rename = "unitStat")]
    pub unit_stat: i32,
    #[serde(rename = "value")]
    pub value: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DragocatsStorageInventoryEndpointsAddUnitNotificationReturnsPartsParams {
    #[serde(rename = "tail")]
    pub tail: i32,
    #[serde(rename = "body")]
    pub body: i32,
    #[serde(rename = "head")]
    pub head: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DragocatsStorageInventoryEndpointsAddUnitNotificationReturns { # [serde (rename = "isNft")] pub is_nft : bool , # [serde (rename = "bindingId")] pub binding_id : i32 , # [serde (rename = "unitType")] pub unit_type : String , # [serde (rename = "combatPower")] pub combat_power : i32 , # [serde (rename = "maxLevel")] pub max_level : i32 , # [serde (rename = "experience")] pub experience : i32 , # [serde (rename = "experienceForLevelup")] pub experience_for_levelup : i32 , # [serde (rename = "rarity")] pub rarity : i32 , # [serde (rename = "status")] pub status : i32 , # [serde (rename = "level")] pub level : i32 , # [serde (rename = "unitCharacteristicDto")] pub unit_characteristic_dto : Vec < DragocatsStorageInventoryEndpointsAddUnitNotificationReturnsUnitCharacteristicDtoParamsParams > , # [serde (rename = "parts")] pub parts : DragocatsStorageInventoryEndpointsAddUnitNotificationReturnsPartsParams }
impl Schema for DragocatsStorageInventoryEndpointsAddUnitNotificationReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"isNft\":{\"type\":\"boolean\"},\"bindingId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"unitType\":{\"type\":\"string\"},\"combatPower\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"maxLevel\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"experience\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"experienceForLevelup\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"rarity\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"status\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"level\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"unitCharacteristicDto\":{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"unitStat\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"value\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}}},\"required\":[\"unitStat\",\"value\"]}},\"parts\":{\"type\":\"object\",\"properties\":{\"tail\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"body\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"head\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}}},\"required\":[\"head\",\"body\",\"tail\"]}},\"required\":[\"bindingId\",\"isNft\",\"unitType\",\"rarity\",\"parts\",\"unitCharacteristicDto\",\"combatPower\",\"level\",\"maxLevel\",\"experience\",\"experienceForLevelup\",\"status\"]}")
    }
}
impl Agent for DragocatsStorageInventoryEndpointsAddUnitNotificationReturns {
    fn topic() -> &'static str {
        "dragocats-storage_inventoryEndpoints_addUnitNotification"
    }
    fn method() -> &'static str {
        "inventoryEndpoints_addUnitNotification"
    }
    fn agent() -> &'static str {
        "dragocats-storage"
    }
}
