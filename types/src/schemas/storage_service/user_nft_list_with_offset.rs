use serde::{Deserialize, Serialize};
use runtime::AccountId;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetUserNftList {
    account_id: AccountId,
    id: String,
    params: Params,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Params {
    size: usize,
    offset: usize,
}