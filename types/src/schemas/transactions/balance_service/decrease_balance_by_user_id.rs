use crate::requests::AuthInfo;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct DecreaseBalanceByUserIdSchema {
    pub id: String,
    #[serde(rename = "topicResponse", alias = "topicRes")]
    pub topic_res: String,
    pub params: DecreaseBalanceByUserIdSchemaParams,
    #[serde(rename = "authInfo")]
    pub auth_info: AuthInfo,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct DecreaseBalanceByUserIdSchemaParams {
    #[serde(rename = "extraDetails")]
    pub extra_details: Option<Value>, // TODO as in TS schemas
    pub creator: String,
    pub reason: String,
    pub currency: String, // TODO some details as in TS
    pub amount: String,
    #[serde(rename = "txId")]
    pub tx_id: String,
    #[serde(rename = "userId")]
    pub user_id: String,
}

#[cfg(test)]
mod tests {
    use crate::schemas::transactions::balance_service::decrease_balance_by_user_id::DecreaseBalanceByUserIdSchema;
    use serde_json::{json, Value};

    #[test]
    fn test_deserialize_option_field() {
        let a: Option<Value> = None;
        let test = json!({
        "extraDetails": a,
        "creator": "1",
        "reason": "2",
        "currency": "3",
        "amount": "4",
        "txId": "5",
        "userId": "6"
        });

        let schema = serde_json::from_value::<DecreaseBalanceByUserIdSchema>(test).unwrap();
        assert!(schema.extra_details.is_none())
    }

    #[test]
    fn test_deserialize_without_option_field() {
        let test = json!({
        "creator": "1",
        "reason": "2",
        "currency": "3",
        "amount": "4",
        "txId": "5",
        "userId": "6"
        });

        let schema = serde_json::from_value::<DecreaseBalanceByUserIdSchema>(test).unwrap();
        assert!(schema.extra_details.is_none())
    }
}
