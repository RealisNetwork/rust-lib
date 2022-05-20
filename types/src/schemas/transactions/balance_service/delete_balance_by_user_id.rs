use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct DeleteBalanceByUserIdSchema {
    #[serde(rename = "userId")]
    pub user_id: String,
}
