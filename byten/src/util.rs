use core::marker::PhantomData;

pub struct Convert<Codec, Decoded> {
    pub codec: Codec,
    pub _marker: PhantomData<Decoded>,
}

impl<Codec, Decoded> Convert<Codec, Decoded> {
    pub const fn codec(codec: Codec) -> Self {
        Self {
            codec,
            _marker: PhantomData,
        }
    }
}

impl<Codec, Decoded> Default for Convert<Codec, Decoded>
where
    Codec: Default,
{
    fn default() -> Self { Self::codec(Codec::default()) }
}

impl<Codec, Decoded, Error> crate::Decoder for Convert<Codec, Decoded>
where
    Codec: crate::Decoder,
    Codec::Decoded: TryInto<Decoded, Error = Error>,
    Error: Into<crate::DecodeError>,
{
    type Decoded = Decoded;
    fn decode(&self, encoded: &[u8], offset: &mut usize) -> Result<Self::Decoded, crate::DecodeError> {
        let intermediate = self.codec.decode(encoded, offset)?;
        let decoded = intermediate.try_into().map_err(Into::into)?;
        Ok(decoded)
    }
}

impl<Codec, Decoded, Error> crate::Encoder for Convert<Codec, Decoded>
where
    Codec: crate::Encoder,
    Decoded: Clone,
    Codec::Decoded: TryFrom<Decoded, Error = Error>,
    Error: Into<crate::EncodeError>,
{
    type Decoded = Decoded;
    fn encode(
        &self,
        decoded: &Self::Decoded,
        encoded: &mut [u8],
        offset: &mut usize,
    ) -> Result<(), crate::EncodeError> {
        let intermediate = decoded.clone().try_into().map_err(Into::into)?;
        self.codec.encode(&intermediate, encoded, offset)
    }
}

impl<Codec, Decoded, Error> crate::Measurer for Convert<Codec, Decoded>
where
    Codec: crate::Measurer,
    Decoded: Clone,
    Codec::Decoded: TryFrom<Decoded, Error = Error>,
    Error: Into<crate::EncodeError>,
{
    type Decoded = Decoded;
    fn measure(&self, decoded: &Self::Decoded) -> Result<usize, crate::EncodeError> {
        let intermediate = decoded.clone().try_into().map_err(Into::into)?;
        self.codec.measure(&intermediate)
    }
}

impl<Codec, Decoded, Error> crate::FixedMeasurer for Convert<Codec, Decoded>
where
    Codec: crate::FixedMeasurer,
    Decoded: Clone,
    Codec::Decoded: TryFrom<Decoded, Error = Error>,
    Error: Into<crate::EncodeError>,
{
    fn fixed_measure(&self) -> usize {
        self.codec.fixed_measure()
    }
}
