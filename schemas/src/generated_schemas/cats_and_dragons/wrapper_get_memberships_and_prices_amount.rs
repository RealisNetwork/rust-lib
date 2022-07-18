// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::Schema;
use serde::de::Deserializer;
use serde::{Deserialize, Serialize};
use serde_json::Value;
impl<'de> Deserialize<'de> for CatsAndDragonsWrapperGetMembershipsAndPricesAmountParams {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(CatsAndDragonsWrapperGetMembershipsAndPricesAmountParams)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct CatsAndDragonsWrapperGetMembershipsAndPricesAmountParams;
impl Schema for CatsAndDragonsWrapperGetMembershipsAndPricesAmountParams {
    fn schema() -> Value {
        todo!()
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatsAndDragonsWrapperGetMembershipsAndPricesAmountReturnsMembershipsInfoParamsMembershipsParams
{
    #[serde(rename = "silver")]
    pub silver: i64,
    #[serde(rename = "gold")]
    pub gold: i64,
    #[serde(rename = "platinum")]
    pub platinum: i64,
    #[serde(rename = "standard")]
    pub standard: i64,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatsAndDragonsWrapperGetMembershipsAndPricesAmountReturnsMembershipsInfoParamsPriceParams
{
    #[serde(rename = "lisUsd")]
    pub lis_usd: i64,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatsAndDragonsWrapperGetMembershipsAndPricesAmountReturnsMembershipsInfoParams { # [serde (rename = "Memberships")] pub memberships : CatsAndDragonsWrapperGetMembershipsAndPricesAmountReturnsMembershipsInfoParamsMembershipsParams , # [serde (rename = "Price")] pub price : CatsAndDragonsWrapperGetMembershipsAndPricesAmountReturnsMembershipsInfoParamsPriceParams }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatsAndDragonsWrapperGetMembershipsAndPricesAmountReturns {
    #[serde(rename = "membershipsInfo")]
    pub memberships_info:
        CatsAndDragonsWrapperGetMembershipsAndPricesAmountReturnsMembershipsInfoParams,
}
