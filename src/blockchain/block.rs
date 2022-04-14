use reqwest;
use runtime::AccountId;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub extrinsics: Vec<Extrinsic>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Extrinsic {
    pub method: Method,
    pub success: bool,
    #[serde(default)]
    pub signature: Option<Signature>,
    pub events: Vec<Event>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Method {
    pub pallet: String,
    pub method: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Signature {
    pub signer: Signer,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Signer {
    pub id: AccountId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub method: Method,
    pub data: Value,
}

#[derive(Debug)]
pub enum ContainsError {
    ExtrinsicNotFound,
    EventNotFound,
}

impl Block {
    /// # Errors
    pub async fn get_block(host: &str, port: &str, number: &str) -> Result<Block, ()> {
        let request = format!("http://{}:{}/blocks/{}", host, port, number);

        reqwest::get(request).await.map_err(|_| ())?.json().await.map_err(|_| ())
    }

    /// # Panics
    /// # Errors
    pub fn contains_event(&self, method: &Method, event_name: &str, account_id: &AccountId) -> Result<(), ContainsError> {
        self.extrinsics
            .iter()
            .filter(|xt| xt.signature.is_some())
            .find(|xt| {
                xt.method.pallet == method.pallet
                    && xt.method.method == method.method
                    && &xt.signature.as_ref().unwrap().signer.id == account_id
            })
            .ok_or(ContainsError::ExtrinsicNotFound)?
            .events
            .iter()
            .find(|event| event.method.method == event_name)
            .ok_or(ContainsError::EventNotFound)
            .map(|_| ())
    }
}
