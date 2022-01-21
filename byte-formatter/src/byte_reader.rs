use crate::errors::Error;

use crate::ByteDeserialize;
use bytebuffer::{ByteBuffer, Endian};
use std::cmp::max;

pub struct ByteReader {
    pub(crate) byte_buffer: ByteBuffer,
}

impl ByteReader {
    #[must_use]
    pub fn new(mut byte_buffer: ByteBuffer) -> Self {
        byte_buffer.set_endian(Endian::LittleEndian);
        Self { byte_buffer }
    }

    #[must_use]
    pub fn from_bytes(bytes: &[u8]) -> Self {
        Self::new(ByteBuffer::from_bytes(bytes))
    }

    pub fn clear(&mut self) {
        self.byte_buffer.clear();
    }

    #[must_use]
    pub fn get_position(&self) -> usize {
        self.byte_buffer.get_rpos()
    }

    #[must_use]
    pub fn get_length(&self) -> usize {
        self.byte_buffer.len()
    }

    pub fn seek_zero(&mut self) {
        self.byte_buffer.set_rpos(0);
    }

    #[must_use]
    pub fn readable_bytes(&self) -> usize {
        max(0, self.get_length() - self.get_position())
    }

    pub fn skip(&mut self, length: usize) {
        for _ in 0..length {
            let _byte = u8::decode(self);
        }
    }

    #[must_use]
    pub fn as_vec(&self) -> Vec<u8> {
        self.byte_buffer.to_bytes()
    }

    /// # Errors
    /// TODO
    pub fn read_bytes(&mut self, length: usize) -> Result<Vec<u8>, Error> {
        self.byte_buffer.read_bytes(length).map_err(Error::IO)
    }

    /// # Errors
    /// TODO
    pub fn read_bytes_and_size(&mut self) -> Result<Vec<u8>, Error> {
        let length = i32::decode(self)?;
        let length = usize::try_from(length).map_err(|_| Error::NegativeLength(length))?;
        self.read_bytes(length)
    }

    /// # Errors
    /// TODO
    pub fn skip_bytes_and_size(&mut self) -> Result<(), Error> {
        let length = i32::decode(self)?;
        let length = usize::try_from(length).map_err(|_| Error::NegativeLength(length))?;
        self.skip(length);

        Ok(())
    }
}
