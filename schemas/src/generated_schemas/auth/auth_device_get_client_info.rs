// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthAuthDeviceGetClientInfoParams {
    #[serde(rename = "providerId")]
    pub provider_id: String,
    #[serde(rename = "provider")]
    pub provider: (),
}
impl Schema for AuthAuthDeviceGetClientInfoParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"providerId\":{\"type\":\"string\"},\"provider\":{}},\"required\":[\"providerId\",\"provider\"]}")
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthAuthDeviceGetClientInfoReturns {
    #[serde(rename = "email")]
    pub email: String,
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "isAuth")]
    pub is_auth: bool,
}
impl Schema for AuthAuthDeviceGetClientInfoReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"email\":{\"type\":\"string\"},\"userId\":{\"type\":\"string\"},\"isAuth\":{\"type\":\"boolean\"}},\"required\":[\"userId\",\"email\",\"isAuth\"]}")
    }
}
