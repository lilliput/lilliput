#[cfg(any(test, feature = "testing"))]
use proptest::{prelude::*, sample::SizeRange};

use super::Value;

/// Represents a sequence of values.
#[derive(Default, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct SeqValue(pub Vec<Value>);

impl SeqValue {
    pub fn as_slice(&self) -> &[Value] {
        &self.0
    }

    pub fn into_vec(self) -> Vec<Value> {
        self.0
    }
}

impl From<Vec<Value>> for SeqValue {
    fn from(value: Vec<Value>) -> Self {
        Self(value)
    }
}

impl<'a> From<&'a SeqValue> for &'a [Value] {
    fn from(value: &'a SeqValue) -> Self {
        &value.0
    }
}

impl From<SeqValue> for Vec<Value> {
    fn from(value: SeqValue) -> Self {
        value.0
    }
}

impl std::fmt::Debug for SeqValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.0.iter()).finish()
    }
}

#[doc(hidden)]
#[cfg(any(test, feature = "testing"))]
pub struct SeqValueArbitraryParameters {
    pub items: BoxedStrategy<Value>,
    pub size: SizeRange,
}

#[cfg(any(test, feature = "testing"))]
impl Default for SeqValueArbitraryParameters {
    fn default() -> Self {
        Self {
            items: Value::arbitrary(),
            size: (0..10).into(),
        }
    }
}

#[cfg(any(test, feature = "testing"))]
impl proptest::arbitrary::Arbitrary for SeqValue {
    type Parameters = SeqValueArbitraryParameters;
    type Strategy = proptest::strategy::BoxedStrategy<Self>;

    fn arbitrary_with(args: Self::Parameters) -> Self::Strategy {
        proptest::collection::vec(args.items, args.size)
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
        value::{NullValue, Value},
    };

    use super::*;

    #[test]
    fn debug() {
        assert_eq!(
            format!(
                "{:?}",
                SeqValue::from(vec![Value::Null(NullValue::default())])
            ),
            "[null]"
        );

        assert_eq!(
            format!(
                "{:#?}",
                SeqValue::from(vec![Value::Null(NullValue::default())])
            ),
            "[\n    Null(\n        null,\n    ),\n]"
        );
    }

    proptest! {
        #[test]
        fn encode_decode_roundtrip(value in SeqValue::arbitrary(), config in EncodingConfig::arbitrary()) {
            let mut encoded: Vec<u8> = Vec::new();
            let writer = VecWriter::new(&mut encoded);
            let mut encoder = Encoder::new(writer, config);
            encoder.encode_seq(&value.0).unwrap();

            let reader = SliceReader::new(&encoded);
            let mut decoder = Decoder::new(reader);
            let decoded = decoder.decode_seq().unwrap();
            prop_assert_eq!(&decoded, &value.0);

            let reader = SliceReader::new(&encoded);
            let mut decoder = Decoder::new(reader);
            let decoded = decoder.decode_value().unwrap();
            let Value::Seq(decoded) = decoded else {
                panic!("expected seq value");
            };
            prop_assert_eq!(&decoded, &value);
        }
    }
}
