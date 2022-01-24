pub mod topic;
pub mod import;

use std::str::FromStr;

use import::Import;
use topic::Topic;

pub enum ParseResult {
    Import(Import),
    Topic(Topic),
}

pub fn parse_line(line: String) -> Option<ParseResult> {
    Import::from_str(&line)
        .ok()
        .or(Topic::from_str(&line).ok())

}