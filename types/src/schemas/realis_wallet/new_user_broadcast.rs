use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewUserBroadcastSchema {
    pub id: String,
    #[serde(rename = "userId")]
    pub user_id: String,
}