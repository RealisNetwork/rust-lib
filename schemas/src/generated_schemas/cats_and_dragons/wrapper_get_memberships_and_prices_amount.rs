// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
impl<'de> Deserialize<'de> for CatsAndDragonsWrapperGetMembershipsAndPricesAmountParams {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde_json::Value::deserialize(deserializer)?;
        Ok(CatsAndDragonsWrapperGetMembershipsAndPricesAmountParams)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct CatsAndDragonsWrapperGetMembershipsAndPricesAmountParams;
impl Schema for CatsAndDragonsWrapperGetMembershipsAndPricesAmountParams {
    fn schema() -> Value {
        serde_json::from_str("{}").unwrap()
    }
}
impl Agent for CatsAndDragonsWrapperGetMembershipsAndPricesAmountParams {
    fn topic() -> &'static str {
        "catsAndDragons_wrapper_getMembershipsAndPricesAmount"
    }
    fn method() -> &'static str {
        "wrapper_getMembershipsAndPricesAmount"
    }
    fn agent() -> &'static str {
        "catsAndDragons"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Public
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatsAndDragonsWrapperGetMembershipsAndPricesAmountReturnsMembershipsInfoParamsPriceParams
{
    #[serde(rename = "lisUsd")]
    pub lis_usd: f64,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatsAndDragonsWrapperGetMembershipsAndPricesAmountReturnsMembershipsInfoParamsMembershipsParams
{
    #[serde(rename = "gold")]
    pub gold: f64,
    #[serde(rename = "silver")]
    pub silver: f64,
    #[serde(rename = "platinum")]
    pub platinum: f64,
    #[serde(rename = "standard")]
    pub standard: f64,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatsAndDragonsWrapperGetMembershipsAndPricesAmountReturnsMembershipsInfoParams { # [serde (rename = "Price")] pub price : CatsAndDragonsWrapperGetMembershipsAndPricesAmountReturnsMembershipsInfoParamsPriceParams , # [serde (rename = "Memberships")] pub memberships : CatsAndDragonsWrapperGetMembershipsAndPricesAmountReturnsMembershipsInfoParamsMembershipsParams }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatsAndDragonsWrapperGetMembershipsAndPricesAmountReturns {
    #[serde(rename = "membershipsInfo")]
    pub memberships_info:
        CatsAndDragonsWrapperGetMembershipsAndPricesAmountReturnsMembershipsInfoParams,
}
impl Schema for CatsAndDragonsWrapperGetMembershipsAndPricesAmountReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"membershipsInfo\":{\"type\":\"object\",\"properties\":{\"Price\":{\"type\":\"object\",\"properties\":{\"lisUsd\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"lisUsd\"]},\"Memberships\":{\"type\":\"object\",\"properties\":{\"gold\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"silver\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"platinum\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"standard\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"standard\",\"silver\",\"gold\",\"platinum\"]}},\"required\":[\"Memberships\",\"Price\"]}},\"required\":[\"membershipsInfo\"]}")
    }
}
impl Agent for CatsAndDragonsWrapperGetMembershipsAndPricesAmountReturns {
    fn topic() -> &'static str {
        "catsAndDragons_wrapper_getMembershipsAndPricesAmount"
    }
    fn method() -> &'static str {
        "wrapper_getMembershipsAndPricesAmount"
    }
    fn agent() -> &'static str {
        "catsAndDragons"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Public
    }
}
