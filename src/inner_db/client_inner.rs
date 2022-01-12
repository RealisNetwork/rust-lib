use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use deadpool_postgres::{Pool, Status};
use log::{error, trace};
use tokio_postgres::{Connection, Socket, Client, Error};
use tokio_postgres::tls::{MakeTlsConnect, TlsStream};
use rawsql::Loader;
use itertools::Itertools;
use crate::healthchecker::HealthChecker;

pub struct DatabaseClientInner {
    pub client_pool: Pool,
    connection_is_alive: Arc<AtomicBool>,
}

impl DatabaseClientInner {
    pub(crate) fn new(client_pool: Pool) -> Self {
        let connection_is_alive = Arc::new(AtomicBool::new(true));

        Self {
            client_pool,
            connection_is_alive,
        }
    }

    // TODO remove this
    pub async fn still_alive(&self) -> Result<(), ()> {
        Ok(())
    }

    pub async fn import_tables_from_file(&self, path: &str) -> Result<(), std::io::Error> {
        let futures = Loader::get_queries_from(path)?
            .queries
            .into_iter()
            .sorted()
            .map(|(_, query)| async move {
                self.client_pool.get().await.unwrap().execute(&query, &[]).await // TODO handle this unwrap
            });

        for future in futures {
            match future.await {
                Ok(_) => trace!("Successful send query!"),
                Err(error) => error!("Cannot send query: {:?}", error),
            }
        }

        Ok(())
    }
}
