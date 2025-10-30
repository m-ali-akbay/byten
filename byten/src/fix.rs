use crate::{Decode, DecodeError, Encode, EncodeError, Measure};

// Generic

pub struct Array<T, const N: usize>(pub [T; N]);

impl<T: Decode, const N: usize> Decode for Array<T, N> {
    fn decode(encoded: &[u8], offset: &mut usize) -> Result<Self, DecodeError> {
        let mut array: heapless::Vec<T, N> = heapless::Vec::new();
        for _ in 0..N {
            let item = T::decode(encoded, offset)?;
            array.push(item).unwrap_or_else(|_| panic!("unexpected heapless vec overflow"));
        }
        let array = array.into_array().unwrap_or_else(|_| panic!("unexpected heapless vec underflow"));
        Ok(Array(array))
    }
}

impl<T: Encode, const N: usize> Encode for Array<T, N> {
    fn encode(&self, encoded: &mut [u8], offset: &mut usize) -> Result<(), EncodeError> {
        for item in self.0.iter() {
            item.encode(encoded, offset)?;
        }
        Ok(())
    }
}

impl<T: Measure, const N: usize> Measure for Array<T, N> {
    fn measure(&self) -> usize {
        self.0.iter().map(|item| item.measure()).sum()
    }
}

impl<T, const N: usize, U: Into<T>> From<[U; N]> for Array<T, N> {
    fn from(array: [U; N]) -> Self {
        let mut heapless_array: heapless::Vec<T, N> = heapless::Vec::new();
        for item in array.into_iter() {
            heapless_array.push(item.into()).unwrap_or_else(|_| panic!("unexpected heapless vec overflow"));
        }
        let heapless_array = heapless_array.into_array().unwrap_or_else(|_| panic!("unexpected heapless vec underflow"));
        Array(heapless_array)
    }
}

impl<T, const N: usize, U: From<T>> From<Array<T, N>> for [U; N] {
    fn from(array: Array<T, N>) -> Self {
        let mut heapless_array: heapless::Vec<U, N> = heapless::Vec::new();
        for item in array.0.into_iter() {
            heapless_array.push(U::from(item)).unwrap_or_else(|_| panic!("unexpected heapless vec overflow"));
        }
        heapless_array.into_array().unwrap_or_else(|_| panic!("unexpected heapless vec underflow"))
    }
}

// u8

pub struct U8Array<const N: usize>(pub [u8; N]);

impl<const N: usize> Decode for U8Array<N> {
    fn decode(encoded: &[u8], offset: &mut usize) -> Result<Self, DecodeError> {
        if *offset + N > encoded.len() {
            return Err(DecodeError::EOF);
        }
        let mut array = [0u8; N];
        array.copy_from_slice(&encoded[*offset..*offset + N]);
        *offset += N;
        Ok(U8Array(array))
    }
}

impl<const N: usize> Encode for U8Array<N> {
    fn encode(&self, encoded: &mut [u8], offset: &mut usize) -> Result<(), EncodeError> {
        if *offset + N > encoded.len() {
            return Err(EncodeError::BufferTooSmall);
        }
        encoded[*offset..*offset + N].copy_from_slice(&self.0);
        *offset += N;
        Ok(())
    }
}

impl<const N: usize> Measure for U8Array<N> {
    fn measure(&self) -> usize { N }
}

impl<const N: usize> From<[u8; N]> for U8Array<N> {
    fn from(array: [u8; N]) -> Self {
        U8Array(array)
    }
}

impl<const N: usize> From<U8Array<N>> for [u8; N] {
    fn from(array: U8Array<N>) -> Self {
        array.0
    }
}
