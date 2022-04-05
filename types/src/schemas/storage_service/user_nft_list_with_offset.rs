use runtime::AccountId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetUserNftListWithOffsetSchema {
    #[serde(rename = "accountId")]
    account_id: AccountId,
    id: String,
    params: GetUserNftListWithOffsetSchemaParams,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetUserNftListWithOffsetSchemaParams {
    size: usize,
    offset: usize,
}
