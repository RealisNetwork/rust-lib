use deadpool_postgres::Pool;
use log::{error, trace};
use rawsql::Loader;
use itertools::Itertools;

pub struct DatabaseClientInner {
    pub client_pool: Pool,
}

impl DatabaseClientInner {
    pub fn new(client_pool: Pool) -> Self {
        Self {
            client_pool,
        }
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
