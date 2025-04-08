/// Represents a unit value.
#[derive(Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct UnitValue(pub ());

impl From<()> for UnitValue {
    fn from(value: ()) -> Self {
        Self(value)
    }
}

impl std::fmt::Debug for UnitValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "unit")
    }
}

impl std::fmt::Display for UnitValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "unit")
    }
}

#[cfg(any(test, feature = "testing"))]
impl proptest::prelude::Arbitrary for UnitValue {
    type Parameters = ();
    type Strategy = proptest::prelude::BoxedStrategy<Self>;

    fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
        use proptest::prelude::*;
        Just(UnitValue::default()).boxed()
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
        assert_eq!(format!("{}", UnitValue::default()), "unit");
    }

    #[test]
    fn debug() {
        assert_eq!(format!("{:?}", UnitValue::default()), "unit");
        assert_eq!(format!("{:#?}", UnitValue::default()), "unit");
    }

    proptest! {
        #[test]
        fn encode_decode_roundtrip(value in UnitValue::arbitrary(), config in EncodingConfig::arbitrary()) {
            let mut encoded: Vec<u8> = Vec::new();
            let writer = VecWriter::new(&mut encoded);
            let mut encoder = Encoder::new(writer, config);
            encoder.encode_unit().unwrap();

            let reader = SliceReader::new(&encoded);
            let mut decoder = Decoder::new(reader);
            decoder.decode_unit().unwrap();

            let reader = SliceReader::new(&encoded);
            let mut decoder = Decoder::new(reader);
            let decoded = decoder.decode_value().unwrap();
            let Value::Unit(decoded) = decoded else {
                panic!("expected unit value");
            };
            prop_assert_eq!(&decoded, &value);
        }
    }
}
