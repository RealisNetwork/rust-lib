use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct BurnProductSchema {
    #[serde(rename = "productId")]
    pub product_id: String,
}
