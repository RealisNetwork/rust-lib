use crate::errors::Error;
use bytebuffer::{ByteBuffer, Endian};

pub struct ByteWriter {
    pub(crate) byte_buffer: ByteBuffer,
}

impl ByteWriter {
    #[must_use]
    pub fn new(byte_buffer: ByteBuffer) -> Self {
        Self { byte_buffer }
    }

    #[must_use]
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let mut byte_buffer = ByteBuffer::from_bytes(bytes);
        byte_buffer.set_endian(Endian::LittleEndian);
        Self::new(byte_buffer)
    }

    pub fn clear(&mut self) {
        self.byte_buffer.clear();
    }

    #[must_use]
    pub fn get_position(&self) -> usize {
        self.byte_buffer.get_wpos()
    }

    #[must_use]
    pub fn as_vec(&self) -> Vec<u8> {
        self.byte_buffer.to_bytes()
    }

    pub fn write_bytes(&mut self, bytes: &[u8]) {
        self.byte_buffer.write_bytes(bytes);
    }

    /// # Errors
    /// TODO
    pub fn write_bytes_and_size(&mut self, bytes: &[u8], length: usize) -> Result<(), Error> {
        self.byte_buffer
            .write_i32(i32::try_from(length).map_err(|_| Error::TooBigLength(length))?);
        self.byte_buffer.write_bytes(bytes);
        Ok(())
    }

    pub fn skip(&mut self, length: usize) {
        self.byte_buffer.set_wpos(self.byte_buffer.get_wpos() + length);
    }

    pub fn seek_zero(&mut self) {
        self.byte_buffer.set_wpos(0);
    }
}

impl Default for ByteWriter {
    fn default() -> Self {
        ByteWriter::from_bytes(&[])
    }
}
