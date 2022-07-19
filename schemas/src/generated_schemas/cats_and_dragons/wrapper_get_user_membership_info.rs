// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatsAndDragonsWrapperGetUserMembershipInfoParams {
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "appId")]
    pub app_id: i64,
}
impl Schema for CatsAndDragonsWrapperGetUserMembershipInfoParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"userId\":{\"type\":\"string\"},\"appId\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"userId\",\"appId\"]}")
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatsAndDragonsWrapperGetUserMembershipInfoReturns {
    #[serde(rename = "membershipId")]
    pub membership_id: i64,
    #[serde(rename = "statusPurchaseAppId")]
    pub status_purchase_app_id: i64,
    #[serde(rename = "balance")]
    pub balance: String,
    #[serde(rename = "isAuthorized")]
    pub is_authorized: bool,
    #[serde(rename = "multiplier")]
    pub multiplier: i64,
}
impl Schema for CatsAndDragonsWrapperGetUserMembershipInfoReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"membershipId\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"statusPurchaseAppId\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"balance\":{\"type\":\"string\"},\"isAuthorized\":{\"type\":\"boolean\"},\"multiplier\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"statusPurchaseAppId\",\"membershipId\",\"multiplier\",\"balance\",\"isAuthorized\"]}")
    }
}
