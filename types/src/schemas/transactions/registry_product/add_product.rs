use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct AddProductSchema {
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "productId")]
    pub product_id: String,
    #[serde(rename = "personalType")]
    pub personal_type: String,
}
