// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::Schema;
use serde::de::Deserializer;
use serde::{Deserialize, Serialize};
use serde_json::Value;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthAdminAddRoleToUserParams {
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "role")]
    pub role: String,
}
impl Schema for AuthAdminAddRoleToUserParams {
    fn schema() -> Value {
        todo!()
    }
}
pub type AuthAdminAddRoleToUserReturns = bool;
