// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
impl<'de> Deserialize<'de> for AuthTwoFactorCanGenerateSecretParams {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(AuthTwoFactorCanGenerateSecretParams)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct AuthTwoFactorCanGenerateSecretParams;
impl Schema for AuthTwoFactorCanGenerateSecretParams {
    fn schema() -> Value {
        serde_json::json!("{}")
    }
}
impl Agent for AuthTwoFactorCanGenerateSecretParams {
    fn topic() -> &'static str {
        "auth_twoFactor_canGenerateSecret"
    }
    fn method() -> &'static str {
        "twoFactor_canGenerateSecret"
    }
    fn agent() -> &'static str {
        "auth"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthTwoFactorCanGenerateSecretReturns(bool);
impl Schema for AuthTwoFactorCanGenerateSecretReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"boolean\"}")
    }
}
impl Agent for AuthTwoFactorCanGenerateSecretReturns {
    fn topic() -> &'static str {
        "auth_twoFactor_canGenerateSecret"
    }
    fn method() -> &'static str {
        "twoFactor_canGenerateSecret"
    }
    fn agent() -> &'static str {
        "auth"
    }
}
