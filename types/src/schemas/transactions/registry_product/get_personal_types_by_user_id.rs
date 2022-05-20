use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct GetPersonalTypesSchema {
    #[serde(rename = "userId")]
    pub user_id: String,
}
