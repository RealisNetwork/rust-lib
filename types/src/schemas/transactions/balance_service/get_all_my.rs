use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct GetAllMySchema {
    pub page: u32,
    #[serde(rename = "perPage")]
    pub per_page: u32,
}
