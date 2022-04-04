use runtime::AccountId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetUserNftListWithOffset {
    #[serde(rename = "accountId")]
    account_id: AccountId,
    id: String,
    params: Params,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Params {
    size: usize,
    offset: usize,
}
