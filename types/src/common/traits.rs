use byte_formatter::ByteReader;
use serde_json::Value;

#[derive(Debug)]
pub enum EncodeError {
    Parse(serde_json::Error),
    Byte(byte_formatter::Error),
}

pub trait Convertable {
    /// # Errors
    fn from_bytes_to_json(byte_reader: &mut ByteReader) -> Result<Value, byte_formatter::Error>;

    ////

    fn topic_to_send() -> String;

    fn topic_to_response() -> String;

    ////

    /// # Errors
    fn from_value_to_bytes(json: Value) -> Result<Vec<u8>, EncodeError>;
}
