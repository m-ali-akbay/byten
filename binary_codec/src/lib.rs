pub mod primitive;
pub mod array;

use std::mem::MaybeUninit;

#[cfg(feature = "derive")]
pub use byten_derive::{Decode, Encode, Measure};

pub enum DecodeError {
    EOF,
    InvalidDiscriminant,
}

pub enum EncodeError {
    BufferTooSmall,
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

impl Decode for u8 {
    fn decode(encoded: &[u8], offset: &mut usize) -> Result<Self, DecodeError> {
        if *offset >= encoded.len() {
            return Err(DecodeError::EOF);
        }
        let value = encoded[*offset];
        *offset += 1;
        Ok(value)
    }
}

impl Encode for u8 {
    fn encode(&self, encoded: &mut [u8], offset: &mut usize) -> Result<(), EncodeError> {
        if *offset >= encoded.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        encoded[*offset] = *self;
        *offset += 1;
        Ok(())
    }
}

impl Measure for u8 {
    fn measure(&self) -> usize {
        1
    }
}

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
