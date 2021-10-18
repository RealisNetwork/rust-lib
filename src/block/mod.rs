use serde::{Deserialize, Serialize};
use runtime::AccountId;
use reqwest;

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

impl Block {
    pub async fn get_block(host: &str, port: &str, number: &str) -> Result<Block, ()>
    {
        let request = format!("http://{}:{}/blocks/{}", host, port, number);

        reqwest::get(request)
            .await
            .map_err(|_| ())?
            .json()
            .await
            .map_err(|_| ())
    }
}
