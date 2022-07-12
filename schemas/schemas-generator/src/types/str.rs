use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct StringParams {
    pub format: Option<StringFormat>,
    pub pattern: Option<String>,
    pub min_length: Option<u64>,
    pub max_length: Option<u64>,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub enum StringFormat {
    #[serde(rename = "email")]
    Email,
    #[serde(rename = "date")]
    Date,
}
