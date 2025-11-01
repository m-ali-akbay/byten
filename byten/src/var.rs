use std::{iter, vec::Vec as StdVec, option::Option as StdOption};

use crate::Encode as _;
use crate::Decode as _;
use crate::Measure as _;

pub mod str;

pub struct Vec<Length, Item> {
    pub length: Length,
    pub item: Item,
}

impl<Length, Item> Vec<Length, Item> {
    pub const fn codec(length: Length, item: Item) -> Self {
        Self { length, item }
    }
}

impl<Length, Item> Default for Vec<Length, Item>
where
    Length: Default,
    Item: Default,
{
    fn default() -> Self {
        Self::codec(Length::default(), Item::default())
    }
}

impl<'encoded, 'decoded, Length, Item> crate::Decoder<'encoded, 'decoded> for Vec<Length, Item>
where
    Length: crate::Decoder<'encoded, 'decoded, Decoded = usize>,
    Item: crate::Decoder<'encoded, 'decoded>,
{
    type Decoded = StdVec<Item::Decoded>;

    fn decode(&self, encoded: &'encoded [u8], offset: &mut usize) -> Result<Self::Decoded, crate::DecodeError> {
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
    Item::Decoded: Sized,
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
    Item::Decoded: Sized,
{
    type Decoded = StdVec<Item::Decoded>;

    fn measure(&self, decoded: &Self::Decoded) -> Result<usize, crate::EncodeError> {
        let size = decoded.len();
        let size_measure = self.length.measure(&size)?;
        let mut items_measure = 0;
        for item in decoded.iter() {
            items_measure += self.item.measure(item)?;
        }
        Ok(size_measure + items_measure)
    }
}

pub struct Remaining;

impl Remaining {
    pub const fn codec() -> Self {
        Remaining
    }
}

impl Default for Remaining {
    fn default() -> Self { Self::codec() }
}

impl<'encoded, 'decoded> crate::Decoder<'encoded, 'decoded> for Remaining
where
    'encoded: 'decoded,
{
    type Decoded = &'decoded [u8];

    fn decode(&self, encoded: &'encoded [u8], offset: &mut usize) -> Result<Self::Decoded, crate::DecodeError> {
        if *offset > encoded.len() {
            return Err(crate::DecodeError::InvalidData);
        }
        let remaining = &encoded[*offset..];
        *offset = encoded.len();
        Ok(remaining)
    }
}

impl crate::Encoder for Remaining {
    type Decoded = [u8];

    fn encode(&self, decoded: &Self::Decoded, encoded: &mut [u8], offset: &mut usize) -> Result<(), crate::EncodeError> {
        let end = *offset + decoded.len();
        if end > encoded.len() {
            return Err(crate::EncodeError::BufferTooSmall);
        }
        encoded[*offset..end].copy_from_slice(decoded);
        *offset = end;
        Ok(())
    }
}

impl crate::Measurer for Remaining {
    type Decoded = [u8];

    fn measure(&self, decoded: &Self::Decoded) -> Result<usize, crate::EncodeError> {
        Ok(decoded.len())
    }
}

#[derive(Copy, Clone)]
pub struct U64BE;

impl U64BE {
    pub const fn codec() -> Self {
        U64BE
    }
}

impl Default for U64BE {
    fn default() -> Self { Self::codec() }
}

impl U64BE {
    fn into_septets_le(num: u64) -> [u8; 10] {
        let mut bits_from_lsb = (0..64).map(move |bit| num & (1 << bit) != 0);

        let septets_le = iter::from_fn(|| {
            let mut septet = 0u8;
            for bit_index in 0..7 {
                let Some(bit) = bits_from_lsb.next() else {
                    break;
                };
                if bit {
                    septet |= 1 << bit_index;
                }
            }
            Some(septet)
        }).take(10);

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
            (0..7).map(move |bit_index| (septet & (1 << bit_index)) != 0)
        }).take(64);
        let mut value = 0u64;
        for (bit_index, bit) in bits_from_lsb.enumerate() {
            if bit {
                value |= 1 << bit_index;
            }
        }
        value
    }

    #[allow(dead_code)]
    fn from_septets_be(septets_be: [u8; 10]) -> u64 {
        let mut septets_le = septets_be;
        septets_le.reverse();
        Self::from_septets_le(septets_le)
    }
}

impl crate::Encoder for U64BE {
    type Decoded = u64;
    fn encode(&self, &decoded: &u64, encoded: &mut [u8], offset: &mut usize) -> Result<(), crate::EncodeError> {
        let septets_be = Self::into_septets_be(decoded);
        let skip = septets_be.iter().take_while(|&&b| b == 0).count();
        let trunc_septets_be = &septets_be[skip..];

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

impl<'encoded> crate::Decoder<'encoded, '_> for U64BE {
    type Decoded = u64;

    fn decode(&self, encoded: &'encoded [u8], offset: &mut usize) -> Result<u64, crate::DecodeError> {
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
    fn measure(&self, &decoded: &u64) -> Result<usize, crate::EncodeError> {
        let septets_be = Self::into_septets_be(decoded);
        let skip = septets_be.iter().take_while(|&&b| b == 0).count();
        let take = septets_be.len() - skip;
        Ok(take.max(1))
    }
}

#[cfg(test)]
mod test {
    use crate::prelude::EncoderToVec;
    use crate::Decoder as _;

    use super::*;

    #[test]
    fn test_septets_le() {
        let fixtures = [
            (0u64,      [0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000]),
            (1u64,      [0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000001]),
            (127u64,    [0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b1111111]),
            (128u64,    [0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000001, 0b0000000]),
            (255u64,    [0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000001, 0b1111111]),
            (16383u64,  [0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b1111111, 0b1111111]),
            (16384u64,  [0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000000, 0b0000001, 0b0000000, 0b0000000]),
            (u64::MAX,  [0b0000001, 0b1111111, 0b1111111, 0b1111111, 0b1111111, 0b1111111, 0b1111111, 0b1111111, 0b1111111, 0b1111111]),
        ];
        for (num, septets_be_fixture) in fixtures.iter() {
            let septets_be = U64BE::into_septets_be(*num);
            assert_eq!(&septets_be, septets_be_fixture, "BE septets failed for {}", num);

            let reconstructed_num = U64BE::from_septets_be(*septets_be_fixture);
            assert_eq!(&reconstructed_num, num, "BE reconstruction failed for {:?}", septets_be_fixture);
        }
    }
    
    #[test]
    fn test_u64be() {
        let fixtures = [
            (0u64,      vec![0b00000000]),
            (1u64,      vec![0b00000001]),
            (127u64,    vec![0b01111111]),
            (128u64,    vec![0b10000001, 0b00000000]),
            (255u64,    vec![0b10000001, 0b01111111]),
            (16383u64,  vec![0b11111111, 0b01111111]),
            (16384u64,  vec![0b10000001, 0b10000000, 0b00000000]),
            (u64::MAX,  vec![0b10000001, 0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b01111111]),
        ];

        for (num, encoded_fixture) in fixtures.iter() {
            let encoded = U64BE.encode_to_vec(num).expect("Encoding failed");
            assert_eq!(&encoded, encoded_fixture, "Encoding failed for {}", num);

            let decoded = U64BE.decode(&encoded, &mut 0).expect("Decoding failed");
            assert_eq!(&decoded, num, "Decoding failed for {:?}", encoded);
        }
    }
}

macro_rules! define_u_be {
    ($name:ident, $ty:ty) => {
        pub struct $name;

        impl $name {
            pub const fn codec() -> Self {
                $name
            }
        }

        impl Default for $name {
            fn default() -> Self { Self::codec() }
        }

        impl<'encoded> crate::Decoder<'encoded, '_> for $name {
            type Decoded = $ty;
            fn decode(&self, encoded: &'encoded [u8], offset: &mut usize) -> Result<$ty, crate::DecodeError> {
                let u64_value = U64BE.decode(encoded, offset)?;
                let val: $ty = u64_value.try_into().map_err(|_| crate::DecodeError::ConversionFailure)?;
                Ok(val)
            }
        }

        impl crate::Encoder for $name {
            type Decoded = $ty;
            fn encode(&self, decoded: &Self::Decoded, encoded: &mut [u8], offset: &mut usize) -> Result<(), crate::EncodeError> {
                let u64_value = *decoded as u64;
                U64BE.encode(&u64_value, encoded, offset)
            }
        }

        impl crate::Measurer for $name {
            type Decoded = $ty;
            fn measure(&self, decoded: &Self::Decoded) -> Result<usize, crate::EncodeError> {
                let u64_value = *decoded as u64;
                U64BE.measure(&u64_value)
            }
        }
    };
}

define_u_be!(USizeBE, usize);
define_u_be!(U32BE, u32);
define_u_be!(U16BE, u16);

pub struct Option<Item> {
    pub item: Item,
}

impl<Item> Option<Item> {
    pub fn codec(item: Item) -> Self {
        Self { item }
    }
}

impl<Item> Default for Option<Item>
where
    Item: Default,
{
    fn default() -> Self { Self::codec(Item::default()) }
}

impl<'encoded, 'decoded, Item> crate::Decoder<'encoded, 'decoded> for Option<Item>
where
    Item: crate::Decoder<'encoded, 'decoded>,
{
    type Decoded = StdOption<Item::Decoded>;

    fn decode(&self, encoded: &'encoded [u8], offset: &mut usize) -> Result<Self::Decoded, crate::DecodeError> {
        let flag = bool::decode(encoded, offset)?;
        if flag {
            Ok(StdOption::None)
        } else {
            let item = self.item.decode(encoded, offset)?;
            Ok(StdOption::Some(item))
        }
    }
}

impl<Item> crate::Encoder for Option<Item>
where
    Item: crate::Encoder,
    Item::Decoded: Sized,
{
    type Decoded = StdOption<Item::Decoded>;

    fn encode(&self, decoded: &Self::Decoded, encoded: &mut [u8], offset: &mut usize) -> Result<(), crate::EncodeError> {
        match decoded {
            StdOption::None => {
                true.encode(encoded, offset)?;
                Ok(())
            }
            StdOption::Some(item) => {
                false.encode(encoded, offset)?;
                self.item.encode(item, encoded, offset)
            }
        }
    }
}

impl<Item> crate::Measurer for Option<Item>
where
    Item: crate::Measurer,
    Item::Decoded: Sized,
{
    type Decoded = StdOption<Item::Decoded>;

    fn measure(&self, decoded: &Self::Decoded) -> Result<usize, crate::EncodeError> {
        Ok(match decoded {
            StdOption::None => true.measure()?,
            StdOption::Some(item) => {
                false.measure()?
                + self.item.measure(item)?
            }
        })
    }
}


pub struct Slice<Length> {
    pub length: Length,
}

impl<Length> Slice<Length> {
    pub const fn codec(length: Length) -> Self {
        Self {
            length,
        }
    }
}

impl<Length> Default for Slice<Length>
where
    Length: Default,
{
    fn default() -> Self {
        Self::codec(Length::default())
    }
}

impl<'encoded, 'decoded, Length> crate::Decoder<'encoded, 'decoded> for Slice<Length>
where
    Length: for<'length> crate::Decoder<'encoded, 'length, Decoded = usize>,
    'encoded: 'decoded,
{
    type Decoded = &'decoded [u8];

    fn decode(&self, encoded: &'encoded [u8], offset: &mut usize) -> Result<Self::Decoded, crate::DecodeError> {
        let size = self.length.decode(encoded, offset)?;
        if *offset + size > encoded.len() {
            return Err(crate::DecodeError::InvalidData);
        }
        let buffer = &encoded[*offset..*offset + size];
        *offset += size;
        Ok(buffer)
    }
}

impl<Length> crate::Encoder for Slice<Length>
where
    Length: crate::Encoder<Decoded = usize>,
{
    type Decoded = [u8];

    fn encode(&self, decoded: &Self::Decoded, encoded: &mut [u8], offset: &mut usize) -> Result<(), crate::EncodeError> {
        let size = decoded.len();
        self.length.encode(&size, encoded, offset)?;
        let end = *offset + size;
        if end > encoded.len() {
            return Err(crate::EncodeError::BufferTooSmall);
        }
        encoded[*offset..end].copy_from_slice(decoded);
        *offset = end;
        Ok(())
    }
}

impl<Length> crate::Measurer for Slice<Length>
where
    Length: crate::Measurer<Decoded = usize>,
{
    type Decoded = [u8];

    fn measure(&self, decoded: &Self::Decoded) -> Result<usize, crate::EncodeError> {
        let size = decoded.len();
        let size_measure = self.length.measure(&size)?;
        Ok(size_measure + size)
    }
}
