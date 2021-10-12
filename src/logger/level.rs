use log::LevelFilter;

pub struct Parser {}

impl Parser {
    pub fn from_str(level: &str) -> Option<LevelFilter> {
        match level.to_lowercase().as_str() {
            "off" => Some(LevelFilter::Off),
            "error" => Some(LevelFilter::Off),
            "warn" => Some(LevelFilter::Off),
            "info" => Some(LevelFilter::Off),
            "debug" => Some(LevelFilter::Off),
            "trace" => Some(LevelFilter::Off),
            _ => None
        }
    }
}