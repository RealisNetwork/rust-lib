use runtime::AccountId;
use serde::Deserialize;

pub trait RealisColdWallet {
    fn get_public() -> AccountId;
}

pub struct RealisGameApi {}

impl RealisColdWallet for RealisGameApi {
    fn get_public() -> AccountId {
        Deserialize::deserialize(serde_json::Value::from("5EYCAe5j8NRhivshsbFnhANZuAxwvVG6pVAgkhhRH1k6fpHM")).unwrap()
    }
}

pub struct Staking {}

impl RealisColdWallet for Staking {
    fn get_public() -> AccountId {
        Deserialize::deserialize(serde_json::Value::from("5EYCAe5gKAhKZcQsCCDnRgeVRrnyjZwG7WkBqzQdqFxQ8T9W")).unwrap()
    }
}