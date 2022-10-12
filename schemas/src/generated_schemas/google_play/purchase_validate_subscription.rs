// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GooglePlayPurchaseValidateSubscriptionParams {
    #[serde(rename = "subscriptionToken")]
    pub subscription_token: String,
    #[serde(rename = "subscriptionProductId")]
    pub subscription_product_id: String,
    #[serde(rename = "packageName")]
    pub package_name: String,
}
impl Schema for GooglePlayPurchaseValidateSubscriptionParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"subscriptionToken\":{\"type\":\"string\"},\"subscriptionProductId\":{\"type\":\"string\"},\"packageName\":{\"type\":\"string\"}},\"required\":[\"subscriptionToken\",\"subscriptionProductId\",\"packageName\"]}") . unwrap ()
    }
}
impl Agent for GooglePlayPurchaseValidateSubscriptionParams {
    fn topic() -> &'static str {
        "google-play_purchase_validateSubscription"
    }
    fn method() -> &'static str {
        "purchase_validateSubscription"
    }
    fn agent() -> &'static str {
        "google-play"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Internal
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GooglePlayPurchaseValidateSubscriptionReturns {
    #[serde(rename = "expiryTimeMillis")]
    pub expiry_time_millis: Option<String>,
    #[serde(rename = "isValid")]
    pub is_valid: bool,
}
impl Schema for GooglePlayPurchaseValidateSubscriptionReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"expiryTimeMillis\":{\"type\":\"string\"},\"isValid\":{\"type\":\"boolean\"}},\"required\":[\"isValid\"]}")
    }
}
impl Agent for GooglePlayPurchaseValidateSubscriptionReturns {
    fn topic() -> &'static str {
        "google-play_purchase_validateSubscription"
    }
    fn method() -> &'static str {
        "purchase_validateSubscription"
    }
    fn agent() -> &'static str {
        "google-play"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Internal
    }
}
