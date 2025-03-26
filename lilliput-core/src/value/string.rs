/// Represents a string.
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

#[cfg(any(test, feature = "testing"))]
impl proptest::arbitrary::Arbitrary for StringValue {
    type Parameters = ();
    type Strategy = proptest::strategy::BoxedStrategy<Self>;

    fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
        use proptest::prelude::*;

        proptest::string::string_regex("[a-zA-Z]+")
            .unwrap()
            .prop_map(StringValue::from)
            .boxed()
    }
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use crate::{
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
        fn encode_decode_roundtrip(value in StringValue::arbitrary()) {
            let mut encoded: Vec<u8> = Vec::new();
            let writer = VecWriter::new(&mut encoded);
            let mut encoder = Encoder::new(writer);
            encoder.encode_str(value.as_str()).unwrap();

            let reader = SliceReader::new(&encoded);
            let mut decoder = Decoder::new(reader);
            let decoded = decoder.decode_string().unwrap();
            prop_assert_eq!(&decoded, value.as_str());

            let reader = SliceReader::new(&encoded);
            let mut decoder = Decoder::new(reader);
            let decoded = decoder.decode_any().unwrap();
            let Value::String(decoded) = decoded else {
                panic!("expected string value");
            };
            prop_assert_eq!(&decoded, &value);
        }
    }
}
