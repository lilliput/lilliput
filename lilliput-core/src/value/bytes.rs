use crate::binary::BytesSlice;

/// Represents a byte sequence.
#[derive(Default, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct BytesValue(pub Vec<u8>);

impl BytesValue {
    pub fn as_slice(&self) -> &[u8] {
        &self.0
    }

    pub fn into_vec(self) -> Vec<u8> {
        self.0
    }
}

impl From<Vec<u8>> for BytesValue {
    fn from(value: Vec<u8>) -> Self {
        Self(value)
    }
}

impl<'a> From<&'a BytesValue> for &'a [u8] {
    fn from(value: &'a BytesValue) -> Self {
        &value.0
    }
}

impl From<BytesValue> for Vec<u8> {
    fn from(value: BytesValue) -> Self {
        value.0
    }
}

impl std::fmt::Debug for BytesValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&BytesSlice(&self.0), f)
    }
}

impl std::fmt::Display for BytesValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&BytesSlice(&self.0), f)
    }
}

#[cfg(any(test, feature = "testing"))]
impl proptest::prelude::Arbitrary for BytesValue {
    type Parameters = ();
    type Strategy = proptest::prelude::BoxedStrategy<Self>;

    fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
        use proptest::prelude::Strategy as _;
        proptest::collection::vec(proptest::num::u8::ANY, 0..=10)
            .prop_map(Self)
            .boxed()
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
        assert_eq!(
            format!("{}", BytesValue::from(vec![1, 2, 3])),
            "[01, 02, 03]"
        );
    }

    #[test]
    fn debug() {
        assert_eq!(
            format!("{:?}", BytesValue::from(vec![1, 2, 3])),
            "[00000001, 00000010, 00000011]"
        );

        assert_eq!(
            format!("{:#?}", BytesValue::from(vec![1, 2, 3])),
            "[0b00000001, 0b00000010, 0b00000011]"
        );
    }

    proptest! {
        #[test]
        fn encode_decode_roundtrip(value in BytesValue::arbitrary(), config in EncodingConfig::arbitrary()) {
            let mut encoded: Vec<u8> = Vec::new();
            let writer = VecWriter::new(&mut encoded);
            let mut encoder = Encoder::new(writer, config);
            encoder.encode_bytes(value.as_slice()).unwrap();

            let reader = SliceReader::new(&encoded);
            let mut decoder = Decoder::new(reader);
            let decoded = decoder.decode_bytes_buf().unwrap();
            prop_assert_eq!(&decoded, value.as_slice());

            let reader = SliceReader::new(&encoded);
            let mut decoder = Decoder::new(reader);
            let decoded = decoder.decode_value().unwrap();
            let Value::Bytes(decoded) = decoded else {
                panic!("expected bytes value");
            };
            prop_assert_eq!(&decoded, &value);
        }
    }
}
