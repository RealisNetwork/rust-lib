// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct DragocatsStorageUnitEndpointsGetByUnitIdParams {
    #[serde(rename = "unitId")]
    pub unit_id: i32,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DragocatsStorageUnitEndpointsGetByUnitIdReturnsPartsParamsParamsAttributesCoefficientsParams
{
    #[serde(rename = "power")]
    pub power: (),
    #[serde(rename = "speed")]
    pub speed: (),
    #[serde(rename = "defence")]
    pub defence: (),
    #[serde(rename = "stamina")]
    pub stamina: (),
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DragocatsStorageUnitEndpointsGetByUnitIdReturnsPartsParamsParams { # [serde (rename = "attributesCoefficients")] pub attributes_coefficients : DragocatsStorageUnitEndpointsGetByUnitIdReturnsPartsParamsParamsAttributesCoefficientsParams , # [serde (rename = "slotId")] pub slot_id : i32 , # [serde (rename = "element")] pub element : String , # [serde (rename = "typeId")] pub type_id : i32 }
#[derive(Debug, Serialize, Deserialize)]
pub struct DragocatsStorageUnitEndpointsGetByUnitIdReturns {
    #[serde(rename = "maxLevel")]
    pub max_level: i32,
    #[serde(rename = "rarity")]
    pub rarity: i32,
    #[serde(rename = "parts")]
    pub parts: Vec<DragocatsStorageUnitEndpointsGetByUnitIdReturnsPartsParamsParams>,
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "unitId")]
    pub unit_id: i32,
    #[serde(rename = "status")]
    pub status: i32,
    #[serde(rename = "level")]
    pub level: i32,
    #[serde(rename = "experienceForLevelup")]
    pub experience_for_levelup: i32,
}
