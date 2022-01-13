use std::time::Duration;
use deadpool_postgres::Pool;
use log::{error, trace};
use rawsql::Loader;
use itertools::Itertools;
use crate::error_registry::{Db, RealisErrors};
use backoff::{ExponentialBackoff, ExponentialBackoffBuilder};

pub struct DatabaseClientInner {
    pub client_pool: Pool,
    pub max_interval: u64,
}

impl DatabaseClientInner {
    pub fn new(client_pool: Pool, max_interval: u64) -> Self {
        Self {
            client_pool,
            max_interval,
        }
    }

    pub async fn import_tables_from_file(&self, path: &str) -> Result<(), RealisErrors> {
        let futures = Loader::get_queries_from(path)?
            .queries
            .into_iter()
            .sorted()
            .map(|(_, query)| async move {
                self.client_pool.get().await?
                    .execute(&query, &[]).await
                    .map_err(|_| RealisErrors::Db(Db::Create))
            });

        for future in futures {
            match future.await {
                Ok(_) => trace!("Successful send query!"),
                Err(error) => error!("Cannot send query: {:?}", error),
            }
        }

        Ok(())
    }

    pub fn get_backoff(&self) -> ExponentialBackoff {
        ExponentialBackoffBuilder::new()
            .with_max_interval(Duration::from_secs(self.max_interval))
            .with_randomization_factor(0.0)
            .with_max_elapsed_time(Some(Duration::from_millis(5000)))
            .build()
    }
}
