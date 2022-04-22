use crate::requests::Response;
use serde_json::Value;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdapterResponseSchema {
    #[serde(flatten)]
    pub result: Response<OldRequest, Value>

}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OldRequest {
    pub id: String,
}