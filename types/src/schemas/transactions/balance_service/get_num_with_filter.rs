use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct GetNumWithFilterSchema {
    #[serde(rename = "userId")]
    pub user_id: Option<String>,
    pub credit: Option<String>,
    pub debit: Option<String>,
}
