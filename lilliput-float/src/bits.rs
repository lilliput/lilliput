use crate::floats::{F16, F24, F32, F40, F48, F56, F64, F8};

pub trait FpFromBits {
    type Bits;

    fn from_bits(bits: Self::Bits) -> Self;
}

macro_rules! impl_float_from_bits {
    ($t:ty => bytes: [u8; $bytes:expr], bits: $bits:ty) => {
        impl FpFromBits for $t {
            type Bits = $bits;

            fn from_bits(bits: Self::Bits) -> Self {
                const PADDED_BYTES: usize = (<$bits>::BITS / u8::BITS) as usize;
                const PADDING: usize = (PADDED_BYTES - $bytes) as usize;
                const MASK: $bits = (!0b0) >> PADDING;
                debug_assert_eq!(bits, bits & MASK);

                Self(bits & MASK)
            }
        }
    };
}

impl_float_from_bits!(F8 => bytes: [u8; 1], bits: u8);
impl_float_from_bits!(F16 => bytes: [u8; 2], bits: u16);
impl_float_from_bits!(F24 => bytes: [u8; 3], bits: u32);
impl_float_from_bits!(F32 => bytes: [u8; 4], bits: u32);
impl_float_from_bits!(F40 => bytes: [u8; 5], bits: u64);
impl_float_from_bits!(F48 => bytes: [u8; 6], bits: u64);
impl_float_from_bits!(F56 => bytes: [u8; 7], bits: u64);
impl_float_from_bits!(F64 => bytes: [u8; 8], bits: u64);

pub trait FpToBits {
    type Bits;

    fn to_bits(self) -> Self::Bits;
}

macro_rules! impl_float_to_bits {
    ($t:ty => bytes: [u8; $bytes:expr], bits: $bits:ty) => {
        impl FpToBits for $t {
            type Bits = $bits;

            fn to_bits(self) -> Self::Bits {
                self.0
            }
        }
    };
}

impl_float_to_bits!(F8 => bytes: [u8; 1], bits: u8);
impl_float_to_bits!(F16 => bytes: [u8; 2], bits: u16);
impl_float_to_bits!(F24 => bytes: [u8; 3], bits: u32);
impl_float_to_bits!(F32 => bytes: [u8; 4], bits: u32);
impl_float_to_bits!(F40 => bytes: [u8; 5], bits: u64);
impl_float_to_bits!(F48 => bytes: [u8; 6], bits: u64);
impl_float_to_bits!(F56 => bytes: [u8; 7], bits: u64);
impl_float_to_bits!(F64 => bytes: [u8; 8], bits: u64);

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use super::*;

    proptest! {
        #[test]
        fn f8_from_to_bits_roundtrip(bits_before in (0_u8..=!0b_0)) {
            let float = F8::from_bits(bits_before);
            let bits_after = float.to_bits();
            prop_assert_eq!(bits_before, bits_after);
        }

        #[test]
        fn f16_from_to_bits_roundtrip(bits_before in (0_u16..=!0b_0)) {
            let float = F16::from_bits(bits_before);
            let bits_after = float.to_bits();
            prop_assert_eq!(bits_before, bits_after);
        }

        #[test]
        fn f24_from_to_bits_roundtrip(bits_before in (0_u32..=(!0b_0 >> 8))) {
            let float = F24::from_bits(bits_before);
            let bits_after = float.to_bits();
            prop_assert_eq!(bits_before, bits_after);
        }

        #[test]
        fn f32_from_to_bits_roundtrip(bits_before in (0_u32..=!0b_0)) {
            let float = F32::from_bits(bits_before);
            let bits_after = float.to_bits();
            prop_assert_eq!(bits_before, bits_after);
        }

        #[test]
        fn f40_from_to_bits_roundtrip(bits_before in (0_u64..=(!0b_0 >> 24))) {
            let float = F40::from_bits(bits_before);
            let bits_after = float.to_bits();
            prop_assert_eq!(bits_before, bits_after);
        }

        #[test]
        fn f48_from_to_bits_roundtrip(bits_before in (0_u64..=(!0b_0 >> 16))) {
            let float = F48::from_bits(bits_before);
            let bits_after = float.to_bits();
            prop_assert_eq!(bits_before, bits_after);
        }

        #[test]
        fn f56_from_to_bits_roundtrip(bits_before in (0_u64..=(!0b_0 >> 8))) {
            let float = F56::from_bits(bits_before);
            let bits_after = float.to_bits();
            prop_assert_eq!(bits_before, bits_after);
        }

        #[test]
        fn f64_from_to_bits_roundtrip(bits_before in (0_u64..=!0b_0)) {
            let float = F64::from_bits(bits_before);
            let bits_after = float.to_bits();
            prop_assert_eq!(bits_before, bits_after);
        }
    }
}
