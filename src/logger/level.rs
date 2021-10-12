use log::LevelFilter;

pub struct Parser {}

impl Parser {
    pub fn from_str(level: &str) -> Option<LevelFilter> {
        match level.to_lowercase().as_str() {
            "off" => Some(LevelFilter::Off),
            "error" => Some(LevelFilter::Error),
            "warn" => Some(LevelFilter::Warn),
            "info" => Some(LevelFilter::Info),
            "debug" => Some(LevelFilter::Debug),
            "trace" => Some(LevelFilter::Trace),
            _ => None
        }
    }
}