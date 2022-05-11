use crate::schemas::realis_orchestrator::adapter_request::change_batch::ChangeBatch as OrchestratorChangeBatch;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeBatch {
    pub batch_size: usize,
    pub batch_timeout: u64,
}

impl ChangeBatch {
    pub fn new(other: OrchestratorChangeBatch) -> Self {
        Self {
            batch_size: other.batch_size,
            batch_timeout: other.batch_timeout,
        }
    }
}
