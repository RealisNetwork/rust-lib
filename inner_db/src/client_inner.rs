use backoff::{ExponentialBackoff, ExponentialBackoffBuilder};
use deadpool_postgres::{Manager, Pool};
use error_registry::{
    custom_errors::{CustomErrorType, Db as CustomDb},
    generated_errors::{Db, GeneratedError},
    BaseError, ErrorType,
};
use itertools::Itertools;
use log::{error, trace};
use rawsql::Loader;
use std::time::Duration;

pub struct DatabaseClientInner {
    pub client_pool: Pool,
    pub max_interval: u64,
    pub max_elapsed_time: u64,
}

impl DatabaseClientInner {
    pub fn new(client_pool: Pool, max_interval: u64, max_elapsed_time: u64) -> Self {
        Self {
            client_pool,
            max_interval,
            max_elapsed_time,
        }
    }

    pub async fn import_tables_from_file(&self, path: &str) -> Result<(), BaseError<()>> {
        let futures = Loader::get_queries_from(path)
            .unwrap()
            .0
            .into_iter()
            .sorted()
            .map(|(_, query)| async move {
                self.client_pool
                    .get()
                    .await?
                    .execute(&query, &[])
                    .await
                    .map_err(|_| BaseError::<()>::from(GeneratedError::Db(Db::Create)))
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
            .with_max_elapsed_time(Option::from(Duration::from_secs(self.max_elapsed_time)))
            .build()
    }
}
