// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
impl<'de> Deserialize<'de> for AuthTwoFactorGenerateSecretParams {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(AuthTwoFactorGenerateSecretParams)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct AuthTwoFactorGenerateSecretParams;
impl Schema for AuthTwoFactorGenerateSecretParams {
    fn schema() -> Value {
        serde_json::json!("{}")
    }
}
impl Agent for AuthTwoFactorGenerateSecretParams {
    fn topic() -> &'static str {
        "auth_twoFactor_generateSecret"
    }
    fn method() -> &'static str {
        "twoFactor_generateSecret"
    }
    fn agent() -> &'static str {
        "auth"
    }
}
impl<'de> Deserialize<'de> for AuthTwoFactorGenerateSecretReturns {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(AuthTwoFactorGenerateSecretReturns)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct AuthTwoFactorGenerateSecretReturns;
impl Schema for AuthTwoFactorGenerateSecretReturns {
    fn schema() -> Value {
        serde_json::json!("{}")
    }
}
impl Agent for AuthTwoFactorGenerateSecretReturns {
    fn topic() -> &'static str {
        "auth_twoFactor_generateSecret"
    }
    fn method() -> &'static str {
        "twoFactor_generateSecret"
    }
    fn agent() -> &'static str {
        "auth"
    }
}
