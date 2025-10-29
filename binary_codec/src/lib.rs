use std::{array::TryFromSliceError, mem::MaybeUninit};

#[cfg(feature = "derive")]
pub use binary_codec_derive::{Decode, Encode, Measure};

pub enum DecodeError {
    SliceError(TryFromSliceError),
    InvalidDiscriminant,
}

impl From<TryFromSliceError> for DecodeError {
    fn from(err: TryFromSliceError) -> Self {
        DecodeError::SliceError(err)
    }
}

pub enum EncodeError {
    SliceError,
}

pub trait Decode: Sized {
    fn decode(encoded: &[u8], offset: &mut usize) -> Result<Self, DecodeError>;
}

pub trait Encode {
    fn encode(&self, encoded: &mut [u8], offset: &mut usize) -> Result<(), EncodeError>;
}

pub trait Measure {
    fn measure(&self) -> usize;
}

macro_rules! impl_prim {
    ($($t:tt),+ $(,)?) => {
        $(
            impl Decode for $t {
                fn decode(encoded: &[u8], offset: &mut usize) -> Result<Self, DecodeError> {
                    let n = std::mem::size_of::<$t>();
                    let bytes = &encoded[*offset..*offset + n];
                    *offset += n;
                    Ok(<$t>::from_le_bytes(bytes.try_into()?))
                }
            }

            impl Encode for $t {
                fn encode(&self, encoded: &mut [u8], offset: &mut usize) -> Result<(), EncodeError> {
                    let bytes = self.to_le_bytes();
                    if *offset + bytes.len() > encoded.len() {
                        return Err(EncodeError::SliceError);
                    }
                    encoded[*offset..*offset + bytes.len()].copy_from_slice(&bytes);
                    *offset += bytes.len();
                    Ok(())
                }
            }

            impl Measure for $t {
                fn measure(&self) -> usize {
                    std::mem::size_of::<$t>()
                }
            }
        )*
    };
}

impl_prim!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, f32, f64);


macro_rules! impl_smart_ptr {
    ($($t:tt),+ $(,)?) => {
        $(
            impl<T: Decode> Decode for $t<T> {
                fn decode(encoded: &[u8], offset: &mut usize) -> Result<Self, DecodeError> {
                    let value = T::decode(encoded, offset)?;
                    Ok(Self::new(value))
                }
            }

            impl<T: Encode> Encode for $t<T> {
                fn encode(&self, encoded: &mut [u8], offset: &mut usize) -> Result<(), EncodeError> {
                    self.as_ref().encode(encoded, offset)
                }
            }

            impl<T: Measure> Measure for $t<T> {
                fn measure(&self) -> usize {
                    self.as_ref().measure()
                }
            }
        )*
    };
}

// Note: Rc and Arc are not implemented as they brings special ownership semantics that may not be desired in all contexts.
impl_smart_ptr!(Box);

impl<T, const N: usize> Decode for [T; N]
where
    T: Decode,
{
    fn decode(encoded: &[u8], offset: &mut usize) -> Result<Self, DecodeError> {
        let mut arr: [T; N] = unsafe { MaybeUninit::uninit().assume_init() };
        for i in 0..N {
            arr[i] = T::decode(encoded, offset)?;
        }
        Ok(arr)
    }
}

impl<T, const N: usize> Encode for [T; N]
where
    T: Encode,
{
    fn encode(&self, encoded: &mut [u8], offset: &mut usize) -> Result<(), EncodeError> {
        for item in self.iter() {
            item.encode(encoded, offset)?;
        }
        Ok(())
    }
}

impl<T, const N: usize> Measure for [T; N]
where
    T: Measure,
{
    fn measure(&self) -> usize {
        self.iter().map(|item| item.measure()).sum()
    }
}
