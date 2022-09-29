// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthAuthDeviceGetClientInfoParams {
    #[serde(rename = "providerId")]
    pub provider_id: String,
    #[serde(rename = "provider")]
    pub provider: String,
}
impl Schema for AuthAuthDeviceGetClientInfoParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"providerId\":{\"type\":\"string\"},\"provider\":{\"type\":\"string\",\"pattern\":\"^(Keycloak)|(DeviceId)$\"}},\"required\":[\"providerId\",\"provider\"]}") . unwrap ()
    }
}
impl Agent for AuthAuthDeviceGetClientInfoParams {
    fn topic() -> &'static str {
        "auth_authDevice_getClientInfo"
    }
    fn method() -> &'static str {
        "authDevice_getClientInfo"
    }
    fn agent() -> &'static str {
        "auth"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthAuthDeviceGetClientInfoReturns {
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "isAuth")]
    pub is_auth: bool,
    #[serde(rename = "email")]
    pub email: String,
}
impl Schema for AuthAuthDeviceGetClientInfoReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"userId\":{\"type\":\"string\"},\"isAuth\":{\"type\":\"boolean\"},\"email\":{\"type\":\"string\"}},\"required\":[\"userId\",\"email\",\"isAuth\"]}")
    }
}
impl Agent for AuthAuthDeviceGetClientInfoReturns {
    fn topic() -> &'static str {
        "auth_authDevice_getClientInfo"
    }
    fn method() -> &'static str {
        "authDevice_getClientInfo"
    }
    fn agent() -> &'static str {
        "auth"
    }
}
