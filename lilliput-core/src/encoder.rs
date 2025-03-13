use crate::{
    Profile,
    value::{NullValue, Value, ValueType},
};

#[derive(Eq, PartialEq, Debug, thiserror::Error)]
pub enum Error {
    #[error("invalid seq")]
    Seq,
    #[error("invalid map")]
    Map,
}

enum EncoderState {
    Seq { pos: usize, len: usize },
    Map { pos: usize, len: usize },
}

impl EncoderState {
    fn seq(len: usize) -> Self {
        Self::Seq { pos: 0, len }
    }

    fn map(len: usize) -> Self {
        Self::Map {
            pos: 0,
            len: 2 * len,
        }
    }

    fn on_encode_value(&mut self) -> Result<(), Error> {
        match self {
            EncoderState::Seq { pos, len } => {
                if pos < len {
                    *pos += 1;
                    Ok(())
                } else {
                    Err(Error::Seq)
                }
            }
            EncoderState::Map { pos, len } => {
                if pos < len {
                    *pos += 1;
                    Ok(())
                } else {
                    Err(Error::Map)
                }
            }
        }
    }
}

pub struct Encoder {
    buf: Vec<u8>,
    pos: usize,
    profile: Profile,
    state: Vec<EncoderState>,
}

impl Encoder {
    pub fn new(profile: Profile) -> Self {
        Encoder {
            buf: vec![],
            pos: 0,
            profile,
            state: vec![],
        }
    }
}

impl Encoder {
    pub fn into_vec(self) -> Result<Vec<u8>, Error> {
        if let Some(state) = self.state.last() {
            match state {
                EncoderState::Seq { .. } => Err(Error::Seq),
                EncoderState::Map { .. } => Err(Error::Map),
            }
        } else {
            Ok(self.buf)
        }
    }

    pub fn encode_any(&mut self, value: &Value) -> Result<(), Error> {
        match value {
            Value::Null(value) => self.encode_null_value(value),
        }
    }

    pub fn encode_null(&mut self) -> Result<(), Error> {
        let head_byte = NullValue::BIT_REPR;

        self.push_byte(head_byte)?;

        self.on_encode_value()
    }

    fn encode_null_value(&mut self, value: &NullValue) -> Result<(), Error> {
        let NullValue = value;
        self.encode_null()
    }
}

impl Encoder {
    fn push_byte(&mut self, byte: u8) -> Result<(), Error> {
        self.buf.push(byte);
        self.pos += 1;

        Ok(())
    }

    fn push_bytes(&mut self, bytes: &[u8]) -> Result<(), Error> {
        self.pos += bytes.len();

        self.buf.extend_from_slice(bytes);

        Ok(())
    }

    fn existing(&self) -> usize {
        self.pos
    }

    fn on_encode_value(&mut self) -> Result<(), Error> {
        if let Some(state) = self.state.last_mut() {
            state.on_encode_value()
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn push_byte() {
        let mut encoder = Encoder::new(Profile::None);

        encoder.push_byte(1).unwrap();
        assert_eq!(encoder.buf, vec![1]);

        encoder.push_byte(2).unwrap();
        assert_eq!(encoder.buf, vec![1, 2]);

        encoder.push_byte(3).unwrap();
        assert_eq!(encoder.buf, vec![1, 2, 3]);
    }

    #[test]
    fn push_bytes() {
        let mut encoder = Encoder::new(Profile::None);

        encoder.push_bytes(&[1]).unwrap();
        assert_eq!(encoder.buf, vec![1]);

        encoder.push_bytes(&[2, 3]).unwrap();
        assert_eq!(encoder.buf, vec![1, 2, 3]);
    }

    #[test]
    fn existing() {
        let mut encoder = Encoder::new(Profile::None);
        assert_eq!(encoder.existing(), 0);

        encoder.push_byte(42).unwrap();
        assert_eq!(encoder.existing(), 1);
    }

    #[test]
    fn into_vec() {
        let mut encoder = Encoder::new(Profile::None);
        encoder.push_bytes(&[1, 2, 3]).unwrap();

        assert_eq!(encoder.into_vec().unwrap(), vec![1, 2, 3]);
    }
}
