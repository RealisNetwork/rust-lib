// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct CronCronUpdateParams {
    #[serde(rename = "howOften")]
    pub how_often: i64,
    #[serde(rename = "startsAt")]
    pub starts_at: i64,
    #[serde(rename = "id")]
    pub id: i64,
}
pub type CronCronUpdateReturns = bool;
