// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationsNotificationsCreateNewParams {
    #[serde(rename = "senderUserId")]
    pub sender_user_id: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "notificationType")]
    pub notification_type: (),
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "category")]
    pub category: String,
}
impl Schema for NotificationsNotificationsCreateNewParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"senderUserId\":{\"type\":\"string\"},\"description\":{\"type\":\"string\"},\"userId\":{\"type\":\"string\"},\"notificationType\":{},\"title\":{\"type\":\"string\"},\"category\":{\"type\":\"string\"}},\"required\":[\"notificationType\",\"userId\",\"category\",\"description\",\"senderUserId\",\"title\"]}") . unwrap ()
    }
}
impl Agent for NotificationsNotificationsCreateNewParams {
    fn topic() -> &'static str {
        "notifications_notifications_createNew"
    }
    fn method() -> &'static str {
        "notifications_createNew"
    }
    fn agent() -> &'static str {
        "notifications"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationsNotificationsCreateNewReturns(pub bool);
impl Schema for NotificationsNotificationsCreateNewReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"boolean\"}")
    }
}
impl Agent for NotificationsNotificationsCreateNewReturns {
    fn topic() -> &'static str {
        "notifications_notifications_createNew"
    }
    fn method() -> &'static str {
        "notifications_createNew"
    }
    fn agent() -> &'static str {
        "notifications"
    }
}
