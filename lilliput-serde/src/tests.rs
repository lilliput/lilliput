use std::collections::BTreeMap;

use proptest::prelude::*;

use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::{from_slice, to_vec, value::*, Error, Value};

#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
#[derive(Default, Copy, Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
struct UnitStruct;

#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
#[derive(Default, Copy, Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
struct NewtypeStruct<T>(pub T);

#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
#[derive(Default, Copy, Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
struct TupleStruct<T>(pub T, pub T);

#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
#[derive(Default, Copy, Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
struct Struct<T> {
    pub a: T,
    pub b: T,
}

#[allow(clippy::enum_variant_names)]
#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
#[derive(Default, Copy, Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
enum Enum<T> {
    #[default]
    UnitVariant,
    NewtypeTupleVariant(T),
    TupleVariant(T, T),
    NewtypeStructVariant {
        a: T,
    },
    StructVariant {
        a: T,
        b: T,
    },
}

impl<T> Enum<T>
where
    T: Arbitrary + std::fmt::Debug,
{
    fn arbitrary_unit_variant() -> impl Strategy<Value = Self> {
        <()>::arbitrary().prop_map(|_| Self::UnitVariant)
    }

    fn arbitrary_newtype_variant() -> impl Strategy<Value = Self> {
        prop_oneof![
            T::arbitrary().prop_map(|a| Self::NewtypeTupleVariant(a)),
            T::arbitrary().prop_map(|a| Self::NewtypeStructVariant { a }),
        ]
    }

    fn arbitrary_tuple_variant() -> impl Strategy<Value = Self> {
        <(T, T)>::arbitrary().prop_map(|(a, b)| Self::TupleVariant(a, b))
    }

    fn arbitrary_struct_variant() -> impl Strategy<Value = Self> {
        <(T, T)>::arbitrary().prop_map(|(a, b)| Self::StructVariant { a, b })
    }
}

fn roundtrip<T>(value: &T) -> Result<T, Error>
where
    T: Serialize + DeserializeOwned,
{
    let encoded: Vec<u8> = to_vec(&value).unwrap();
    let decoded: T = from_slice(&encoded).unwrap();
    Ok(decoded)
}

mod value {
    use lilliput_core::value::UnitValue;

    use super::*;

    proptest! {
        #[test]
        fn int_roundtrip(value in IntValue::arbitrary()) {
            let decoded = roundtrip(&value)?;
            prop_assert_eq!(&decoded, &value);
        }

        #[test]
        fn string_roundtrip(value in StringValue::arbitrary()) {
            let decoded = roundtrip(&value)?;
            prop_assert_eq!(&decoded, &value);
        }

        #[test]
        fn seq_roundtrip(value in SeqValue::arbitrary()) {
            let decoded = roundtrip(&value)?;
            prop_assert_eq!(&decoded, &value);
        }

        #[test]
        fn map_roundtrip(value in MapValue::arbitrary()) {
            let decoded = roundtrip(&value)?;
            prop_assert_eq!(&decoded, &value);
        }

        #[test]
        fn float_roundtrip(value in FloatValue::arbitrary()) {
            let decoded = roundtrip(&value)?;
            prop_assert_eq!(&decoded, &value);
        }

        #[test]
        fn bytes_roundtrip(value in BytesValue::arbitrary()) {
            let decoded = roundtrip(&value)?;
            prop_assert_eq!(&decoded, &value);
        }

        #[test]
        fn bool_roundtrip(value in BoolValue::arbitrary()) {
            let decoded = roundtrip(&value)?;
            prop_assert_eq!(&decoded, &value);
        }

        #[test]
        fn unit_roundtrip(value in UnitValue::arbitrary()) {
            let decoded = roundtrip(&value)?;
            prop_assert_eq!(&decoded, &value);
        }

        #[test]
        fn null_roundtrip(value in NullValue::arbitrary()) {
            let decoded = roundtrip(&value)?;
            prop_assert_eq!(&decoded, &value);
        }

        #[test]
        fn any_roundtrip(value in Value::arbitrary()) {
            let encoded = to_vec(&value).unwrap();

            let decoded = roundtrip(&value)?;
            match (&decoded, &value) {
                (Value::Int(lhs), Value::Int(rhs)) => assert_eq!(lhs, rhs),
                (Value::String(lhs), Value::String(rhs)) => assert_eq!(lhs, rhs),
                (Value::Seq(lhs), Value::Seq(rhs)) => assert_eq!(lhs, rhs),
                (Value::Map(lhs), Value::Map(rhs)) => assert_eq!(lhs, rhs),
                (Value::Float(lhs), Value::Float(rhs)) => assert_eq!(lhs, rhs),
                (Value::Bytes(lhs), Value::Bytes(rhs)) => assert_eq!(lhs, rhs),
                (Value::Bool(lhs), Value::Bool(rhs)) => assert_eq!(lhs, rhs),
                (Value::Unit(lhs), Value::Unit(rhs)) => assert_eq!(lhs, rhs),
                (Value::Null(lhs), Value::Null(rhs)) => assert_eq!(lhs, rhs),
                (lhs, rhs) => panic!("{lhs:#?} != {rhs:#?} (encoded: {encoded:?}"),
            }
            assert_eq!(&decoded, &value);
            prop_assert_eq!(&decoded, &value);
        }
    }
}

mod bytes_repr {
    use super::*;

    #[test]
    fn seq() {
        #[derive(Eq, PartialEq, Debug, serde::Serialize, serde::Deserialize)]
        struct Subject {
            id: u32,
            name: String,
            data: Vec<u8>,
        }

        let value = Subject {
            id: 42,
            name: "Bob".to_owned(),
            data: vec![1, 2, 3, 4],
        };

        let encoded = to_vec(&value).unwrap();
        let decoded: Subject = from_slice(&encoded).unwrap();

        assert_eq!(decoded, value);
    }

    #[test]
    fn bytes() {
        #[derive(Eq, PartialEq, Debug, serde::Serialize, serde::Deserialize)]
        struct Subject {
            id: u32,
            name: String,
            #[serde(with = "serde_bytes")]
            data: Vec<u8>,
        }

        let value = Subject {
            id: 42,
            name: "Bob".to_owned(),
            data: vec![1, 2, 3, 4],
        };

        let encoded = to_vec(&value).unwrap();
        let decoded: Subject = from_slice(&encoded).unwrap();

        assert_eq!(decoded, value);
    }
}

mod zero_copy {
    use super::*;

    #[test]
    fn borrowed() {
        #[derive(Eq, PartialEq, Debug, serde::Serialize, serde::Deserialize)]
        struct Subject<'a> {
            id: u32,
            name: &'a str,
            #[serde(with = "serde_bytes")]
            data: &'a [u8],
        }

        let value = Subject {
            id: 42,
            name: "Bob",
            data: &[1, 2, 3, 4],
        };

        let encoded = to_vec(&value).unwrap();
        let decoded: Subject = from_slice(&encoded).unwrap();

        assert_eq!(decoded, value);
    }

    #[test]
    fn owned() {
        #[derive(Eq, PartialEq, Debug, serde::Serialize, serde::Deserialize)]
        struct Subject {
            id: u32,
            name: String,
            #[serde(with = "serde_bytes")]
            data: Vec<u8>,
        }

        let value = Subject {
            id: 42,
            name: "Bob".to_owned(),
            data: vec![1, 2, 3, 4],
        };

        let encoded = to_vec(&value).unwrap();
        let decoded: Subject = from_slice(&encoded).unwrap();

        assert_eq!(decoded, value);
    }
}

proptest! {
    #[test]
    fn i8_roundtrip(value in i8::arbitrary()) {
        let decoded = roundtrip(&value)?;
        prop_assert_eq!(&decoded, &value);
    }

    #[test]
    fn i16_roundtrip(value in i16::arbitrary()) {
        let decoded = roundtrip(&value)?;
        prop_assert_eq!(&decoded, &value);
    }

    #[test]
    fn i32_roundtrip(value in i32::arbitrary()) {
        let decoded = roundtrip(&value)?;
        prop_assert_eq!(&decoded, &value);
    }

    #[test]
    fn i64_roundtrip(value in i64::arbitrary()) {
        let decoded = roundtrip(&value)?;
        prop_assert_eq!(&decoded, &value);
    }

    #[test]
    fn u8_roundtrip(value in u8::arbitrary()) {
        let decoded = roundtrip(&value)?;
        prop_assert_eq!(&decoded, &value);
    }

    #[test]
    fn u16_roundtrip(value in u16::arbitrary()) {
        let decoded = roundtrip(&value)?;
        prop_assert_eq!(&decoded, &value);
    }

    #[test]
    fn u32_roundtrip(value in u32::arbitrary()) {
        let decoded = roundtrip(&value)?;
        prop_assert_eq!(&decoded, &value);
    }

    #[test]
    fn u64_roundtrip(value in u64::arbitrary()) {
        let decoded = roundtrip(&value)?;
        prop_assert_eq!(&decoded, &value);
    }

    #[test]
    fn f32_roundtrip(value in f32::arbitrary()) {
        let decoded = roundtrip(&value)?;
        prop_assert_eq!(&decoded, &value);
    }

    #[test]
    fn f64_roundtrip(value in f64::arbitrary()) {
        let decoded = roundtrip(&value)?;
        prop_assert_eq!(&decoded, &value);
    }

    #[test]
    fn char_roundtrip(value in char::arbitrary()) {
        let decoded = roundtrip(&value)?;
        prop_assert_eq!(&decoded, &value);
    }

    #[test]
    fn str_roundtrip(value in String::arbitrary()) {
        let decoded = roundtrip(&value)?;
        prop_assert_eq!(&decoded, &value);
    }

    #[test]
    fn bytes_roundtrip(value in Vec::<u8>::arbitrary()) {
        let decoded = roundtrip(&value)?;
        prop_assert_eq!(&decoded, &value);
    }

    #[test]
    fn none_roundtrip(_value in UnitStruct::arbitrary()) {
        let decoded = roundtrip(&Option::<UnitStruct>::None)?;
        prop_assert_eq!(&decoded, &None);
    }

    #[test]
    fn some_roundtrip(value in UnitStruct::arbitrary()) {
        let decoded = roundtrip(&Some(value))?;
        prop_assert_eq!(&decoded, &Some(value));
    }

    #[test]
    fn unit_roundtrip(_value in <()>::arbitrary()) {
        roundtrip(&())?;
        prop_assert_eq!(&(), &());
    }

    #[test]
    fn unit_struct_roundtrip(value in UnitStruct::arbitrary()) {
        let decoded = roundtrip(&value)?;
        prop_assert_eq!(&decoded, &value);
    }

    #[test]
    fn unit_variant_roundtrip(value in Enum::<bool>::arbitrary_unit_variant()) {
        let decoded = roundtrip(&value)?;
        prop_assert_eq!(&decoded, &value);
    }

    #[test]
    fn newtype_struct_roundtrip(value in NewtypeStruct::<bool>::arbitrary()) {
        let decoded = roundtrip(&value)?;
        prop_assert_eq!(&decoded, &value);
    }

    #[test]
    fn newtype_variant_roundtrip(value in Enum::<bool>::arbitrary_newtype_variant()) {
        let decoded = roundtrip(&value)?;
        prop_assert_eq!(&decoded, &value);
    }

    #[test]
    fn seq_roundtrip(value in Vec::<bool>::arbitrary()) {
        let decoded = roundtrip(&value)?;
        prop_assert_eq!(&decoded, &value);
    }

    #[test]
    fn tuple_roundtrip(value in (u8::arbitrary(), i8::arbitrary())) {
        let decoded = roundtrip(&value)?;
        prop_assert_eq!(&decoded, &value);
    }

    #[test]
    fn tuple_struct_roundtrip(value in (u8::arbitrary(), i8::arbitrary())) {
        let decoded = roundtrip(&value)?;
        prop_assert_eq!(&decoded, &value);
    }

    #[test]
    fn tuple_variant_roundtrip(value in Enum::<bool>::arbitrary_tuple_variant()) {
        let decoded = roundtrip(&value)?;
        prop_assert_eq!(&decoded, &value);
    }

    #[test]
    fn map_roundtrip(value in BTreeMap::<bool, bool>::arbitrary()) {
        let decoded = roundtrip(&value)?;
        prop_assert_eq!(&decoded, &value);
    }

    #[test]
    fn struct_roundtrip(value in Struct::<bool>::arbitrary()) {
        let decoded = roundtrip(&value)?;
        prop_assert_eq!(&decoded, &value);
    }

    #[test]
    fn struct_variant_roundtrip(value in Enum::<bool>::arbitrary_struct_variant()) {
        let decoded = roundtrip(&value)?;
        prop_assert_eq!(&decoded, &value);
    }
}
