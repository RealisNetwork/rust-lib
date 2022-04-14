use crate::{ByteReader, Error};

pub trait ByteDeserialize {
    /// # Errors
    /// TODO
    fn decode(byte_reader: &mut ByteReader) -> Result<Self, Error>
    where
        Self: Sized;
}

impl<T> ByteDeserialize for Vec<T>
where
    T: ByteDeserialize,
{
    fn decode(byte_reader: &mut ByteReader) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let length = usize::decode(byte_reader)?;

        let mut array = Vec::with_capacity(length);

        for _ in 0..length {
            array.push(T::decode(byte_reader)?);
        }

        Ok(array)
    }
}

impl<T> ByteDeserialize for Option<T>
where
    T: ByteDeserialize,
{
    fn decode(byte_reader: &mut ByteReader) -> Result<Self, Error>
    where
        Self: Sized,
    {
        if bool::decode(byte_reader)? {
            Ok(Some(T::decode(byte_reader)?))
        } else {
            Ok(None)
        }
    }
}

impl ByteDeserialize for bool {
    fn decode(byte_reader: &mut ByteReader) -> Result<Self, Error> {
        byte_reader.byte_buffer.read_bit().map_err(Error::IO)
    }
}

impl ByteDeserialize for u8 {
    fn decode(byte_reader: &mut ByteReader) -> Result<Self, Error>
    where
        Self: Sized,
    {
        byte_reader.byte_buffer.read_u8().map_err(Error::IO)
    }
}

impl ByteDeserialize for i8 {
    fn decode(byte_reader: &mut ByteReader) -> Result<Self, Error>
    where
        Self: Sized,
    {
        byte_reader.byte_buffer.read_i8().map_err(Error::IO)
    }
}

impl ByteDeserialize for i16 {
    fn decode(byte_reader: &mut ByteReader) -> Result<Self, Error>
    where
        Self: Sized,
    {
        byte_reader.byte_buffer.read_i16().map_err(Error::IO)
    }
}

impl ByteDeserialize for i32 {
    fn decode(byte_reader: &mut ByteReader) -> Result<Self, Error>
    where
        Self: Sized,
    {
        byte_reader.byte_buffer.read_i32().map_err(Error::IO)
    }
}

impl ByteDeserialize for i64 {
    fn decode(byte_reader: &mut ByteReader) -> Result<Self, Error>
    where
        Self: Sized,
    {
        byte_reader.byte_buffer.read_i64().map_err(Error::IO)
    }
}

impl ByteDeserialize for usize {
    fn decode(byte_reader: &mut ByteReader) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let length = i32::decode(byte_reader)?;
        usize::try_from(length).map_err(|_| Error::NegativeLength(length))
    }
}

impl ByteDeserialize for char {
    fn decode(byte_reader: &mut ByteReader) -> Result<Self, Error>
    where
        Self: Sized,
    {
        byte_reader.byte_buffer.read_u8().map_err(Error::IO).map(std::convert::Into::into)
    }
}

impl ByteDeserialize for String {
    fn decode(byte_reader: &mut ByteReader) -> Result<Self, Error>
    where
        Self: Sized,
    {
        String::from_utf8(byte_reader.read_bytes_and_size()?).map_err(|error| Error::InvalidStringCharacter(format!("{:?}", error)))
    }
}
