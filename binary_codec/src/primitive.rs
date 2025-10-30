macro_rules! impl_prim {
    ($ty:tt, $wrapper:ident, $from_bytes:ident, $to_bytes:ident) => {
        pub struct $wrapper(pub $ty);

        impl crate::Decode for $wrapper {
            fn decode(encoded: &[u8], offset: &mut usize) -> Result<Self, crate::DecodeError> {
                const SIZE: usize = $ty::BITS as usize / 8;
                if *offset + SIZE > encoded.len() {
                    return Err(crate::DecodeError::EOF);
                }
                let bytes: [u8; SIZE] = encoded[*offset..*offset + SIZE].try_into().unwrap();
                *offset += SIZE;
                Ok(<$ty>::$from_bytes(bytes).into())
            }
        }

        impl crate::Encode for $wrapper {
            fn encode(&self, encoded: &mut [u8], offset: &mut usize) -> Result<(), crate::EncodeError> {
                const SIZE: usize = $ty::BITS as usize / 8;
                if *offset + SIZE > encoded.len() {
                    return Err(crate::EncodeError::BufferTooSmall);
                }
                let bytes = self.0.$to_bytes();
                encoded[*offset..*offset + SIZE].copy_from_slice(&bytes);
                *offset += SIZE;
                Ok(())
            }
        }

        impl crate::Measure for $wrapper {
            fn measure(&self) -> usize {
                $ty::BITS as usize / 8
            }
        }

        impl From<$ty> for $wrapper {
            fn from(value: $ty) -> Self {
                $wrapper(value)
            }
        }

        impl From<$wrapper> for $ty {
            fn from(value: $wrapper) -> Self {
                value.0
            }
        }

        impl From<&$ty> for $wrapper {
            fn from(value: &$ty) -> Self {
                $wrapper(*value)
            }
        }
    };
}

impl_prim!(u16, U16BE, from_be_bytes, to_be_bytes);
impl_prim!(u32, U32BE, from_be_bytes, to_be_bytes);
impl_prim!(u64, U64BE, from_be_bytes, to_be_bytes);
impl_prim!(u128, U128BE, from_be_bytes, to_be_bytes);
impl_prim!(i16, I16BE, from_be_bytes, to_be_bytes);
impl_prim!(i32, I32BE, from_be_bytes, to_be_bytes);
impl_prim!(i64, I64BE, from_be_bytes, to_be_bytes);
impl_prim!(i128, I128BE, from_be_bytes, to_be_bytes);

impl_prim!(u16, U16LE, from_be_bytes, to_be_bytes);
impl_prim!(u32, U32LE, from_be_bytes, to_be_bytes);
impl_prim!(u64, U64LE, from_be_bytes, to_be_bytes);
impl_prim!(u128, U128LE, from_be_bytes, to_be_bytes);
impl_prim!(i16, I16LE, from_be_bytes, to_be_bytes);
impl_prim!(i32, I32LE, from_be_bytes, to_be_bytes);
impl_prim!(i64, I64LE, from_be_bytes, to_be_bytes);
impl_prim!(i128, I128LE, from_be_bytes, to_be_bytes);

