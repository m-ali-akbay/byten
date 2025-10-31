pub mod fix;
pub mod prelude;
pub mod prim;
pub mod util;
pub mod var;

use std::{convert::Infallible, num::TryFromIntError};

#[cfg(feature = "derive")]
pub use byten_derive::{Decode, Encode, Measure, FixedMeasure};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum DecodeError {
    #[error("End of file reached")]
    EOF,

    #[error("Invalid discriminant")]
    InvalidDiscriminant,

    #[error("Invalid usize")]
    InvalidUSize,

    #[error("Data conversion failure")]
    ConversionFailure,

    #[error("Invalid data")]
    InvalidData,

    #[error("Codec failure")]
    CodecFailure,
    
    #[cfg(feature = "anyhow")]
    #[error("Anyhow: {0}")]
    Anyhow(#[from] anyhow::Error),
}

impl From<Infallible> for DecodeError {
    fn from(_: Infallible) -> Self {
        unreachable!()
    }
}

impl From<TryFromIntError> for DecodeError {
    fn from(_: TryFromIntError) -> Self {
        DecodeError::CodecFailure
    }
}

#[derive(Error, Debug)]
pub enum EncodeError {
    #[error("Buffer too small")]
    BufferTooSmall,

    #[error("Invalid usize")]
    InvalidUSize,

    #[error("Data conversion failure")]
    CodecFailure,

    #[cfg(feature = "anyhow")]
    #[error("Anyhow: {0}")]
    Anyhow(#[from] anyhow::Error),
}

impl From<Infallible> for EncodeError {
    fn from(_: Infallible) -> Self {
        unreachable!()
    }
}

impl From<TryFromIntError> for EncodeError {
    fn from(_: TryFromIntError) -> Self {
        EncodeError::CodecFailure
    }
}

// codec traits

pub trait Encoder {
    type Decoded;
    fn encode(&self, decoded: &Self::Decoded, encoded: &mut [u8], offset: &mut usize) -> Result<(), EncodeError>;
}

pub trait Decoder {
    type Decoded;
    fn decode(&self, encoded: &[u8], offset: &mut usize) -> Result<Self::Decoded, DecodeError>;
}

pub trait Measurer {
    type Decoded;
    fn measure(&self, decoded: &Self::Decoded) -> Result<usize, EncodeError>;
}

pub trait FixedMeasurer: Measurer {
    fn fixed_measure(&self) -> usize;
}

// self-codecs

pub trait Decode: Sized {
    fn decode(encoded: &[u8], offset: &mut usize) -> Result<Self, DecodeError>;
}

pub trait Encode {
    fn encode(&self, encoded: &mut [u8], offset: &mut usize) -> Result<(), EncodeError>;
}

pub trait Measure {
    fn measure(&self) -> Result<usize, EncodeError>;
}

pub trait FixedMeasure: Measure {
    fn fixed_measure() -> usize;
}

pub struct SelfCodec<T> {
    _marker: core::marker::PhantomData<T>,
}

impl<T> SelfCodec<T> {
    pub const fn codec() -> Self {
        SelfCodec {
            _marker: core::marker::PhantomData,
        }
    }
}

impl<T> Default for SelfCodec<T> {
    fn default() -> Self { Self::codec() }
}

impl<T: Decode> Decoder for SelfCodec<T> {
    type Decoded = T;
    fn decode(&self, encoded: &[u8], offset: &mut usize) -> Result<Self::Decoded, DecodeError> {
        T::decode(encoded, offset)
    }
}

impl<T: Encode> Encoder for SelfCodec<T> {
    type Decoded = T;
    fn encode(&self, decoded: &Self::Decoded, encoded: &mut [u8], offset: &mut usize) -> Result<(), EncodeError> {
        decoded.encode(encoded, offset)
    }
}

impl<T: Measure> Measurer for SelfCodec<T> {
    type Decoded = T;
    fn measure(&self, decoded: &Self::Decoded) -> Result<usize, EncodeError> {
        decoded.measure()
    }
}

impl<T: FixedMeasure> FixedMeasurer for SelfCodec<T> {
    fn fixed_measure(&self) -> usize {
        T::fixed_measure()
    }
}

// very basic implementations

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

impl FixedMeasure for u8 {
    fn fixed_measure() -> usize { 1 }
}

impl Measure for u8 {
    fn measure(&self) -> Result<usize, EncodeError> { Ok(Self::fixed_measure()) }
}

impl<const N: usize> Decode for [u8; N] {
    fn decode(encoded: &[u8], offset: &mut usize) -> Result<Self, DecodeError> {
        if *offset + N > encoded.len() {
            return Err(DecodeError::EOF);
        }
        let mut array = [0u8; N];
        array.copy_from_slice(&encoded[*offset..*offset + N]);
        *offset += N;
        Ok(array)
    }
}

impl<const N: usize> Encode for [u8; N] {
    fn encode(&self, encoded: &mut [u8], offset: &mut usize) -> Result<(), EncodeError> {
        if *offset + N > encoded.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        encoded[*offset..*offset + N].copy_from_slice(self);
        *offset += N;
        Ok(())
    }
}

impl<const N: usize> FixedMeasure for [u8; N] {
    fn fixed_measure() -> usize { N }
}

impl<const N: usize> Measure for [u8; N] {
    fn measure(&self) -> Result<usize, EncodeError> { Ok(Self::fixed_measure()) }
}

impl Decode for bool {
    fn decode(encoded: &[u8], offset: &mut usize) -> Result<Self, DecodeError> {
        let byte = u8::decode(encoded, offset)?;
        match byte {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(DecodeError::InvalidData),
        }
    }
}

impl Encode for bool {
    fn encode(&self, encoded: &mut [u8], offset: &mut usize) -> Result<(), EncodeError> {
        let byte = if *self { 1u8 } else { 0u8 };
        byte.encode(encoded, offset)
    }
}

impl FixedMeasure for bool {
    fn fixed_measure() -> usize { 1 }
}

impl Measure for bool {
    fn measure(&self) -> Result<usize, EncodeError> { Ok(Self::fixed_measure()) }
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

            impl<T: FixedMeasure> FixedMeasure for $t<T> {
                fn fixed_measure() -> usize {
                    T::fixed_measure()
                }
            }

            impl<T: Measure> Measure for $t<T> {
                fn measure(&self) -> Result<usize, EncodeError> {
                    self.as_ref().measure()
                }
            }
        )*
    };
}

// Note: Rc and Arc are not implemented as they brings special ownership semantics that may not be desired in all contexts.
impl_smart_ptr!(Box);
