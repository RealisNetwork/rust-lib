use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::requests::Response;
use crate::schemas::realis_orchestrator::response_message::default::OldRequest;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NftMindNotificationSchema {
    pub result: Response<OldRequest, Value>
}

