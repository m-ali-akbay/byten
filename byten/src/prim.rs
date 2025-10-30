macro_rules! impl_prim {
    ($ty:tt, $wrapper:ident, $from_bytes:ident, $to_bytes:ident) => {
        pub struct $wrapper;

        impl Default for $wrapper {
            fn default() -> Self {
                $wrapper
            }
        }

        impl crate::Decoder for $wrapper {
            type Decoded = $ty;
            fn decode(&self, encoded: &[u8], offset: &mut usize) -> Result<Self::Decoded, crate::DecodeError> {
                const SIZE: usize = $ty::BITS as usize / 8;
                if *offset + SIZE > encoded.len() {
                    return Err(crate::DecodeError::EOF);
                }
                let bytes: [u8; SIZE] = encoded[*offset..*offset + SIZE].try_into().unwrap();
                *offset += SIZE;
                Ok(<$ty>::$from_bytes(bytes))
            }
        }

        impl crate::Encoder for $wrapper {
            type Decoded = $ty;
            fn encode(&self, decoded: &Self::Decoded, encoded: &mut [u8], offset: &mut usize) -> Result<(), crate::EncodeError> {
                const SIZE: usize = $ty::BITS as usize / 8;
                if *offset + SIZE > encoded.len() {
                    return Err(crate::EncodeError::BufferTooSmall);
                }
                let bytes = decoded.$to_bytes();
                encoded[*offset..*offset + SIZE].copy_from_slice(&bytes);
                *offset += SIZE;
                Ok(())
            }
        }

        impl crate::Measurer for $wrapper {
            type Decoded = $ty;
            fn measure(&self, _decoded: &Self::Decoded) -> usize {
                $ty::BITS as usize / 8
            }
        }
    };
}

// BE
impl_prim!(u16, U16BE, from_be_bytes, to_be_bytes);
impl_prim!(u32, U32BE, from_be_bytes, to_be_bytes);
impl_prim!(u64, U64BE, from_be_bytes, to_be_bytes);
impl_prim!(u128, U128BE, from_be_bytes, to_be_bytes);
impl_prim!(i16, I16BE, from_be_bytes, to_be_bytes);
impl_prim!(i32, I32BE, from_be_bytes, to_be_bytes);
impl_prim!(i64, I64BE, from_be_bytes, to_be_bytes);
impl_prim!(i128, I128BE, from_be_bytes, to_be_bytes);

// LE
impl_prim!(u16, U16LE, from_le_bytes, to_le_bytes);
impl_prim!(u32, U32LE, from_le_bytes, to_le_bytes);
impl_prim!(u64, U64LE, from_le_bytes, to_le_bytes);
impl_prim!(u128, U128LE, from_le_bytes, to_le_bytes);
impl_prim!(i16, I16LE, from_le_bytes, to_le_bytes);
impl_prim!(i32, I32LE, from_le_bytes, to_le_bytes);
impl_prim!(i64, I64LE, from_le_bytes, to_le_bytes);
impl_prim!(i128, I128LE, from_le_bytes, to_le_bytes);
