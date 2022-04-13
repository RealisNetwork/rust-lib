use runtime::AccountId;
use serde::{Deserialize, Serialize};

#[allow(clippy::pedantic)]
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct GetBalanceSchema {
    #[serde(rename = "topicResponse")]
    pub topic_res: String,
    #[serde(rename = "accountId")]
    pub account_id: AccountId,
    pub id: String,
}
