// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatsAndDragonsWrapperGetNicknameChangePriceParams {
    #[serde(rename = "userId")]
    pub user_id: String,
}
impl Schema for CatsAndDragonsWrapperGetNicknameChangePriceParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"userId\":{\"type\":\"string\"}},\"required\":[\"userId\"]}")
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatsAndDragonsWrapperGetNicknameChangePriceReturns {
    #[serde(rename = "nicknameChangePrice")]
    pub nickname_change_price: String,
}
impl Schema for CatsAndDragonsWrapperGetNicknameChangePriceReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"nicknameChangePrice\":{\"type\":\"string\"}},\"required\":[\"nicknameChangePrice\"]}")
    }
}