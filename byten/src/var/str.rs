use std::string::String as StdString;
use std::ffi::CString as StdCString;
use std::ffi::CStr as StdCStr;

pub struct Str<Length> {
    pub length: Length,
}

impl<Length> Str<Length> {
    pub const fn codec(length: Length) -> Self {
        Self {
            length,
        }
    }
}

impl<Length> Default for Str<Length>
where
    Length: Default,
{
    fn default() -> Self {
        Self::codec(Length::default())
    }
}

impl<'encoded, 'decoded, 'length, Length> crate::Decoder<'encoded, 'decoded> for Str<Length>
where
    Length: crate::Decoder<'encoded, 'length, Decoded = usize>,
    'encoded: 'decoded,
{
    type Decoded = &'decoded str;

    fn decode(&self, encoded: &'encoded [u8], offset: &mut usize) -> Result<Self::Decoded, crate::DecodeError> {
        let size = self.length.decode(encoded, offset)?;
        if *offset + size > encoded.len() {
            return Err(crate::DecodeError::InvalidData);
        }
        let string_bytes = &encoded[*offset..*offset + size];
        let string = std::str::from_utf8(string_bytes).map_err(|_| crate::DecodeError::InvalidData)?;
        *offset += size;
        Ok(string)
    }
}

impl<Length> crate::Encoder for Str<Length>
where
    Length: crate::Encoder<Decoded = usize>,
{
    type Decoded = str;

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

impl<Length> crate::Measurer for Str<Length>
where
    Length: crate::Measurer<Decoded = usize>,
{
    type Decoded = str;

    fn measure(&self, decoded: &Self::Decoded) -> Result<usize, crate::EncodeError> {
        let size = decoded.len();
        let size_measure = self.length.measure(&size)?;
        Ok(size_measure + size)
    }
}

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

impl<'encoded, 'length, Length> crate::Decoder<'encoded, 'static> for String<Length>
where
    Length: crate::Decoder<'encoded, 'length, Decoded = usize>,
{
    type Decoded = StdString;

    fn decode(&self, encoded: &'encoded [u8], offset: &mut usize) -> Result<Self::Decoded, crate::DecodeError> {
        let str_codec = Str::codec(&self.length);
        let s = str_codec.decode(encoded, offset)?;
        Ok(s.to_owned())
    }
}

impl<Length> crate::Encoder for String<Length>
where
    Length: crate::Encoder<Decoded = usize>,
{
    type Decoded = StdString;

    fn encode(&self, decoded: &Self::Decoded, encoded: &mut [u8], offset: &mut usize) -> Result<(), crate::EncodeError> {
        let str_codec = Str::codec(&self.length);
        str_codec.encode(&decoded.as_str(), encoded, offset)
    }
}

impl<Length> crate::Measurer for String<Length>
where
    Length: crate::Measurer<Decoded = usize>,
{
    type Decoded = StdString;

    fn measure(&self, decoded: &Self::Decoded) -> Result<usize, crate::EncodeError> {
        let str_codec = Str::codec(&self.length);
        str_codec.measure(&decoded.as_str())
    }
}

pub struct CStr;

impl CStr {
    pub const fn codec() -> Self {
        Self
    }
}

impl Default for CStr {
    fn default() -> Self { Self::codec() }
}

impl<'encoded, 'decoded> crate::Decoder<'encoded, 'decoded> for CStr
where
    'encoded: 'decoded,
{
    type Decoded = &'decoded StdCStr;

    fn decode(&self, encoded: &'encoded [u8], offset: &mut usize) -> Result<Self::Decoded, crate::DecodeError> {
        let length = encoded[*offset..]
            .iter()
            .position(|&b| b == 0)
            .ok_or(crate::DecodeError::InvalidData)?;
        let cstr = StdCStr::from_bytes_with_nul(&encoded[*offset..*offset + length + 1])
            .map_err(|_| crate::DecodeError::InvalidData)?;
        *offset += length + 1;
        Ok(cstr)
    }
}

impl crate::Encoder for CStr {
    type Decoded = StdCStr;

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

impl crate::Measurer for CStr {
    type Decoded = StdCStr;

    fn measure(&self, decoded: &Self::Decoded) -> Result<usize, crate::EncodeError> {
        Ok(decoded.to_bytes_with_nul().len())
    }
}

pub struct CString;

impl CString {
    pub const fn codec() -> Self {
        Self
    }
}

impl Default for CString {
    fn default() -> Self { Self::codec() }
}

impl<'encoded> crate::Decoder<'encoded, 'static> for CString {
    type Decoded = StdCString;

    fn decode(&self, encoded: &'encoded [u8], offset: &mut usize) -> Result<Self::Decoded, crate::DecodeError> {
        Ok(CStr::codec().decode(encoded, offset)?.to_owned())
    }
}

impl crate::Encoder for CString {
    type Decoded = StdCString;

    fn encode(&self, decoded: &Self::Decoded, encoded: &mut [u8], offset: &mut usize) -> Result<(), crate::EncodeError> {
        CStr::codec().encode(&decoded.as_c_str(), encoded, offset)
    }
}

impl crate::Measurer for CString {
    type Decoded = StdCString;

    fn measure(&self, decoded: &Self::Decoded) -> Result<usize, crate::EncodeError> {
        CStr::codec().measure(&decoded.as_c_str())
    }
}
