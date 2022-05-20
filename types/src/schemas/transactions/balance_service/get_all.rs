use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct GetAllSchema {
    #[serde(rename = "userId")]
    pub user_id: String,
    pub page: u32,
    #[serde(rename = "perPage")]
    pub per_page: u32,
}
