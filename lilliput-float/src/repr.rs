use crate::floats::{F16, F24, F32, F40, F48, F56, F64, F8};

pub trait FpRepr: Sized + Copy + PartialEq + PartialOrd {
    type Bits;

    const ZERO: Self;
    const ONE: Self;

    const MIN: Self;
    const MAX: Self;
    const MIN_POSITIVE: Self;

    const INFINITY: Self;
    const NEG_INFINITY: Self;

    const BITS: u32;
    const SIGN_BITS: u32;
    const EXPONENT_BITS: u32;
    const SIGNIFICAND_BITS: u32;

    const EXPONENT_MAX: Self::Bits;
    const EXPONENT_BIAS: Self::Bits;

    const SIGN_MASK: Self::Bits;
    const EXPONENT_MASK: Self::Bits;
    const SIGNIFICAND_MASK: Self::Bits;

    const IMPLICIT_BIT: Self::Bits;
}

macro_rules! impl_float_repr {
    ($t:ty, bytes: [u8; $bytes:expr], bits: $bits:ty, sign: 1, exponent: $exponent:expr, significand: $significand:expr) => {
        impl FpRepr for $t {
            type Bits = $bits;

            const ZERO: Self = Self(0);
            const ONE: Self = Self((Self::EXPONENT_MASK >> 1) & Self::EXPONENT_MASK);

            const MIN: Self = Self(
                Self::SIGN_MASK
                    | ((Self::EXPONENT_MASK << 1) & Self::EXPONENT_MASK)
                    | Self::SIGNIFICAND_MASK,
            );
            const MAX: Self =
                Self(((Self::EXPONENT_MASK << 1) & Self::EXPONENT_MASK) | Self::SIGNIFICAND_MASK);
            const MIN_POSITIVE: Self = Self(1 << Self::SIGNIFICAND_BITS);

            const INFINITY: Self = Self(Self::EXPONENT_MASK);
            const NEG_INFINITY: Self = Self(Self::SIGN_MASK | Self::EXPONENT_MASK);

            const BITS: u32 = Self::SIGN_BITS + Self::EXPONENT_BITS + Self::SIGNIFICAND_BITS;
            const SIGN_BITS: u32 = 1;
            const EXPONENT_BITS: u32 = $exponent;
            const SIGNIFICAND_BITS: u32 = $significand;

            const EXPONENT_MAX: Self::Bits = (1 << Self::EXPONENT_BITS) - 1;
            const EXPONENT_BIAS: Self::Bits = (Self::EXPONENT_MAX >> 1) as Self::Bits;

            const SIGN_MASK: Self::Bits = 1 << (Self::BITS - 1);
            const EXPONENT_MASK: Self::Bits = (Self::SIGN_MASK - 1) & !Self::SIGNIFICAND_MASK;
            const SIGNIFICAND_MASK: Self::Bits = (1 << Self::SIGNIFICAND_BITS) - 1;
            const IMPLICIT_BIT: Self::Bits = 1 << Self::SIGNIFICAND_BITS;
        }
    };
}

impl_float_repr!(F8, bytes: [u8; 1], bits: u8, sign: 1, exponent: 4, significand: 3);
impl_float_repr!(F16, bytes: [u8; 2], bits: u16, sign: 1, exponent: 5, significand: 10);
impl_float_repr!(F24, bytes: [u8; 3], bits: u32, sign: 1, exponent: 7, significand: 16);
impl_float_repr!(F32, bytes: [u8; 4], bits: u32, sign: 1, exponent: 8, significand: 23);
impl_float_repr!(F40, bytes: [u8; 5], bits: u64, sign: 1, exponent: 8, significand: 31);
impl_float_repr!(F48, bytes: [u8; 6], bits: u64, sign: 1, exponent: 9, significand: 38);
impl_float_repr!(F56, bytes: [u8; 7], bits: u64, sign: 1, exponent: 10, significand: 45);
impl_float_repr!(F64, bytes: [u8; 8], bits: u64, sign: 1, exponent: 11, significand: 52);

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "native-f16")]
    #[test]
    fn f16_matches_native_behavior() {
        assert_eq!(F16::ZERO, F16::from(0.0_f16));
        assert_eq!(F16::ONE, F16::from(1.0_f16));
        assert_eq!(F16::MIN, F16::from(f16::MIN));
        assert_eq!(F16::MAX, F16::from(f16::MAX));
        assert_eq!(F16::MIN_POSITIVE, F16::from(f16::MIN_POSITIVE));
        assert_eq!(F16::INFINITY, F16::from(f16::INFINITY));
        assert_eq!(F16::NEG_INFINITY, F16::from(f16::NEG_INFINITY));
    }

    #[test]
    fn f32_matches_native_behavior() {
        assert_eq!(F32::ZERO, F32::from(0.0_f32));
        assert_eq!(F32::ONE, F32::from(1.0_f32));
        assert_eq!(F32::MIN, F32::from(f32::MIN));
        assert_eq!(F32::MAX, F32::from(f32::MAX));
        assert_eq!(F32::MIN_POSITIVE, F32::from(f32::MIN_POSITIVE));
        assert_eq!(F32::INFINITY, F32::from(f32::INFINITY));
        assert_eq!(F32::NEG_INFINITY, F32::from(f32::NEG_INFINITY));
    }

    #[test]
    fn f64_matches_native_behavior() {
        assert_eq!(F64::ZERO, F64::from(0.0_f64));
        assert_eq!(F64::ONE, F64::from(1.0_f64));
        assert_eq!(F64::MIN, F64::from(f64::MIN));
        assert_eq!(F64::MAX, F64::from(f64::MAX));
        assert_eq!(F64::MIN_POSITIVE, F64::from(f64::MIN_POSITIVE));
        assert_eq!(F64::INFINITY, F64::from(f64::INFINITY));
        assert_eq!(F64::NEG_INFINITY, F64::from(f64::NEG_INFINITY));
    }
}
