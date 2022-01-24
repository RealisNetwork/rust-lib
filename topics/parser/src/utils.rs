use std::str::FromStr;

use crate::{import::Import, topic::Topic};

#[derive(Debug)]
pub enum ParseResult {
    Import(Import),
    Topic(Topic),
}

pub fn parse_line(line: &str) -> Option<ParseResult> {
    Import::from_str(line)
        .map(|value| ParseResult::Import(value))
        .ok()
        .or(Topic::from_str(line).ok().map(|value| ParseResult::Topic(value)))
}
