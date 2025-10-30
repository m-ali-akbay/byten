use std::mem::MaybeUninit;

use crate::{Decode, DecodeError, Encode, EncodeError, Measure};

pub struct Array<T, const N: usize>(pub [T; N]);

impl<T: Decode, const N: usize> Decode for Array<T, N> {
    fn decode(encoded: &[u8], offset: &mut usize) -> Result<Self, DecodeError> {
        let mut array: [MaybeUninit<T>; N] = unsafe { MaybeUninit::uninit().assume_init() };
        for i in 0..N {
            let item = T::decode(encoded, offset)?;
            array[i] = MaybeUninit::new(item);
        }
        let initialized_array = unsafe { std::mem::transmute_copy::<_, [T; N]>(&array) };
        Ok(Array(initialized_array))
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

impl<T, const N: usize, U: From<T>> From<Array<T, N>> for [U; N] {
    fn from(array: Array<T, N>) -> Self {
        array.0.map(|item| item.into())
    }
}

impl<T: Clone, const N: usize, U: From<T>> From<&[T; N]> for Array<U, N> {
    fn from(array: &[T; N]) -> Self {
        let mut result: [MaybeUninit<U>; N] = unsafe { MaybeUninit::uninit().assume_init() };
        for i in 0..N {
            result[i] = MaybeUninit::new(array[i].clone().into());
        }
        Array(unsafe { std::mem::transmute_copy::<_, [U; N]>(&result) })
    }
}
