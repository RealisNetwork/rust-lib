// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::Schema;
use serde::de::Deserializer;
use serde::{Deserialize, Serialize};
use serde_json::Value;
impl<'de> Deserialize<'de> for EmailEmailFindAllParams {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(EmailEmailFindAllParams)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct EmailEmailFindAllParams;
impl Schema for EmailEmailFindAllParams {
    fn schema() -> Value {
        todo!()
    }
}
impl<'de> Deserialize<'de> for EmailEmailFindAllReturns {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(EmailEmailFindAllReturns)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct EmailEmailFindAllReturns;
