use std::cmp::Ordering;

use crate::{
    bits::FpToBits,
    floats::{F16, F24, F32, F40, F48, F56, F64, F8},
    repr::FpRepr,
};

// Adapted from rustc's compiler-builtins:
// https://github.com/rust-lang/compiler-builtins/blob/3dea633a80d32da75e923a940d16ce98cce74822/src/float/cmp.rs

macro_rules! impl_float_partial_eq_and_ord {
    ($t:ty => unsigned: $unsigned:ty, signed: $signed:ty) => {
        impl PartialEq for $t {
            fn eq(&self, other: &Self) -> bool {
                self.partial_cmp(other) == Some(Ordering::Equal)
            }
        }

        impl PartialOrd for $t {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                let sign_bit = <$t>::SIGN_MASK;
                let abs_mask = sign_bit - 1;
                let exponent_mask = <$t>::EXPONENT_MASK;
                let inf_rep = exponent_mask;

                let lhs = *self;
                let rhs = *other;

                let lhs_bits = lhs.to_bits();
                let rhs_bits = rhs.to_bits();
                let lhs_abs = lhs_bits & abs_mask;
                let rhs_abs = rhs_bits & abs_mask;

                // If either a or b is NaN, they are unordered.
                if lhs_abs > inf_rep || rhs_abs > inf_rep {
                    return None;
                }

                // If a and b are both zeros, they are equal.
                if (lhs_abs | rhs_abs) == 0 {
                    return Some(Ordering::Equal);
                }

                let lhs_srep: $signed = lhs_bits as $signed;
                let rhs_srep: $signed = rhs_bits as $signed;

                // If at least one of a and b is positive, we get the same result comparing
                // a and b as signed integers as we would with a fp_ting-point compare.
                if (lhs_srep & rhs_srep) >= 0 {
                    if lhs_srep < rhs_srep {
                        Some(Ordering::Less)
                    } else if lhs_srep == rhs_srep {
                        Some(Ordering::Equal)
                    } else {
                        Some(Ordering::Greater)
                    }
                    // Otherwise, both are negative, so we need to flip the sense of the
                    // comparison to get the correct result.  (This assumes a twos- or ones-
                    // complement integer representation; if integers are represented in a
                    // sign-magnitude representation, then this flip is incorrect).
                } else if lhs_srep > rhs_srep {
                    Some(Ordering::Less)
                } else if lhs_srep == rhs_srep {
                    Some(Ordering::Equal)
                } else {
                    Some(Ordering::Greater)
                }
            }
        }
    };
}

impl_float_partial_eq_and_ord!(F8 => unsigned: u8, signed: i8);
impl_float_partial_eq_and_ord!(F16 => unsigned: u16, signed: i16);
impl_float_partial_eq_and_ord!(F24 => unsigned: u32, signed: i32);
impl_float_partial_eq_and_ord!(F32 => unsigned: u32, signed: i32);
impl_float_partial_eq_and_ord!(F40 => unsigned: u64, signed: i64);
impl_float_partial_eq_and_ord!(F48 => unsigned: u64, signed: i64);
impl_float_partial_eq_and_ord!(F56 => unsigned: u64, signed: i64);
impl_float_partial_eq_and_ord!(F64 => unsigned: u64, signed: i64);

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use super::*;

    proptest! {
        #[test]
        fn f32_matches_native_behavior(native_lhs in f32::arbitrary(), native_rhs in f32::arbitrary()) {
            let (lhs, rhs) = (F32::from(native_lhs), F32::from(native_rhs));
            let actual = lhs.partial_cmp(&rhs);
            let expected = native_lhs.partial_cmp(&native_rhs);
            prop_assert_eq!(actual, expected);
        }

        #[test]
        fn f64_matches_native_behavior(native_lhs in f64::arbitrary(), native_rhs in f64::arbitrary()) {
            let (lhs, rhs) = (F64::from(native_lhs), F64::from(native_rhs));
            let actual = lhs.partial_cmp(&rhs);
            let expected = native_lhs.partial_cmp(&native_rhs);
            prop_assert_eq!(actual, expected);
        }
    }
}
