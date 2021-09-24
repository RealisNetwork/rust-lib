use crate::{
    config::Config,
    primitives::adapter::{
        db::Status,
        request::{Params, Request},
        Error,
    },
};
use aes::Aes256;
use bip39::{Language, Mnemonic, MnemonicType};
use block_modes::{block_padding::Pkcs7, Cbc};
use log::{error, trace};
use postgres::NoTls;
use rawsql::{self, Loader};
use runtime::AccountId;
use sp_core::{
    crypto::{Pair, Ss58Codec},
    H256,
};
use std::{
    str::FromStr,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};
use tokio_postgres::Client;

#[allow(dead_code)]
type Aes256Cbc = Cbc<Aes256, Pkcs7>;

pub struct Database {
    client: Client,
    has_err: Arc<AtomicBool>,
}

// TODO add trace
impl Database {
    /// # Panics
    /// # Errors
    pub async fn new() -> Result<Self, tokio_postgres::Error> {
        let (client, connection) =
            tokio_postgres::connect(&Config::key_from_value("DATABASE_CFG").unwrap(), NoTls)
                .await?;
        let has_err = Arc::new(AtomicBool::new(false));
        tokio::spawn({
            let has_err = Arc::clone(&has_err);
            async move {
                if connection.await.is_err() {
                    has_err.store(true, Ordering::Release);
                }
            }
        });

        Ok(Database { client, has_err })
    }

    /// # Panics
    /// # Errors
    pub async fn still_alive(&self) -> Result<(), Error> {
        if self.has_err.load(Ordering::Acquire) {
            Err(Error::Disconnected)
        } else {
            Ok(())
        }
    }

    /// # Panics
    /// # Errors
    pub async fn add_user(&self, user_id: &str) -> Result<(AccountId, String), Error> {
        self.still_alive().await?;

        let (account_id, mnemonic) = Database::wallet();
        // TODO compare returned value with 1 (must change only one raw)
        self.client
            .execute(
                "UPDATE users_accounts \
                 SET account_id=$1 \
                 WHERE user_id=$2",
                &[&account_id.to_string(), &user_id],
            )
            .await
            .map_err(Error::Postgres)?;

        Ok((account_id, mnemonic))
    }

    /// # Panics
    /// # Errors
    pub async fn remove_user(&self, user_id: &str) -> Result<(), Error> {
        self.still_alive().await?;

        // TODO compare returned value with 1 (must change only one raw)
        self.client
            .execute(
                "DELETE FROM users_accounts \
            WHERE user_id = $1",
                &[&user_id],
            )
            .await
            .map(|_| ())
            .map_err(Error::Postgres)
    }

    /// # Panics
    /// # Errors
    pub async fn get_account_id(&self, user_id: &str) -> Result<AccountId, Error> {
        self.still_alive().await?;

        let row = self
            .client
            .query_one(
                "SELECT account_id FROM users_accounts WHERE user_id=$1",
                &[&user_id],
            )
            .await
            .map_err(Error::Postgres)?;
        match row.try_get(0) {
            Ok(value) => Ok(AccountId::from_ss58check(value).unwrap()),
            Err(error) => Err(error),
        }
        .map_err(Error::Postgres)
    }

    /// # Panics
    /// # Errors
    pub async fn add_request_status(&self, request: &Request, status: Status) -> Result<(), Error> {
        self.still_alive().await?;

        if let Params::CreateWallet {} = request.params {
            self.client
                .execute_raw(
                    "INSERT INTO users_accounts \
                        (user_id, account_id) \
                        VALUES ($1, NULL)",
                    &[&request.auth_info.user_id],
                )
                .await
                .map_err(Error::Postgres)?;
        }

        let json = serde_json::to_value(&request).unwrap();

        self.client
            .execute(
                "INSERT INTO requests (id, user_id, json, status) \
                VALUES ($1, $2, $3, $4)",
                &[
                    &request.id,
                    &request.auth_info.user_id,
                    &json,
                    &(status as u32),
                ],
            )
            .await
            .map_err(Error::Postgres)?;

        Ok(())
    }

    /// # Panics
    /// # Errors
    pub async fn update_request_status(
        &self,
        request: &Request,
        status: Status,
    ) -> Result<(), Error> {
        self.still_alive().await?;

        // TODO compare returned value with 1 (must change only one raw)
        self.client
            .execute(
                "UPDATE requests \
                SET status = $1 \
                WHERE id=$2",
                &[&(status as u32), &request.id],
            )
            .await
            .map(|_| ())
            .map_err(Error::Postgres)
    }

    /// # Errors
    pub async fn update_tx_hash(&self, tx_hash: H256, id: &str) -> Result<(), Error> {
        self.still_alive().await?;

        // TODO compare returned value with 1 (must change only one raw)
        self.client
            .execute(
                "UPDATE requests \
                SET hash = $1 \
                WHERE id=$2",
                &[&format!("{:?}", tx_hash), &id],
            )
            .await
            .map(|_| ())
            .map_err(Error::Postgres)
    }

    /// # Panics
    /// # Errors
    pub async fn import_tables_from_file(&self, path: &str) -> Result<(), Error> {
        self.still_alive().await?;

        let queries = Loader::get_queries_from(path)
            .map_err(|_| Error::FileNotFound(String::from(path)))?
            .queries;

        let mut queries = queries.iter().collect::<Vec<(&String, &String)>>();

        queries.sort();

        // TODO: если у нас есть вектор футур, то их выполнение можно распараллелить
        for query in queries {
            // TODO compare returned value with 1 (must change only one raw)
            match self.client.execute(query.1.as_str(), &[]).await {
                Ok(_value) => trace!("Successful send query!"),
                Err(error) => error!("Cannot send query: {:?}", error),
            }
        }

        Ok(())
    }

    /// # Errors
    pub async fn restore(&self, status: Status) -> Result<Vec<Request>, Error> {
        self.still_alive().await?;

        let rows = self
            .client
            .query(
                "SELECT id \
                FROM requests \
                WHERE status = $1",
                &[&(status as u32)],
            )
            .await
            .map_err(Error::Postgres)?;

        let mut buffer = vec![];

        // TODO: если у нас есть вектор футур, то их выполнение можно распараллелить
        for request_id in rows.into_iter().map(|row| row.get::<_, String>(0)) {
            match self.get_request(&request_id).await {
                Ok(value) => buffer.push(value),
                Err(error) => error!("Cannot restore, because {:?} {:?}", request_id, error),
            }
        }

        Ok(buffer)
    }

    async fn get_request(&self, request_id: &str) -> Result<Request, Error> {
        self.still_alive().await?;

        let row = self
            .client
            .query_one(
                "
                    SELECT json \
                    FROM requests \
                    WHERE id = $1",
                &[&request_id],
            )
            .await
            .map_err(Error::Postgres)?;
        let request = row.try_get(0).map_err(Error::Postgres)?;

        Ok(serde_json::from_value(request).map_err(Error::SerdeJSON)?)
    }

    /// # Errors
    pub async fn get_tx_hash(&self, request_id: &str) -> Result<H256, Error> {
        self.still_alive().await?;

        let row = self
            .client
            .query_one(
                "
                    SELECT hash \
                    FROM requests \
                    WHERE id = $1",
                &[&request_id],
            )
            .await
            .map_err(Error::Postgres)?;
        let hash_string: String = row.try_get(0).map_err(Error::Postgres)?;
        let tx_hash = H256::from_str(hash_string.as_str()).map_err(|_| Error::CannotDecode)?;

        Ok(tx_hash)
    }

    /// # Errors
    pub async fn add_mnemonic(&self, mnemonic: &[u8], user_id: &str) -> Result<(), Error> {
        self.still_alive().await?;

        self.client
            .execute(
                "
                    UPDATE users_accounts \
                    SET mnemonic = $1 \
                    WHERE user_id = $2",
                &[&mnemonic, &user_id],
            )
            .await
            .map_err(Error::Postgres)?;

        Ok(())
    }

    // pub async fn get_mnemonic(&mut self, account_id: AccountId) -> Result<String,
    // Error> {     self.still_alive().await;
    //     let row = self.client.query_one("SELECT mnemonic \
    //      FROM users_accounts \
    //      WHERE account_id = $1",
    //     &[&account_id.to_string()]
    //     ).await;
    //     warn!("Query successful");
    //     match row {
    //         Ok(value) => {
    //             let db_mnemonic = value.try_get(0).map_err(Error::Postgres);
    //             let decoded_mnemonic = decrypted_mnemonic(db_mnemonic.unwrap());
    //             Ok(decoded_mnemonic)
    //         }
    //         Err(error) => {
    //             error!("Cannot decode mnemonic: {:?}", error);
    //             Err(Error::Postgres(error))
    //         },
    //     }
    // }

    fn wallet() -> (AccountId, String) {
        trace!("Realis db - creating wallet...");

        let mnemonic = Mnemonic::new(MnemonicType::Words12, Language::English);
        let phrase = mnemonic.phrase();

        let (pair, _) = sp_keyring::sr25519::sr25519::Pair::from_phrase(phrase, None).unwrap();
        let public = pair.public();
        let account_id = AccountId::new(public.0);

        (account_id, String::from(phrase))
    }
}
