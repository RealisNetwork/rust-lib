use crate::{requests::Response, schemas::realis_orchestrator::response_message::default::OldRequest};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NftMindNotificationSchema {
    #[serde(flatten)]
    pub result: Response<OldRequest, Value>,
}
