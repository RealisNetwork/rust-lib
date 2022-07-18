// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::Schema;
use serde::de::Deserializer;
use serde::{Deserialize, Serialize};
use serde_json::Value;
impl<'de> Deserialize<'de> for UserProfileGetAllUsersSubscribedToMailingParams {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(UserProfileGetAllUsersSubscribedToMailingParams)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct UserProfileGetAllUsersSubscribedToMailingParams;
impl Schema for UserProfileGetAllUsersSubscribedToMailingParams {
    fn schema() -> Value {
        todo!()
    }
}
pub type UserProfileGetAllUsersSubscribedToMailingReturns = Vec<String>;
