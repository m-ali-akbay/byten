use crate::{DecodeError, Decoder, EncodeError, Encoder, FixedMeasurer, Measurer};

pub struct Array<Item, const N: usize>(pub Item);

impl<Item, const N: usize> Array<Item, N> {
    pub fn codec(item: Item) -> Self {
        Self(item)
    }
}

impl<Item, const N: usize> Default for Array<Item, N>
where
    Item: Default,
{
    fn default() -> Self { Self::codec(Item::default()) }
}

impl<'encoded, 'decoded, Item, const N: usize> Decoder<'encoded, 'decoded> for Array<Item, N>
where
    Item: Decoder<'encoded, 'decoded>,
{
    type Decoded = [Item::Decoded; N];

    fn decode(&self, encoded: &'encoded [u8], offset: &mut usize) -> Result<Self::Decoded, DecodeError> {
        let mut array: heapless::Vec<Item::Decoded, N> = heapless::Vec::new();
        for _ in 0..N {
            let item = self.0.decode(encoded, offset)?;
            array.push(item).unwrap_or_else(|_| panic!("unexpected heapless vec overflow"));
        }
        let array = array.into_array().unwrap_or_else(|_| panic!("unexpected heapless vec underflow"));
        Ok(array)
    }
}

impl<Item, const N: usize> Encoder for Array<Item, N>
where
    Item: Encoder,
    Item::Decoded: Sized,
{
    type Decoded = [Item::Decoded; N];

    fn encode(&self, decoded: &Self::Decoded, encoded: &mut [u8], offset: &mut usize) -> Result<(), EncodeError> {
        for item in decoded.iter() {
            self.0.encode(item, encoded, offset)?;
        }
        Ok(())
    }
}

impl<Item, const N: usize> FixedMeasurer for Array<Item, N>
where
    Item: FixedMeasurer,
    Item::Decoded: Sized,
{
    fn measure_fixed(&self) -> usize {
        N * self.0.measure_fixed()
    }
}

impl<Item, const N: usize> Measurer for Array<Item, N>
where
    Item: Measurer,
    Item::Decoded: Sized,
{
    type Decoded = [Item::Decoded; N];

    fn measure(&self, decoded: &Self::Decoded) -> Result<usize, EncodeError> {
        let mut total = 0;
        for item in decoded.iter() {
            total += self.0.measure(item)?;
        }
        Ok(total)
    }
}
