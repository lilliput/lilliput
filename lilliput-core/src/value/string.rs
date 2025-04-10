#[cfg(any(test, feature = "testing"))]
use proptest::prelude::*;
#[cfg(any(test, feature = "testing"))]
use proptest_derive::Arbitrary;

/// Represents a string.
#[cfg_attr(any(test, feature = "testing"), derive(Arbitrary))]
#[derive(Default, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct StringValue(pub String);

impl StringValue {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_string(self) -> String {
        self.0
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

#[cfg(test)]
mod tests {
    use proptest::prelude::*;
    use test_log::test;

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
        fn encode_decode_roundtrip(value in StringValue::arbitrary(), config in EncodingConfig::arbitrary()) {
            let mut encoded: Vec<u8> = Vec::new();
            let writer = VecWriter::new(&mut encoded);
            let mut encoder = Encoder::new(writer, config);
            encoder.encode_str(value.as_str()).unwrap();

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
