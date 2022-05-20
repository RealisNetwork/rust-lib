use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct GetBalanceByUserIdSchema {
    currency: String,
}
