// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DragocatsStorageUnitEndpointsGetByUnitIdParams {
    #[serde(rename = "unitId")]
    pub unit_id: i32,
}
impl Schema for DragocatsStorageUnitEndpointsGetByUnitIdParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"unitId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}}},\"required\":[\"unitId\"]}")
    }
}
impl Agent for DragocatsStorageUnitEndpointsGetByUnitIdParams {
    fn topic() -> &'static str {
        "dragocats-storage_unitEndpoints_getByUnitId"
    }
    fn method() -> &'static str {
        "unitEndpoints_getByUnitId"
    }
    fn agent() -> &'static str {
        "dragocats-storage"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DragocatsStorageUnitEndpointsGetByUnitIdReturnsPartsParamsParamsAttributesCoefficientsParams
{
    #[serde(rename = "defence")]
    pub defence: (),
    #[serde(rename = "stamina")]
    pub stamina: (),
    #[serde(rename = "power")]
    pub power: (),
    #[serde(rename = "speed")]
    pub speed: (),
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DragocatsStorageUnitEndpointsGetByUnitIdReturnsPartsParamsParams { # [serde (rename = "typeId")] pub type_id : i32 , # [serde (rename = "element")] pub element : String , # [serde (rename = "slotId")] pub slot_id : i32 , # [serde (rename = "attributesCoefficients")] pub attributes_coefficients : DragocatsStorageUnitEndpointsGetByUnitIdReturnsPartsParamsParamsAttributesCoefficientsParams }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DragocatsStorageUnitEndpointsGetByUnitIdReturns {
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "experienceForLevelup")]
    pub experience_for_levelup: i32,
    #[serde(rename = "level")]
    pub level: i32,
    #[serde(rename = "maxLevel")]
    pub max_level: i32,
    #[serde(rename = "unitId")]
    pub unit_id: i32,
    #[serde(rename = "status")]
    pub status: i32,
    #[serde(rename = "rarity")]
    pub rarity: i32,
    #[serde(rename = "parts")]
    pub parts: Vec<DragocatsStorageUnitEndpointsGetByUnitIdReturnsPartsParamsParams>,
}
impl Schema for DragocatsStorageUnitEndpointsGetByUnitIdReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"userId\":{\"type\":\"string\"},\"experienceForLevelup\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"level\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"maxLevel\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"unitId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"status\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"rarity\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"parts\":{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"typeId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"element\":{\"type\":\"string\"},\"slotId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"attributesCoefficients\":{\"type\":\"object\",\"properties\":{\"defence\":{},\"stamina\":{},\"power\":{},\"speed\":{}},\"required\":[\"stamina\",\"power\",\"speed\",\"defence\"]}},\"required\":[\"slotId\",\"element\",\"typeId\",\"attributesCoefficients\"]}}},\"required\":[\"userId\",\"unitId\",\"level\",\"status\",\"rarity\",\"maxLevel\",\"experienceForLevelup\",\"parts\"]}")
    }
}
impl Agent for DragocatsStorageUnitEndpointsGetByUnitIdReturns {
    fn topic() -> &'static str {
        "dragocats-storage_unitEndpoints_getByUnitId"
    }
    fn method() -> &'static str {
        "unitEndpoints_getByUnitId"
    }
    fn agent() -> &'static str {
        "dragocats-storage"
    }
}
