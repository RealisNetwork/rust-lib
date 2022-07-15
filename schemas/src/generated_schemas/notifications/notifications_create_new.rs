// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct NotificationsNotificationsCreateNewParams {
    #[serde(rename = "notificationType")]
    pub notification_type: (),
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "category")]
    pub category: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "senderUserId")]
    pub sender_user_id: String,
}
pub type NotificationsNotificationsCreateNewReturns = bool;
