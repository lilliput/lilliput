use std::num::FpCategory;

use crate::bits::{FpFromBits, FpToBits};
use crate::classify::FpClassify;
use crate::floats::{F16, F24, F32, F40, F48, F56, F64, F8};
use crate::repr::FpRepr;
use crate::sealed::Sealed;

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum FpTruncateError {
    Overflow,
    Underflow,
}

pub trait FpTruncate<T>: Sized + Sealed {
    fn truncate(self) -> T;
    fn try_truncate(self) -> Result<T, FpTruncateError>;
}

// Source: https://github.com/rust-lang/compiler-builtins/blob/3dea633a80d32da75e923a940d16ce98cce74822/src/float/trunc.rs#L4
macro_rules! impl_float_truncate {
    ($src:ty => [$($dst:ty),* $(,)?]) => {
        $(
            impl_float_truncate!($src => $dst);
        )*
    };
    ($src:ty => $dst:ty) => {
        impl FpTruncate<$dst> for $src {
            fn truncate(self) -> $dst {
                type SrcBits = <$src as FpRepr>::Bits;
                type DstBits = <$dst as FpRepr>::Bits;

                let src_bits: u32 = <$src>::BITS;

                let src_exp_bias: SrcBits = <$src>::EXPONENT_BIAS;

                let src_min_normal: SrcBits = <$src>::IMPLICIT_BIT;
                let src_significand_mask: SrcBits = <$src>::SIGNIFICAND_MASK;
                let src_infinity: SrcBits = <$src>::EXPONENT_MASK;
                let src_sign_mask: SrcBits = <$src>::SIGN_MASK;
                let src_abs_mask: SrcBits = src_sign_mask - 1;
                let round_mask: SrcBits =
                    (1 << (<$src>::SIGNIFICAND_BITS - <$dst>::SIGNIFICAND_BITS)) - 1;
                let halfway: SrcBits = 1 << (<$src>::SIGNIFICAND_BITS - <$dst>::SIGNIFICAND_BITS - 1);
                let src_qnan: SrcBits = 1 << (<$src>::SIGNIFICAND_BITS - 1);
                let src_nan_code: SrcBits = src_qnan - 1;

                let dst_bits: u32 = <$dst>::BITS;

                let dst_inf_exp: DstBits = <$dst>::EXPONENT_MAX;
                let dst_exp_bias: DstBits = <$dst>::EXPONENT_BIAS;

                let underflow_exponent: SrcBits = (src_exp_bias as SrcBits) + 1 - dst_exp_bias as SrcBits;
                let overflow_exponent: SrcBits = src_exp_bias + (dst_inf_exp - dst_exp_bias) as SrcBits;
                let underflow: SrcBits = underflow_exponent << <$src>::SIGNIFICAND_BITS;
                let overflow: SrcBits = overflow_exponent << <$src>::SIGNIFICAND_BITS;

                let dst_qnan: DstBits = 1 << (<$dst>::SIGNIFICAND_BITS - 1);
                let dst_nan_code: DstBits = dst_qnan - 1;

                let bits = self.to_bits();

                let sign_bits_delta: u32 = <$src>::SIGNIFICAND_BITS - <$dst>::SIGNIFICAND_BITS;

                // Break `src` into a sign and representation of the absolute value.
                let src_abs: SrcBits = bits & src_abs_mask;
                let sign: SrcBits = bits & src_sign_mask;
                let mut abs_result: DstBits;

                if src_abs.wrapping_sub(underflow) < src_abs.wrapping_sub(overflow) {
                    // The exponent of `src` is within the range of normal numbers in the
                    // destination format. We can convert by simply right-shifting with
                    // rounding and adjusting the exponent.
                    abs_result = (src_abs >> sign_bits_delta) as DstBits;
                    let tmp: DstBits =
                        (src_exp_bias.wrapping_sub(dst_exp_bias as SrcBits) << <$dst>::SIGNIFICAND_BITS) as DstBits;
                    abs_result = abs_result.wrapping_sub(tmp.into());

                    let round_bits = src_abs & round_mask;

                    if round_bits > halfway {
                        // Round to nearest.
                        abs_result += 1;
                    } else if round_bits == halfway {
                        // Tie to even.
                        abs_result += abs_result & 1;
                    };
                } else if src_abs > src_infinity {
                    // `src` is NaN.
                    //
                    // Conjure the result by beginning with infinity, setting the qNaN
                    // bit and inserting the (truncated) trailing NaN field.
                    abs_result = dst_inf_exp << <$dst>::SIGNIFICAND_BITS;
                    abs_result |= dst_qnan;
                    abs_result |= dst_nan_code
                        & ((src_abs & src_nan_code) >> (<$src>::SIGNIFICAND_BITS - <$dst>::SIGNIFICAND_BITS)) as DstBits;
                } else if src_abs >= overflow {
                    // src overflows to infinity.
                    abs_result = dst_inf_exp << <$dst>::SIGNIFICAND_BITS;
                } else {
                    // `src` underflows on conversion to the destination type or is an exact zero.
                    //
                    // The result may be a subnormal or zero.  Extract the exponent
                    // to get the shift amount for the denormalization.
                    let src_exp: SrcBits = src_abs >> <$src>::SIGNIFICAND_BITS;
                    let shift: u32 = (src_exp_bias - (dst_exp_bias as SrcBits) + 1 - src_exp) as u32;

                    let significand = (bits & src_significand_mask) | src_min_normal;

                    // Right shift by the denormalization amount with sticky.
                    if shift > <$src>::SIGNIFICAND_BITS {
                        abs_result = 0;
                    } else {
                        let sticky: SrcBits = if (significand << (src_bits - shift)) != 0 {
                            1
                        } else {
                            0
                        };
                        let denormalized_significand: SrcBits = significand >> shift | sticky;
                        abs_result =
                            (denormalized_significand >> (<$src>::SIGNIFICAND_BITS - <$dst>::SIGNIFICAND_BITS)) as DstBits;
                        let round_bits = denormalized_significand & round_mask;
                        // Round to nearest
                        if round_bits > halfway {
                            abs_result += 1;
                        }
                        // Ties to even
                        else if round_bits == halfway {
                            abs_result += abs_result & 1;
                        };
                    }
                }

                // Apply the sign-bit to the absolute value.
                let result: DstBits = abs_result | sign.wrapping_shr(src_bits - dst_bits) as DstBits;

                <$dst>::from_bits(result)
            }

            fn try_truncate(self) -> Result<$dst, FpTruncateError> {
                let output: $dst = self.truncate();

                let before = self.classify();
                let after = output.classify();

                use FpCategory::*;

                match (before == after, before, after) {
                    (true, _, _) => Ok(output),
                    (false, Normal, Infinite) => Err(FpTruncateError::Overflow),
                    (false, Normal, Subnormal) => Err(FpTruncateError::Underflow),
                    (false, Normal, Zero) => Err(FpTruncateError::Underflow),
                    (false, Subnormal, Zero) => Err(FpTruncateError::Underflow),
                    (false, _, _) => unreachable!(),
                }
            }
        }
    };
}

impl_float_truncate!(F8 => []);
impl_float_truncate!(F16 => [F8]);
impl_float_truncate!(F24 => [F8, F16]);
impl_float_truncate!(F32 => [F8, F16, F24]);
impl_float_truncate!(F40 => [F8, F16, F24, F32]);
impl_float_truncate!(F48 => [F8, F16, F24, F32, F40]);
impl_float_truncate!(F56 => [F8, F16, F24, F32, F40, F48]);
impl_float_truncate!(F64 => [F8, F16, F24, F32, F40, F48, F56]);

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use super::*;

    proptest! {
        #[cfg(feature = "native-f16")]
        #[test]
        fn f64_to_f16_matches_native_behavior(native in f64::arbitrary()) {
            let subject = F64::from(native);
            let actual: F16 = subject.truncate();
            let expected = F16::from(native as f16);
            prop_assert_eq!(actual, expected);
        }

        #[test]
        fn f64_to_f32_matches_native_behavior(native in f64::arbitrary()) {
            let subject = F64::from(native);
            let actual: F32 = subject.truncate();
            let expected = F32::from(native as f32);
            prop_assert_eq!(actual, expected);
        }

        #[test]
        fn truncate_f64_to_f8(native in f64::arbitrary()) {
            let subject = F64::from(native);
            let _: F8 = subject.truncate();
        }

        #[test]
        fn truncate_f64_to_f16(native in f64::arbitrary()) {
            let subject = F64::from(native);
            let _: F16 = subject.truncate();
        }

        #[test]
        fn truncate_f64_to_f24(native in f64::arbitrary()) {
            let subject = F64::from(native);
            let _: F24 = subject.truncate();
        }

        #[test]
        fn truncate_f64_to_f32(native in f64::arbitrary()) {
            let subject = F64::from(native);
            let _: F32 = subject.truncate();
        }

        #[test]
        fn truncate_f64_to_f40(native in f64::arbitrary()) {
            let subject = F64::from(native);
            let _: F40 = subject.truncate();
        }

        #[test]
        fn truncate_f64_to_f48(native in f64::arbitrary()) {
            let subject = F64::from(native);
            let _: F48 = subject.truncate();
        }

        #[test]
        fn truncate_f64_to_f56(native in f64::arbitrary()) {
            let subject = F64::from(native);
            let _: F56 = subject.truncate();
        }
    }
}
