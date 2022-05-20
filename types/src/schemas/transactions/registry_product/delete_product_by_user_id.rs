use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct DeleteProductByUserIdSchema {
    #[serde(rename = "userId")]
    pub user_id: String,
}
