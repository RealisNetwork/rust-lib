use rust_lib::request::{Gettable, Request};
use primitives::errors::Error;

use crate::traits::{AdapterRequest, RequestType, Wallet};
use rust_lib::primitives::adapter::request::{Id, TopicRes};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use substrate_api_client::{
    compose_extrinsic, Api, ApiClientError, UncheckedExtrinsicV4, XtStatus,
};
use substrate_api_client::sp_runtime::app_crypto::{sr25519, Pair};
use primitives::responses::ResponseFormatter;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateWalletCall {
    id: Id,
    topic_res: TopicRes,
    mnemonic: Vec<u8>,
}

impl CreateWalletCall {
    fn decode(&self) -> String {
        String::from_utf8(self.mnemonic.clone()).unwrap()
    }
}

impl Request for CreateWalletCall {
    type Error = Error;

    fn new(json: &Value) -> Result<Self, Error>
        where
            Self: Sized,
    {
        serde_json::from_value(json.clone()).map_err(Error::SerdeJSON)
    }

    fn get_id(&self) -> String {
        self.id.clone()
    }
}

impl AdapterRequest for CreateWalletCall {
    fn classify(self) -> RequestType {
        RequestType::CreateWallet(Box::new(self))
    }
}

impl Wallet for CreateWalletCall {
    fn add_to_whitelist(&self, url: &str) -> Result<Value, Error> {
        let mnemonic = self.decode();

        let (pair, _) = Pair::from_phrase(&mnemonic, None)
            .map_err(|_| ApiClientError::NoSigner)
            .map_err(Error::Api)?;

        let api = Api::<sr25519::Pair>::new(url.to_string())
            .map(|api| api.set_signer(pair))
            .map_err(Error::Api)?;

        let xt: UncheckedExtrinsicV4<_> =
            compose_extrinsic!(&api, "RealisGameApi", "add_to_whitelist");

        match api.send_extrinsic(xt.hex_encode(), XtStatus::InBlock) {
            Ok(Some(hash)) => Ok(ResponseFormatter::with_hash(self, &hash.to_string())),
            Ok(None) => Ok(ResponseFormatter::with_error(self, primitives::errors::Error::ErrorFromString("Blockchain error".to_string()))),
            Err(error) => Ok(ResponseFormatter::with_error(self, primitives::errors::Error::Api(error))),
        }
    }
}

impl Gettable for CreateWalletCall {
    fn topic() -> String {
        String::from("adapter-create_wallet")
    }
}
