#[cfg(any(test, feature = "testing"))]
use proptest::{prelude::*, sample::SizeRange};
#[cfg(any(test, feature = "testing"))]
use proptest_derive::Arbitrary;

use super::Value;

pub type Seq = Vec<Value>;

#[cfg(any(test, feature = "testing"))]
pub(crate) fn arbitrary_seq() -> impl Strategy<Value = Seq> {
    arbitrary_seq_with(Value::arbitrary(), 0..10)
}

#[cfg(any(test, feature = "testing"))]
pub(crate) fn arbitrary_seq_with(
    element: impl Strategy<Value = Value>,
    size: impl Into<SizeRange>,
) -> impl Strategy<Value = Seq> {
    proptest::collection::vec(element, size.into()).prop_map(Seq::from_iter)
}

/// Represents a sequence of values.
#[cfg_attr(any(test, feature = "testing"), derive(Arbitrary))]
#[derive(Default, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct SeqValue(
    #[cfg_attr(any(test, feature = "testing"), proptest(strategy = "arbitrary_seq()"))] pub Seq,
);

impl SeqValue {
    pub fn as_slice(&self) -> &[Value] {
        &self.0
    }

    pub fn into_vec(self) -> Seq {
        self.0
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl From<Seq> for SeqValue {
    fn from(value: Seq) -> Self {
        Self(value)
    }
}

impl<'a> From<&'a SeqValue> for &'a [Value] {
    fn from(value: &'a SeqValue) -> Self {
        &value.0
    }
}

impl From<SeqValue> for Seq {
    fn from(value: SeqValue) -> Self {
        value.0
    }
}

impl std::fmt::Debug for SeqValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.0.iter()).finish()
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for SeqValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for SeqValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Self(Vec::deserialize(deserializer)?))
    }
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;
    use test_log::test;

    use crate::{
        config::EncoderConfig,
        decoder::Decoder,
        encoder::Encoder,
        io::{SliceReader, VecWriter},
        value::{NullValue, Value},
    };

    use super::*;

    #[test]
    fn debug() {
        assert_eq!(
            format!("{:?}", SeqValue::from(vec![Value::Null(NullValue)])),
            "[null]"
        );

        assert_eq!(
            format!("{:#?}", SeqValue::from(vec![Value::Null(NullValue)])),
            "[\n    Null(\n        null,\n    ),\n]"
        );
    }

    proptest! {
        #[test]
        fn encode_decode_roundtrip(value in SeqValue::arbitrary(), config in EncoderConfig::arbitrary()) {
            let mut encoded: Vec<u8> = Vec::new();
            let writer = VecWriter::new(&mut encoded);
            let mut encoder = Encoder::new_with_config(writer, config);
            encoder.encode_seq(&value.0).unwrap();

            // the encoded length of a seq depends on the items
            // contained within, so we're not checking it here.

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
