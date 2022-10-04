// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusMembershipGetAllActiveParams {
    #[serde(rename = "userId")]
    pub user_id: String,
}
impl Schema for StatusMembershipGetAllActiveParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"userId\":{\"type\":\"string\"}},\"required\":[\"userId\"]}") . unwrap ()
    }
}
impl Agent for StatusMembershipGetAllActiveParams {
    fn topic() -> &'static str {
        "status_membership_getAllActive"
    }
    fn method() -> &'static str {
        "membership_getAllActive"
    }
    fn agent() -> &'static str {
        "status"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusMembershipGetAllActiveReturnsParamsMembershipParams {}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusMembershipGetAllActiveReturnsParams {
    #[serde(rename = "isActive")]
    pub is_active: bool,
    #[serde(rename = "endDate")]
    pub end_date: String,
    #[serde(rename = "duration")]
    pub duration: String,
    #[serde(rename = "userId")]
    pub user_id: f64,
    #[serde(rename = "subscriptionProductId")]
    pub subscription_product_id: String,
    #[serde(rename = "subscriptionToken")]
    pub subscription_token: String,
    #[serde(rename = "subscriptionTokenHash")]
    pub subscription_token_hash: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    #[serde(rename = "membership")]
    pub membership: StatusMembershipGetAllActiveReturnsParamsMembershipParams,
    #[serde(rename = "subscriptionOrderId")]
    pub subscription_order_id: String,
    #[serde(rename = "appId")]
    pub app_id: f64,
    #[serde(rename = "id")]
    pub id: f64,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusMembershipGetAllActiveReturns(pub Vec<StatusMembershipGetAllActiveReturnsParams>);
impl Schema for StatusMembershipGetAllActiveReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"isActive\":{\"type\":\"boolean\"},\"endDate\":{\"type\":\"string\"},\"duration\":{\"type\":\"string\"},\"userId\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"subscriptionProductId\":{\"type\":\"string\"},\"subscriptionToken\":{\"type\":\"string\"},\"subscriptionTokenHash\":{\"type\":\"string\"},\"createdAt\":{\"type\":\"string\"},\"updatedAt\":{\"type\":\"string\"},\"membership\":{\"type\":\"object\",\"properties\":{}},\"subscriptionOrderId\":{\"type\":\"string\"},\"appId\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"id\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"id\",\"userId\",\"appId\",\"isActive\",\"endDate\",\"duration\",\"subscriptionProductId\",\"subscriptionOrderId\",\"subscriptionToken\",\"subscriptionTokenHash\",\"createdAt\",\"updatedAt\",\"membership\"]}}")
    }
}
impl Agent for StatusMembershipGetAllActiveReturns {
    fn topic() -> &'static str {
        "status_membership_getAllActive"
    }
    fn method() -> &'static str {
        "membership_getAllActive"
    }
    fn agent() -> &'static str {
        "status"
    }
}
