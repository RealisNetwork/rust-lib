use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct AddProductHashSchema {
    #[serde(rename = "productId")]
    pub product_id: String,
    #[serde(rename = "hashId")]
    pub hash_id: String,
}
