use std::io::Error as IOError;

#[derive(Debug)]
pub enum Error {
    IO(IOError),
    NegativeLength(i32),
    TooBigLength(usize),
    InvalidStringCharacter(String),
    Custom(String),
}
