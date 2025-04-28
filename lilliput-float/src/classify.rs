use std::num::FpCategory;

use crate::bits::FpToBits;
use crate::floats::{F16, F24, F32, F40, F48, F56, F64, F8};
use crate::repr::FpRepr;
use crate::PackedFloat;

pub trait FpClassify: Sized {
    fn classify(&self) -> FpCategory;

    fn is_zero(&self) -> bool {
        matches!(self.classify(), FpCategory::Zero)
    }

    fn is_nan(&self) -> bool {
        matches!(self.classify(), FpCategory::Nan)
    }

    fn is_infinite(&self) -> bool {
        matches!(self.classify(), FpCategory::Infinite)
    }

    fn is_subnormal(&self) -> bool {
        matches!(self.classify(), FpCategory::Subnormal)
    }

    fn is_normal(&self) -> bool {
        matches!(self.classify(), FpCategory::Normal)
    }
}

macro_rules! impl_float_classify {
    ($t:ty) => {
        impl FpClassify for $t {
            fn classify(&self) -> FpCategory {
                let bits = self.to_bits();
                let exponent_bits = bits & Self::EXPONENT_MASK;
                let significand_bits = bits & Self::SIGNIFICAND_MASK;

                match (exponent_bits, significand_bits) {
                    (Self::EXPONENT_MASK, 0) => FpCategory::Infinite,
                    (Self::EXPONENT_MASK, _) => FpCategory::Nan,
                    (0, 0) => FpCategory::Zero,
                    (0, _) => FpCategory::Subnormal,
                    _ => FpCategory::Normal,
                }
            }
        }
    };
}

impl_float_classify!(F8);
impl_float_classify!(F16);
impl_float_classify!(F24);
impl_float_classify!(F32);
impl_float_classify!(F40);
impl_float_classify!(F48);
impl_float_classify!(F56);
impl_float_classify!(F64);

impl FpClassify for PackedFloat {
    fn classify(&self) -> FpCategory {
        match self {
            Self::F8(value) => value.classify(),
            Self::F16(value) => value.classify(),
            Self::F24(value) => value.classify(),
            Self::F32(value) => value.classify(),
            Self::F40(value) => value.classify(),
            Self::F48(value) => value.classify(),
            Self::F56(value) => value.classify(),
            Self::F64(value) => value.classify(),
        }
    }
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use super::*;

    proptest! {
        #[test]
        fn f32_matches_native_behavior(native in f32::arbitrary()) {
            let subject = F32::from(native);
            let actual = subject.classify();
            let expected = native.classify();
            prop_assert_eq!(actual, expected);
        }

        #[test]
        fn f64_matches_native_behavior(native in f64::arbitrary()) {
            let subject = F64::from(native);
            let actual = subject.classify();
            let expected = native.classify();
            prop_assert_eq!(actual, expected);
        }
    }
}
