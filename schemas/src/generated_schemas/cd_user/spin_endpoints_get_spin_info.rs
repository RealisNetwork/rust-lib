// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
impl<'de> Deserialize<'de> for CdUserSpinEndpointsGetSpinInfoParams {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(CdUserSpinEndpointsGetSpinInfoParams)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct CdUserSpinEndpointsGetSpinInfoParams;
impl Schema for CdUserSpinEndpointsGetSpinInfoParams {
    fn schema() -> Value {
        serde_json::json!("{}")
    }
}
impl Agent for CdUserSpinEndpointsGetSpinInfoParams {
    fn topic() -> &'static str {
        "cd-user_spinEndpoints_getSpinInfo"
    }
    fn method() -> &'static str {
        "spinEndpoints_getSpinInfo"
    }
    fn agent() -> &'static str {
        "cd-user"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CdUserSpinEndpointsGetSpinInfoReturns {
    #[serde(rename = "LastSpinDate")]
    pub last_spin_date: String,
    #[serde(rename = "AdSpinsAmount")]
    pub ad_spins_amount: i32,
}
impl Schema for CdUserSpinEndpointsGetSpinInfoReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"LastSpinDate\":{\"type\":\"string\"},\"AdSpinsAmount\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}}},\"required\":[\"LastSpinDate\",\"AdSpinsAmount\"]}")
    }
}
impl Agent for CdUserSpinEndpointsGetSpinInfoReturns {
    fn topic() -> &'static str {
        "cd-user_spinEndpoints_getSpinInfo"
    }
    fn method() -> &'static str {
        "spinEndpoints_getSpinInfo"
    }
    fn agent() -> &'static str {
        "cd-user"
    }
}
