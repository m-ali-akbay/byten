use std::{iter, vec::Vec as StdVec};

use crate::Decoder;
use crate::Encode as _;
use crate::Decode as _;

pub struct Vec<Length, Item> {
    pub length: Length,
    pub item: Item,
}

impl<Length, Item> Default for Vec<Length, Item>
where
    Length: Default,
    Item: Default,
{
    fn default() -> Self {
        Vec {
            length: Length::default(),
            item: Item::default(),
        }
    }
}

impl<Length, Item> Decoder for Vec<Length, Item>
where
    Length: crate::Decoder<Decoded = usize>,
    Item: crate::Decoder,
{
    type Decoded = StdVec<Item::Decoded>;

    fn decode(&self, encoded: &[u8], offset: &mut usize) -> Result<Self::Decoded, crate::DecodeError> {
        let size = self.length.decode(encoded, offset)?;
        let mut vec = StdVec::with_capacity(size);
        for _ in 0..size {
            let item = self.item.decode(encoded, offset)?;
            vec.push(item);
        }
        Ok(vec)
    }
}

impl<Length, Item> crate::Encoder for Vec<Length, Item>
where
    Length: crate::Encoder<Decoded = usize>,
    Item: crate::Encoder,
{
    type Decoded = StdVec<Item::Decoded>;

    fn encode(&self, decoded: &Self::Decoded, encoded: &mut [u8], offset: &mut usize) -> Result<(), crate::EncodeError> {
        let size = decoded.len();
        self.length.encode(&size, encoded, offset)?;
        for item in decoded.iter() {
            self.item.encode(item, encoded, offset)?;
        }
        Ok(())
    }
}

impl<Length, Item> crate::Measurer for Vec<Length, Item>
where
    Length: crate::Measurer<Decoded = usize>,
    Item: crate::Measurer,
{
    type Decoded = StdVec<Item::Decoded>;

    fn measure(&self, decoded: &Self::Decoded) -> usize {
        let size = decoded.len();
        let size_measure = self.length.measure(&size);
        let items_measure: usize = decoded.iter().map(|item| self.item.measure(item)).sum();
        size_measure + items_measure
    }
}

#[derive(Copy, Clone)]
pub struct U64BE;

impl Default for U64BE {
    fn default() -> Self {
        U64BE
    }
}

impl U64BE {
    fn into_septets_le(num: u64) -> [u8; 10] {
        let mut bits_from_lsb = (0..64).into_iter().map(move |bit| num & (1 << bit) != 0);

        let septets_le = iter::from_fn(|| {
            let mut septet = 0u8;
            for bit_index in (0..7).rev() {
                if let Some(bit) = bits_from_lsb.next() {
                    if bit {
                        septet |= 1 << bit_index;
                    }
                } else {
                    return None;
                }
            }
            Some(septet)
        });

        let mut array = [0u8; 10];
        for (index, septet) in septets_le.enumerate() {
            array[index] = septet;
        }

        array
    }

    fn into_septets_be(num: u64) -> [u8; 10] {
        let mut septets_be = Self::into_septets_le(num);
        septets_be.reverse();
        septets_be
    }

    fn from_septets_le(septets_le: [u8; 10]) -> u64 {
        let septets_le = septets_le.into_iter();
        let bits_from_lsb = septets_le.flat_map(|septet| {
            (0..7).rev().map(move |bit_index| (septet & (1 << bit_index)) != 0)
        }).take(64);
        let mut value = 0u64;
        for (bit_index, bit) in bits_from_lsb.enumerate() {
            if bit {
                value |= 1 << bit_index;
            }
        }
        value
    }
}

impl crate::Encoder for U64BE {
    type Decoded = u64;
    fn encode(&self, &decoded: &u64, encoded: &mut [u8], offset: &mut usize) -> Result<(), crate::EncodeError> {
        let septets_be = Self::into_septets_be(decoded);
        let skip = septets_be.iter().rev().take_while(|&&b| b == 0).count();
        let trunc_septets_be = &septets_be[0..septets_be.len() - skip];

        if trunc_septets_be.is_empty() {
            return 0u8.encode(encoded, offset);
        }

        for &septet in &trunc_septets_be[0..trunc_septets_be.len() - 1] {
            (septet | 0x80).encode(encoded, offset)?;
        }

        trunc_septets_be[trunc_septets_be.len() - 1].encode(encoded, offset)?;

        Ok(())
    }
}

impl crate::Decoder for U64BE {
    type Decoded = u64;

    fn decode(&self, encoded: &[u8], offset: &mut usize) -> Result<u64, crate::DecodeError> {
        let mut septets_be = heapless::Vec::<u8, 10>::new();

        loop {
            let flagged_septet = u8::decode(encoded, offset)?;
            let septet = flagged_septet & 0x7F;

            septets_be.push(septet).map_err(|_| crate::DecodeError::InvalidData)?;

            if flagged_septet & 0x80 == 0 {
                break;
            }
        }

        let mut septets_le_array = [0u8; 10];
        for (i, &septet) in septets_be.iter().rev().enumerate() {
            septets_le_array[i] = septet;
        }

        Ok(U64BE::from_septets_le(septets_le_array))
    }
}

impl crate::Measurer for U64BE {
    type Decoded = u64;
    fn measure(&self, &decoded: &u64) -> usize {
        let septets_be = Self::into_septets_be(decoded);
        let skip = septets_be.iter().rev().take_while(|&&b| b == 0).count();
        let take = septets_be.len() - skip;
        take.max(1)
    }
}

pub struct USizeBE;

impl Default for USizeBE {
    fn default() -> Self {
        USizeBE
    }
}

impl crate::Decoder for USizeBE {
    type Decoded = usize;
    fn decode(&self, encoded: &[u8], offset: &mut usize) -> Result<usize, crate::DecodeError> {
        let u64_value = U64BE.decode(encoded, offset)?;
        Ok(u64_value.try_into().map_err(|_| crate::DecodeError::ConversionFailure)?)
    }
}

impl crate::Encoder for USizeBE {
    type Decoded = usize;
    fn encode(&self, &decoded: &usize, encoded: &mut [u8], offset: &mut usize) -> Result<(), crate::EncodeError> {
        let u64_value = decoded as u64;
        U64BE.encode(&u64_value, encoded, offset)
    }
}

impl crate::Measurer for USizeBE {
    type Decoded = usize;
    fn measure(&self, &decoded: &usize) -> usize {
        let u64_value = decoded as u64;
        U64BE.measure(&u64_value)
    }
}
