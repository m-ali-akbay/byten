use crate::{Encode, EncodeError, Measure};

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
