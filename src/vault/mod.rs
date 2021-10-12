extern crate hashicorp_vault as vault;
use crate::config::Config;

/// # Panics
#[must_use]
pub fn get_nft_masters(host: String, token: String) -> Vec<u8> {
    let client = vault::Client::new(host, token).unwrap();

    // let _ = client.set_secret("foo", "bar");

    let secret = client.get_secret("nft-masters").unwrap();

    secret.as_bytes().to_vec()
}
