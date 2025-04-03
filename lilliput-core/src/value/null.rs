/// Represents a null value.
#[derive(Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct NullValue(pub ());

impl From<()> for NullValue {
    fn from(value: ()) -> Self {
        Self(value)
    }
}

impl std::fmt::Debug for NullValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "null")
    }
}

impl std::fmt::Display for NullValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "null")
    }
}

#[cfg(any(test, feature = "testing"))]
impl proptest::prelude::Arbitrary for NullValue {
    type Parameters = ();
    type Strategy = proptest::prelude::BoxedStrategy<Self>;

    fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
        use proptest::prelude::*;
        Just(NullValue::default()).boxed()
    }
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use crate::{
        config::EncodingConfig,
        decoder::Decoder,
        encoder::Encoder,
        io::{SliceReader, VecWriter},
        value::Value,
    };

    use super::*;

    #[test]
    fn display() {
        assert_eq!(format!("{}", NullValue::default()), "null");
    }

    #[test]
    fn debug() {
        assert_eq!(format!("{:?}", NullValue::default()), "null");
        assert_eq!(format!("{:#?}", NullValue::default()), "null");
    }

    proptest! {
        #[test]
        fn encode_decode_roundtrip(value in NullValue::arbitrary(), config in EncodingConfig::arbitrary()) {
            let mut encoded: Vec<u8> = Vec::new();
            let writer = VecWriter::new(&mut encoded);
            let mut encoder = Encoder::new(writer, config);
            encoder.encode_null().unwrap();

            let reader = SliceReader::new(&encoded);
            let mut decoder = Decoder::new(reader);
            decoder.decode_null().unwrap();

            let reader = SliceReader::new(&encoded);
            let mut decoder = Decoder::new(reader);
            let decoded = decoder.decode_any().unwrap();
            let Value::Null(decoded) = decoded else {
                panic!("expected null value");
            };
            prop_assert_eq!(&decoded, &value);
        }
    }
}
