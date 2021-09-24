use std::{str::FromStr, string::ToString};

#[derive(Debug)]
pub enum Status {
    // Got from message-broker
    Got,
    // Sent to blockchain , wait for result
    InProgress,
    // In block
    Success,
    // Blockchain fail
    Error,
    // Response sent
    Complete,
    // Responder can't send
    SendError,
}

impl Status {
    #[must_use]
    pub fn to_u32(&self) -> u32 {
        match self {
            Status::Got => 1,
            Status::InProgress => 2,
            Status::Success => 3,
            Status::Error => 4,
            Status::Complete => 5,
            Status::SendError => 6,
        }
    }
}

impl ToString for Status {
    fn to_string(&self) -> String {
        match self {
            Status::Got => 1,
            Status::InProgress => 2,
            Status::Success => 3,
            Status::Error => 4,
            Status::Complete => 5,
            Status::SendError => 6,
        }
        .to_string()
    }
}

impl FromStr for Status {
    type Err = String;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        match string {
            "1" => Ok(Status::Got),
            "2" => Ok(Status::InProgress),
            "3" => Ok(Status::Success),
            "4" => Ok(Status::Error),
            "5" => Ok(Status::Complete),
            "6" => Ok(Status::SendError),
            _ => Err(String::from("Unknown request status id!")),
        }
    }
}
