use hashicorp_vault::{
    client::{TokenData, VaultClient},
    Client,
};

pub use hashicorp_vault::Error;

pub struct Vault {
    client: VaultClient<TokenData>,
}

impl Vault {
    #[must_use]
    pub fn new(host: String, token: String) -> Result<Self, Error> {
        let client = Client::new(host, token)?;

        Ok(Self { client })
    }

    pub fn get_secret(&self, key: &str) -> Result<String, Error> {
        self.client.get_secret(key)
    }
}
