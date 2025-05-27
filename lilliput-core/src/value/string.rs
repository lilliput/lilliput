#[cfg(any(test, feature = "testing"))]
use proptest::prelude::*;
#[cfg(any(test, feature = "testing"))]
use proptest_derive::Arbitrary;

/// Represents a string.
#[cfg_attr(any(test, feature = "testing"), derive(Arbitrary))]
#[derive(Default, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct StringValue(pub String);

impl StringValue {
    /// Returns a reference to the internal string.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Returns the internal string, consuming `self`.
    pub fn into_string(self) -> String {
        self.0
    }

    /// Returns the length of the internal string.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns `true`, if the internal string is empty, otherwise `false`.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl From<String> for StringValue {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl<'a> From<&'a StringValue> for &'a str {
    fn from(value: &'a StringValue) -> Self {
        &value.0
    }
}

impl From<StringValue> for String {
    fn from(value: StringValue) -> Self {
        value.0
    }
}

impl std::fmt::Debug for StringValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            write!(f, "{:#?}", self.0)
        } else {
            write!(f, "{:?}", self.0)
        }
    }
}

impl std::fmt::Display for StringValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for StringValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for StringValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Self(String::deserialize(deserializer)?))
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
        value::Value,
    };

    use super::*;

    #[test]
    fn display() {
        assert_eq!(
            format!("{}", StringValue::from("lorem ipsum".to_owned())),
            "lorem ipsum"
        );
    }

    #[test]
    fn debug() {
        assert_eq!(
            format!("{:?}", StringValue::from("lorem ipsum".to_owned())),
            "\"lorem ipsum\""
        );

        assert_eq!(
            format!("{:#?}", StringValue::from("lorem ipsum".to_owned())),
            "\"lorem ipsum\""
        );
    }

    proptest! {
        #[test]
        fn encode_decode_roundtrip(value in StringValue::arbitrary(), config in EncoderConfig::arbitrary()) {
            let mut encoded: Vec<u8> = Vec::new();
            let writer = VecWriter::new(&mut encoded);
            let mut encoder = Encoder::new_with_config(writer, config);
            encoder.encode_str(value.as_str()).unwrap();

            prop_assert!(encoded.len() <= 1 + 8 + value.len());

            let reader = SliceReader::new(&encoded);
            let mut decoder = Decoder::new(reader);
            let decoded = decoder.decode_string().unwrap();
            prop_assert_eq!(&decoded, value.as_str());

            let reader = SliceReader::new(&encoded);
            let mut decoder = Decoder::new(reader);
            let decoded = decoder.decode_value().unwrap();
            let Value::String(decoded) = decoded else {
                panic!("expected string value");
            };
            prop_assert_eq!(&decoded, &value);
        }
    }
}
