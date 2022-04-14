use crate::schemas::realis_orchestrator::change_batch::ChangeBatch as OrchestratorChangeBatch;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeBatch {
    pub batch_size: usize,
    pub batch_timeout: u64,
}

impl ChangeBatch {
    pub fn new(other: OrchestratorChangeBatch) -> Self {
        ChangeBatch {
            batch_size: other.batch_size,
            batch_timeout: other.batch_timeout,
        }
    }
}