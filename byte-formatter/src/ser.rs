use crate::{ByteWriter, Error};

pub trait ByteSerialize {
    /// # Errors
    /// TODO
    fn encode(self, byte_writer: &mut ByteWriter) -> Result<(), Error>;
}

impl<T> ByteSerialize for Vec<T>
where
    T: ByteSerialize,
{
    fn encode(self, byte_writer: &mut ByteWriter) -> Result<(), Error> {
        let length = self.len();
        i16::try_from(length)
            .map_err(|_| Error::TooBigLength(length))?
            .encode(byte_writer)?;
        self.into_iter()
            .map(|value| value.encode(byte_writer))
            .collect::<Result<Vec<_>, Error>>()
            .map(|_| ())
    }
}

impl<T> ByteSerialize for Option<T>
where
    T: ByteSerialize,
{
    fn encode(self, byte_writer: &mut ByteWriter) -> Result<(), Error> {
        match self {
            None => false.encode(byte_writer),
            Some(value) => {
                true.encode(byte_writer)?;
                value.encode(byte_writer)
            }
        }
    }
}

impl ByteSerialize for bool {
    fn encode(self, byte_writer: &mut ByteWriter) -> Result<(), Error> {
        byte_writer.byte_buffer.write_bit(self);
        Ok(())
    }
}

impl ByteSerialize for u8 {
    fn encode(self, byte_writer: &mut ByteWriter) -> Result<(), Error> {
        byte_writer.byte_buffer.write_u8(self);
        Ok(())
    }
}

impl ByteSerialize for i8 {
    fn encode(self, byte_writer: &mut ByteWriter) -> Result<(), Error> {
        byte_writer.byte_buffer.write_i8(self);
        Ok(())
    }
}

impl ByteSerialize for i16 {
    fn encode(self, byte_writer: &mut ByteWriter) -> Result<(), Error> {
        byte_writer.byte_buffer.write_i16(self);
        Ok(())
    }
}

impl ByteSerialize for i32 {
    fn encode(self, byte_writer: &mut ByteWriter) -> Result<(), Error> {
        byte_writer.byte_buffer.write_i32(self);
        Ok(())
    }
}

impl ByteSerialize for i64 {
    fn encode(self, byte_writer: &mut ByteWriter) -> Result<(), Error> {
        byte_writer.byte_buffer.write_i64(self);
        Ok(())
    }
}

impl ByteSerialize for char {
    fn encode(self, byte_writer: &mut ByteWriter) -> Result<(), Error> {
        byte_writer.byte_buffer.write_u8(self as u8);
        Ok(())
    }
}

impl ByteSerialize for &str {
    fn encode(self, byte_writer: &mut ByteWriter) -> Result<(), Error> {
        self.to_string().encode(byte_writer)
    }
}

impl ByteSerialize for String {
    fn encode(self, byte_writer: &mut ByteWriter) -> Result<(), Error> {
        let bytes = self.as_bytes();
        let length = bytes.len();
        i32::try_from(length)
            .map_err(|_| Error::TooBigLength(length))?
            .encode(byte_writer)?;
        byte_writer.byte_buffer.write_bytes(bytes);
        Ok(())
    }
}
