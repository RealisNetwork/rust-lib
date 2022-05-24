use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct GetBalanceByUserIdSchema {
    pub currency: String,
}
