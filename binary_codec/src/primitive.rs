macro_rules! impl_prim {
    ($($t:tt),+ $(,)?) => {
        $(
            pub mod $t {
                pub mod le {
                    pub fn decode(encoded: &[u8], offset: &mut usize) -> Result<$t, crate::DecodeError> {
                        let n = std::mem::size_of::<$t>();
                        let bytes = &encoded[*offset..*offset + n];
                        *offset += n;
                        Ok(<$t>::from_le_bytes(bytes.try_into().map_err(|_| crate::DecodeError::EOF)?))
                    }

                    pub fn encode(decoded: &$t, encoded: &mut [u8], offset: &mut usize) -> Result<(), crate::EncodeError> {
                        let bytes = decoded.to_le_bytes();
                        if *offset + bytes.len() > encoded.len() {
                            return Err(crate::EncodeError::BufferTooSmall);
                        }
                        encoded[*offset..*offset + bytes.len()].copy_from_slice(&bytes);
                        *offset += bytes.len();
                        Ok(())
                    }

                    pub const fn measure(_decoded: &$t) -> usize {
                        $t::BITS as usize / 8
                    }
                }

                pub mod be {
                    pub fn decode(encoded: &[u8], offset: &mut usize) -> Result<$t, crate::DecodeError> {
                        let n = std::mem::size_of::<$t>();
                        let bytes = &encoded[*offset..*offset + n];
                        *offset += n;
                        Ok(<$t>::from_be_bytes(bytes.try_into().map_err(|_| crate::DecodeError::EOF)?))
                    }

                    pub fn encode(decoded: &$t, encoded: &mut [u8], offset: &mut usize) -> Result<(), crate::EncodeError> {
                        let bytes = decoded.to_be_bytes();
                        if *offset + bytes.len() > encoded.len() {
                            return Err(crate::EncodeError::BufferTooSmall);
                        }
                        encoded[*offset..*offset + bytes.len()].copy_from_slice(&bytes);
                        *offset += bytes.len();
                        Ok(())
                    }

                    pub const fn measure(_decoded: &$t) -> usize {
                        $t::BITS as usize / 8
                    }
                }
            }
        )*
    };
}

impl_prim!(u16, u32, u64, u128, i8, i16, i32, i64, i128);
