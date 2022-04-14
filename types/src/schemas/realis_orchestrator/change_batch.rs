use crate::schemas::realis_adapter::change_batch::ChangeBatch as AdapterChangeBatch;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeBatch {
    pub batch_size: usize,
    pub batch_timeout: u64,
}

impl From<AdapterChangeBatch> for ChangeBatch {
    fn from(other: AdapterChangeBatch) -> Self {
        ChangeBatch {
            batch_size: other.batch_size,
            batch_timeout: other.batch_timeout,
        }
    }
}
