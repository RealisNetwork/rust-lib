// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct ProductFactoryProductTypeGetAllParams {
    #[serde(rename = "page")]
    pub page: i64,
    #[serde(rename = "perPage")]
    pub per_page: i64,
}
