pub use hashicorp_vault::Error;
use hashicorp_vault::{
    client::{VaultClient, TokenData, HttpVerb, EndpointResponse},
    Client,
};
use serde::de::DeserializeOwned;

#[derive(Debug)]
pub struct Vault<T> {
    pub(crate) client: VaultClient<T>,
}

impl Vault<()> {
    pub fn new(host: String, role_id: String, secret_id: String) -> Result<Self, Error> {
        // let client = Client::new_app_id(host, role_id, secret_id)?;
        let client = Client::new_app_role(host, role_id, Some(secret_id))?;

        Ok(Self { client })
    }
}

impl Vault<TokenData> {
    pub fn new(host: String, token: String) -> Result<Self, Error> {
        let client = Client::new(host, token)?;

        Ok(Self { client })
    }
}

impl Vault<()> {
    pub fn get_secret<T: DeserializeOwned>(&self, key: &str) -> Result<T, Error> {
        let res = self.client.call_endpoint(HttpVerb::GET, key, None, None)?;
        match res {
            EndpointResponse::VaultResponse(res) => match res.data {
                Some(data) => Ok(data),
                None => Err(Error::Vault("Secret is missing: {:?}".to_owned())),
            },
            EndpointResponse::Empty => Err(Error::Vault("Received an empty response".to_string())),
        }
    }

    pub fn set_secret(&self, key: &str, value: &str) -> Result<(), Error> {
        self.client.set_secret(key, value)
    }

    #[cfg(test)]
    pub fn delete_secret(&self, key: &str) -> Result<(), Error> {
        self.client.delete_secret(key)
    }
}

/// This tests require
/// *.env variable - with name `VAULT_ROLE_ID` and `VAULT_SECRET_ID` - for auth in vault
/// *vault - running by host: `http://127.0.0.1:8200`
#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::vault::Vault;

    // Command for port forward
    // kubectl port-forward pods/vault-0 8200:8200 -n vault
    const HOST: &str = "http://127.0.0.1:8200";

    #[test]
    fn test_connect_app_auth() {
        let role_id = dotenv::var("VAULT_ROLE_ID").unwrap();
        let secret_id = dotenv::var("VAULT_SECRET_ID").unwrap();
        let res = Vault::<()>::new(HOST.to_owned(), role_id, secret_id);
        assert!(res.is_ok());
    }

    #[test]
    fn test_get_secret_app_auth() {
        const KEY: &str = "realis/masterAccount";
        let role_id = dotenv::var("VAULT_ROLE_ID").unwrap();
        let secret_id = dotenv::var("VAULT_SECRET_ID").unwrap();
        let vault = Vault::<()>::new(HOST.to_owned(), role_id, secret_id).unwrap();

        let result = vault.get_secret::<HashMap<String, String>>(KEY);
        assert!(result.is_ok());
    }
}
