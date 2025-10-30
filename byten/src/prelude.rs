use crate::{Encode, EncodeError, Encoder, Measure, Measurer};

pub trait EncodeToVec {
  fn encode_to_vec(&self) -> Result<Vec<u8>, EncodeError>;
}

impl<T: Encode + Measure> EncodeToVec for T {
    fn encode_to_vec(&self) -> Result<Vec<u8>, EncodeError> {
        let size = self.measure();
        let mut vec = vec![0u8; size];
        let mut offset = 0;
        self.encode(&mut vec, &mut offset)?;
        Ok(vec)
    }
}

pub trait EncoderToVec {
    type Decoded;
    fn encode_to_vec(&self, decoded: &Self::Decoded) -> Result<Vec<u8>, EncodeError>;
}

impl<Decoded, C: Encoder<Decoded=Decoded> + Measurer<Decoded=Decoded>> EncoderToVec for C {
    type Decoded = Decoded;
    fn encode_to_vec(&self, decoded: &Self::Decoded) -> Result<Vec<u8>, EncodeError> {
        let size = self.measure(decoded);
        let mut vec = vec![0u8; size];
        let mut offset = 0;
        self.encode(decoded, &mut vec, &mut offset)?;
        Ok(vec)
    }
}
