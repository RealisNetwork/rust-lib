use log::{warn, info};

pub struct Config {
    pub query: String,
    pub filename: String,
    pub database_cfg: String,
    pub url: String,
    pub nats_options: String,
    pub cluster_id: String,
    pub client_id: String,
    pub subject: String,
}

impl Config {
    /// # Errors
    ///
    /// If `.env` file can not be parsed
    ///
    /// # Panics
    pub fn key_from_value(key: &str) -> Result<String, String> {
        dotenv::dotenv().ok();
        match dotenv::var(key) {
            Ok(value) => {
                info!("Got env value: {}; by key: {}", value, key);
                Ok(value)
            },
            Err(_) => Err(format!("Missing env value by key: {:?}", key)),
        }
    }

    #[must_use]
    pub fn is_restore() -> bool {
        dotenv::dotenv().ok();

        match dotenv::var("IS_RESTORE") {
            Ok(value) => matches!(&value.to_lowercase()[..], "true"),
            Err(_) => false,
        }
    }

    #[must_use]
    pub fn get_max_batch_size() -> usize {
        dotenv::dotenv().ok();

        let default_value = 500;

        match dotenv::var("MAX_BATCH_SIZE") {
            Ok(value) => match value.parse::<usize>() {
                Ok(value) => value,
                Err(_) => default_value,
            },
            Err(_) => default_value,
        }
    }

    #[must_use]
    pub fn get_batch_timeout() -> u64 {
        dotenv::dotenv().ok();

        let default_value = 100;

        match dotenv::var("BATCH_TIMEOUT") {
            Ok(value) => match value.parse::<u64>() {
                Ok(value) => value,
                Err(_) => default_value,
            },
            Err(_) => default_value,
        }
    }

    #[must_use]
    pub fn get_subjects(key: &str) -> Vec<String> {
        dotenv::dotenv().ok();
        if let Ok(value) = dotenv::var(key) {
            value
                .split(',')
                .map(std::string::ToString::to_string)
                .collect()
        } else {
            warn!("Missing enf field: {}", key);
            vec![String::from("adapter")]
        }
    }
}
