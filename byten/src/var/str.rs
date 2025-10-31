use std::{string::String as StdString, ffi::CString as StdCString};

pub struct String<Length> {
    pub length: Length,
}

impl<Length> String<Length> {
    pub const fn codec(length: Length) -> Self {
        Self { length }
    }
}

impl<Length> Default for String<Length>
where
    Length: Default,
{
    fn default() -> Self {
        Self::codec(Length::default())
    }
}

impl<'encoded, 'length, Length> crate::BorrowedDecoder<'encoded, '_> for String<Length>
where
    Length: crate::BorrowedDecoder<'encoded, 'length, Decoded = usize>,
{
    type Decoded = StdString;

    fn borrowed_decode(&self, encoded: &'encoded [u8], offset: &mut usize) -> Result<Self::Decoded, crate::DecodeError> {
        let size = self.length.borrowed_decode(encoded, offset)?;
        if *offset + size > encoded.len() {
            return Err(crate::DecodeError::InvalidData);
        }
        let string_bytes = &encoded[*offset..*offset + size];
        let string = StdString::from_utf8(string_bytes.to_vec()).map_err(|_| crate::DecodeError::InvalidData)?;
        *offset += size;
        Ok(string)
    }
}

impl<Length> crate::Encoder for String<Length>
where
    Length: crate::Encoder<Decoded = usize>,
{
    type Decoded = StdString;

    fn encode(&self, decoded: &Self::Decoded, encoded: &mut [u8], offset: &mut usize) -> Result<(), crate::EncodeError> {
        let size = decoded.len();
        self.length.encode(&size, encoded, offset)?;
        let end = *offset + size;
        if end > encoded.len() {
            return Err(crate::EncodeError::BufferTooSmall);
        }
        encoded[*offset..end].copy_from_slice(decoded.as_bytes());
        *offset = end;
        Ok(())
    }
}

impl<Length> crate::Measurer for String<Length>
where
    Length: crate::Measurer<Decoded = usize>,
{
    type Decoded = StdString;

    fn measure(&self, decoded: &Self::Decoded) -> Result<usize, crate::EncodeError> {
        let size = decoded.len();
        let size_measure = self.length.measure(&size)?;
        Ok(size_measure + size)
    }
}

pub struct CString;

impl CString {
    pub const fn codec() -> Self { Self }
}

impl Default for CString {
    fn default() -> Self { Self::codec() }
}

impl<'encoded> crate::BorrowedDecoder<'encoded, '_> for CString {
    type Decoded = StdCString;

    fn borrowed_decode(&self, encoded: &'encoded [u8], offset: &mut usize) -> Result<Self::Decoded, crate::DecodeError> {
        let start = *offset;
        let end = encoded[start..]
            .iter()
            .position(|&b| b == 0)
            .map(|pos| start + pos)
            .ok_or(crate::DecodeError::InvalidData)?;
        let c_string = StdCString::new(&encoded[start..end]).map_err(|_| crate::DecodeError::InvalidData)?;
        *offset = end + 1; // Move past the null terminator
        Ok(c_string)
    }
}

impl crate::Encoder for CString {
    type Decoded = StdCString;

    fn encode(&self, decoded: &Self::Decoded, encoded: &mut [u8], offset: &mut usize) -> Result<(), crate::EncodeError> {
        let bytes = decoded.to_bytes_with_nul();
        let end = *offset + bytes.len();
        if end > encoded.len() {
            return Err(crate::EncodeError::BufferTooSmall);
        }
        encoded[*offset..end].copy_from_slice(bytes);
        *offset = end;
        Ok(())
    }
}

impl crate::Measurer for CString {
    type Decoded = StdCString;

    fn measure(&self, decoded: &Self::Decoded) -> Result<usize, crate::EncodeError> {
        Ok(decoded.to_bytes_with_nul().len())
    }
}

pub struct Str<'decoded,Length> {
    pub lifetime: core::marker::PhantomData<&'decoded ()>,
    pub length: Length,
}

impl<'decoded, Length> Str<'decoded, Length> {
    pub const fn codec(length: Length) -> Self {
        Self {
            lifetime: core::marker::PhantomData,
            length,
        }
    }
}

impl<'decoded, Length> Default for Str<'decoded, Length>
where
    Length: Default,
{
    fn default() -> Self {
        Self::codec(Length::default())
    }
}

impl<'encoded, 'decoded, 'length, Length> crate::BorrowedDecoder<'encoded, 'decoded> for Str<'decoded, Length>
where
    Length: crate::BorrowedDecoder<'encoded, 'length, Decoded = usize>,
    'encoded: 'decoded,
{
    type Decoded = &'decoded str;

    fn borrowed_decode(&self, encoded: &'encoded [u8], offset: &mut usize) -> Result<Self::Decoded, crate::DecodeError> {
        let size = self.length.borrowed_decode(encoded, offset)?;
        if *offset + size > encoded.len() {
            return Err(crate::DecodeError::InvalidData);
        }
        let string_bytes = &encoded[*offset..*offset + size];
        let string = std::str::from_utf8(string_bytes).map_err(|_| crate::DecodeError::InvalidData)?;
        *offset += size;
        Ok(string)
    }
}

impl<'decoded, Length> crate::Encoder for Str<'decoded, Length>
where
    Length: crate::Encoder<Decoded = usize>,
{
    type Decoded = &'decoded str;

    fn encode(&self, decoded: &Self::Decoded, encoded: &mut [u8], offset: &mut usize) -> Result<(), crate::EncodeError> {
        let size = decoded.len();
        self.length.encode(&size, encoded, offset)?;
        let end = *offset + size;
        if end > encoded.len() {
            return Err(crate::EncodeError::BufferTooSmall);
        }
        encoded[*offset..end].copy_from_slice(decoded.as_bytes());
        *offset = end;
        Ok(())
    }
}

impl<'decoded, Length> crate::Measurer for Str<'decoded, Length>
where
    Length: crate::Measurer<Decoded = usize>,
{
    type Decoded = &'decoded str;

    fn measure(&self, decoded: &Self::Decoded) -> Result<usize, crate::EncodeError> {
        let size = decoded.len();
        let size_measure = self.length.measure(&size)?;
        Ok(size_measure + size)
    }
}
