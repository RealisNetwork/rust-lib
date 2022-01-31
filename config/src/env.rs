use dotenv::Error;
pub use realis_macros::Env;

#[derive(Debug)]
pub enum EnvLoadedError {
    Load(Error),
    Convert(String),
}

impl From<Error> for EnvLoadedError {
    fn from(error: Error) -> Self {
        Self::Load(error)
    }
}

impl From<String> for EnvLoadedError {
    fn from(error_msg: String) -> Self {
        Self::Convert(error_msg)
    }
}

pub trait EnvLoaded: Sized {
    fn load(key: Option<String>) -> Result<Self, EnvLoadedError>;
}

impl EnvLoaded for bool {
    fn load(key: Option<String>) -> Result<Self, EnvLoadedError> {
        // log::info!("Read env by key: {}", key);
        Ok(dotenv::var(key.unwrap())?
            .parse::<bool>()
            .map_err(|error| error.to_string())?)
    }
}

impl EnvLoaded for usize {
    fn load(key: Option<String>) -> Result<Self, EnvLoadedError> {
        // log::info!("Read env by key: {:?}", key);
        Ok(dotenv::var(key.unwrap())?
            .parse::<usize>()
            .map_err(|error| error.to_string())?)
    }
}

impl EnvLoaded for u16 {
    fn load(key: Option<String>) -> Result<Self, EnvLoadedError> {
        // log::info!("Read env by key: {:?}", key);
        Ok(dotenv::var(key.unwrap())?
            .parse::<u16>()
            .map_err(|error| error.to_string())?)
    }
}

impl EnvLoaded for u32 {
    fn load(key: Option<String>) -> Result<Self, EnvLoadedError> {
        // log::info!("Read env by key: {:?}", key);
        Ok(dotenv::var(key.unwrap())?
            .parse::<u32>()
            .map_err(|error| error.to_string())?)
    }
}

impl EnvLoaded for u64 {
    fn load(key: Option<String>) -> Result<Self, EnvLoadedError> {
        // log::info!("Read env by key: {:?}", key);
        Ok(dotenv::var(key.unwrap())?
            .parse::<u64>()
            .map_err(|error| error.to_string())?)
    }
}

impl EnvLoaded for String {
    fn load(key: Option<String>) -> Result<Self, EnvLoadedError> {
        // println!("Read env by key: {:?}", key);
        Ok(dotenv::var(key.unwrap())?)
    }
}

pub fn default_logger_level() -> String {
    log::LevelFilter::Trace.to_string()
}
