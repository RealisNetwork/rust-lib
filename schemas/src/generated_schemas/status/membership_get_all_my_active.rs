// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
impl<'de> Deserialize<'de> for StatusMembershipGetAllMyActiveParams {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde_json::Value::deserialize(deserializer)?;
        Ok(StatusMembershipGetAllMyActiveParams)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct StatusMembershipGetAllMyActiveParams;
impl Schema for StatusMembershipGetAllMyActiveParams {
    fn schema() -> Value {
        serde_json::from_str("{}").unwrap()
    }
}
impl Agent for StatusMembershipGetAllMyActiveParams {
    fn topic() -> &'static str {
        "status_membership_getAllMyActive"
    }
    fn method() -> &'static str {
        "membership_getAllMyActive"
    }
    fn agent() -> &'static str {
        "status"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Protected
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusMembershipGetAllMyActiveReturnsParamsMembershipParams(Value);
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusMembershipGetAllMyActiveReturnsParams {
    #[serde(rename = "id")]
    pub id: f64,
    #[serde(rename = "appId")]
    pub app_id: f64,
    #[serde(rename = "userId")]
    pub user_id: f64,
    #[serde(rename = "subscriptionProductId")]
    pub subscription_product_id: String,
    #[serde(rename = "subscriptionTokenHash")]
    pub subscription_token_hash: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "subscriptionOrderId")]
    pub subscription_order_id: String,
    #[serde(rename = "duration")]
    pub duration: String,
    #[serde(rename = "membership")]
    pub membership: StatusMembershipGetAllMyActiveReturnsParamsMembershipParams,
    #[serde(rename = "subscriptionToken")]
    pub subscription_token: String,
    #[serde(rename = "isActive")]
    pub is_active: bool,
    #[serde(rename = "endDate")]
    pub end_date: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusMembershipGetAllMyActiveReturns(
    pub Vec<StatusMembershipGetAllMyActiveReturnsParams>,
);
impl Schema for StatusMembershipGetAllMyActiveReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"id\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"appId\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"userId\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"subscriptionProductId\":{\"type\":\"string\"},\"subscriptionTokenHash\":{\"type\":\"string\"},\"updatedAt\":{\"type\":\"string\"},\"createdAt\":{\"type\":\"string\"},\"subscriptionOrderId\":{\"type\":\"string\"},\"duration\":{\"type\":\"string\"},\"membership\":{\"type\":\"object\",\"properties\":{}},\"subscriptionToken\":{\"type\":\"string\"},\"isActive\":{\"type\":\"boolean\"},\"endDate\":{\"type\":\"string\"}},\"required\":[\"id\",\"userId\",\"appId\",\"isActive\",\"endDate\",\"duration\",\"subscriptionProductId\",\"subscriptionOrderId\",\"subscriptionToken\",\"subscriptionTokenHash\",\"createdAt\",\"updatedAt\",\"membership\"]}}")
    }
}
impl Agent for StatusMembershipGetAllMyActiveReturns {
    fn topic() -> &'static str {
        "status_membership_getAllMyActive"
    }
    fn method() -> &'static str {
        "membership_getAllMyActive"
    }
    fn agent() -> &'static str {
        "status"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Protected
    }
}
