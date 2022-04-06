use runtime::AccountId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetUserNftListSchema {
    #[serde(rename = "accountId")]
    account_id: AccountId,
    id: String,
}
