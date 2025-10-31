use core::marker::PhantomData;

pub struct ConvertDecoded<Decoded, Encoder> {
    pub _marker: PhantomData<Decoded>,
    pub encoder: Encoder,
}

impl<Decoded, Encoder> Default for ConvertDecoded<Decoded, Encoder>
where
    Encoder: Default,
{
    fn default() -> Self {
        ConvertDecoded {
            _marker: PhantomData,
            encoder: Encoder::default(),
        }
    }
}

impl<Decoded, Encoder, Error> crate::Encoder for ConvertDecoded<Decoded, Encoder>
where
    Decoded: Clone,
    Encoder: crate::Encoder,
    Encoder::Decoded: TryFrom<Decoded, Error = Error>,
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
        self.encoder.encode(&intermediate, encoded, offset)
    }
}

impl<Decoded, Encoder, Error> crate::Measurer for ConvertDecoded<Decoded, Encoder>
where
    Decoded: Clone,
    Encoder: crate::Measurer,
    Encoder::Decoded: TryFrom<Decoded, Error = Error>,
    Error: Into<crate::EncodeError>,
{
    type Decoded = Decoded;

    fn measure(&self, decoded: &Self::Decoded) -> Result<usize, crate::EncodeError> {
        let intermediate = decoded.clone().try_into().map_err(Into::into)?;
        self.encoder.measure(&intermediate)
    }
}

impl<Decoded, Encoder, Error> crate::FixedMeasurer for ConvertDecoded<Decoded, Encoder>
where
    Decoded: Clone,
    Encoder: crate::FixedMeasurer,
    Encoder::Decoded: TryFrom<Decoded, Error = Error>,
    Error: Into<crate::EncodeError>,
{
    fn fixed_measure(&self) -> usize {
        self.encoder.fixed_measure()
    }
}

pub struct ConvertEncoded<Decoder, Encoded> {
    pub decoder: Decoder,
    pub _marker: PhantomData<Encoded>,
}

impl<Decoder, Encoded> Default for ConvertEncoded<Decoder, Encoded>
where
    Decoder: Default,
{
    fn default() -> Self {
        ConvertEncoded {
            decoder: Decoder::default(),
            _marker: PhantomData,
        }
    }
}

impl<Decoder, Encoded, Error> crate::Decoder for ConvertEncoded<Decoder, Encoded>
where
    Encoded: TryFrom<Decoder::Decoded, Error = Error>,
    Error: Into<crate::DecodeError>,
    Decoder: crate::Decoder,
{
    type Decoded = Encoded;
    fn decode(&self, encoded: &[u8], offset: &mut usize) -> Result<Self::Decoded, crate::DecodeError> {
        let intermediate = self.decoder.decode(encoded, offset)?;
        let decoded = intermediate.try_into().map_err(Into::into)?;
        Ok(decoded)
    }
}

impl<Decoder, Encoder> crate::Measurer for ConvertEncoded<Decoder, Encoder>
where
    Decoder: crate::Measurer,
    Encoder: crate::Measurer<Decoded = Decoder::Decoded>,
{
    type Decoded = Decoder::Decoded;

    fn measure(&self, decoded: &Self::Decoded) -> Result<usize, crate::EncodeError> {
        self.decoder.measure(decoded)
    }
}

impl<Decoder, Encoder> crate::FixedMeasurer for ConvertEncoded<Decoder, Encoder>
where
    Decoder: crate::FixedMeasurer,
    Encoder: crate::FixedMeasurer<Decoded = Decoder::Decoded>,
{
    fn fixed_measure(&self) -> usize {
        self.decoder.fixed_measure()
    }
}

pub struct Asymmetric<Decoder, Encoder> {
    pub decoder: Decoder,
    pub encoder: Encoder,
}

impl<Decoder, Encoder> Default for Asymmetric<Decoder, Encoder>
where
    Decoder: Default,
    Encoder: Default,
{
    fn default() -> Self {
        Asymmetric {
            decoder: Decoder::default(),
            encoder: Encoder::default(),
        }
    }
}

impl<Decoder, Encoder> crate::Decoder for Asymmetric<Decoder, Encoder>
where
    Decoder: crate::Decoder,
{
    type Decoded = Decoder::Decoded;
    fn decode(&self, encoded: &[u8], offset: &mut usize) -> Result<Self::Decoded, crate::DecodeError> {
        self.decoder.decode(encoded, offset)
    }
}

impl<Decoder, Encoder> crate::Encoder for Asymmetric<Decoder, Encoder>
where
    Encoder: crate::Encoder,
{
    type Decoded = Encoder::Decoded;
    fn encode(
        &self,
        decoded: &Self::Decoded,
        encoded: &mut [u8],
        offset: &mut usize,
    ) -> Result<(), crate::EncodeError> {
        self.encoder.encode(decoded, encoded, offset)
    }
}

impl<Decoder, Encoder> crate::Measurer for Asymmetric<Decoder, Encoder>
where
    Encoder: crate::Measurer,
{
    type Decoded = Encoder::Decoded;
    fn measure(&self, decoded: &Self::Decoded) -> Result<usize, crate::EncodeError> {
        self.encoder.measure(decoded)
    }
}

impl<Decoder, Encoder> crate::FixedMeasurer for Asymmetric<Decoder, Encoder>
where
    Encoder: crate::FixedMeasurer,
{
    fn fixed_measure(&self) -> usize {
        self.encoder.fixed_measure()
    }
}
