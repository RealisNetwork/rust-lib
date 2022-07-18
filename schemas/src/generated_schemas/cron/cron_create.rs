// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct CronCronCreateParams {
    #[serde(rename = "key")]
    pub key: String,
    #[serde(rename = "howOften")]
    pub how_often: i64,
    #[serde(rename = "startsAt")]
    pub starts_at: i64,
}
pub type CronCronCreateReturns = bool;
