use error_registry::BaseError;
pub use realis_macros::Env;

#[allow(clippy::module_name_repetitions)]
pub trait EnvLoaded: Sized {
    fn load(key: Option<String>) -> Result<Self, BaseError<()>>;
}

impl EnvLoaded for bool {
    fn load(key: Option<String>) -> Result<Self, BaseError<()>> {
        // println!("Read env by key: {:?}", key);
        Ok(dotenv::var(key.clone().unwrap())?.parse::<bool>()?)
    }
}

impl EnvLoaded for usize {
    fn load(key: Option<String>) -> Result<Self, BaseError<()>> {
        // println!("Read env by key: {:?}", key);
        Ok(dotenv::var(key.clone().unwrap())?.parse::<usize>()?)
    }
}

impl EnvLoaded for u16 {
    fn load(key: Option<String>) -> Result<Self, BaseError<()>> {
        // println!("Read env by key: {:?}", key);
        Ok(dotenv::var(key.clone().unwrap())?.parse::<u16>()?)
    }
}

impl EnvLoaded for u32 {
    fn load(key: Option<String>) -> Result<Self, BaseError<()>> {
        // println!("Read env by key: {:?}", key);
        Ok(dotenv::var(key.clone().unwrap())?.parse::<u32>()?)
    }
}

impl EnvLoaded for u64 {
    fn load(key: Option<String>) -> Result<Self, BaseError<()>> {
        // println!("Read env by key: {:?}", key);
        Ok(dotenv::var(key.clone().unwrap())?.parse::<u64>()?)
    }
}

impl EnvLoaded for Vec<u8> {
    fn load(key: Option<String>) -> Result<Self, BaseError<()>> {
        // println!("Read env by key: {:?}", key);
        Ok(hex::decode(dotenv::var(key.clone().unwrap())?)?)
    }
}

impl EnvLoaded for String {
    fn load(key: Option<String>) -> Result<Self, BaseError<()>> {
        // println!("Read env by key: {:?}", key);
        Ok(dotenv::var(key.clone().unwrap())?)
    }
}

pub fn default_logger_level() -> String {
    log::LevelFilter::Trace.to_string()
}
