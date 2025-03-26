use crate::{
    sealed::Sealed,
    value::{SignedIntValue, UnsignedIntValue},
};

pub trait ToZigZag: Sized + Sealed {
    type ZigZag: Sized;

    fn to_zig_zag(self) -> Self::ZigZag;
}

pub trait FromZigZag: Sized + Sealed {
    type ZigZag: Sized;

    fn from_zig_zag(zig_zag: Self::ZigZag) -> Self;
}

macro_rules! impl_zig_zag {
    (signed: $s:ty, unsigned: $u:ty) => {
        impl ToZigZag for $s {
            type ZigZag = $u;

            fn to_zig_zag(self) -> Self::ZigZag {
                (self >> (Self::BITS as usize) - 1) as Self::ZigZag ^ (self << 1) as Self::ZigZag
            }
        }

        impl FromZigZag for $s {
            type ZigZag = $u;

            fn from_zig_zag(zig_zag: Self::ZigZag) -> Self {
                (zig_zag >> 1) as Self ^ -((zig_zag & 1) as Self)
            }
        }
    };
}

impl_zig_zag!(signed: i8, unsigned: u8);
impl_zig_zag!(signed: i16, unsigned: u16);
impl_zig_zag!(signed: i32, unsigned: u32);
impl_zig_zag!(signed: i64, unsigned: u64);

impl ToZigZag for SignedIntValue {
    type ZigZag = UnsignedIntValue;

    fn to_zig_zag(self) -> Self::ZigZag {
        match self {
            Self::I8(signed) => UnsignedIntValue::U8(signed.to_zig_zag()),
            Self::I16(signed) => UnsignedIntValue::U16(signed.to_zig_zag()),
            Self::I32(signed) => UnsignedIntValue::U32(signed.to_zig_zag()),
            Self::I64(signed) => UnsignedIntValue::U64(signed.to_zig_zag()),
        }
    }
}

impl FromZigZag for SignedIntValue {
    type ZigZag = UnsignedIntValue;

    fn from_zig_zag(zig_zag: Self::ZigZag) -> Self {
        match zig_zag {
            UnsignedIntValue::U8(unsigned) => Self::I8(i8::from_zig_zag(unsigned)),
            UnsignedIntValue::U16(unsigned) => Self::I16(i16::from_zig_zag(unsigned)),
            UnsignedIntValue::U32(unsigned) => Self::I32(i32::from_zig_zag(unsigned)),
            UnsignedIntValue::U64(unsigned) => Self::I64(i64::from_zig_zag(unsigned)),
        }
    }
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use super::*;

    proptest! {
        #[test]
        fn zig_zag_roundtrip(before in i8::MIN..=i8::MAX) {
            let zig_zag = before.to_zig_zag();
            let after = i8::from_zig_zag(zig_zag);

            prop_assert_eq!(&before, &after);
        }
    }
}
