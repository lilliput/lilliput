use crate::bits::{FpFromBits, FpToBits};
use crate::floats::{F16, F24, F32, F40, F48, F56, F64, F8};
use crate::repr::FpRepr;
use crate::sealed::Sealed;

pub trait FpTruncate<T>: Sized + Sealed {
    fn truncate(self) -> (Self, T);
}

// Source: https://github.com/rust-lang/compiler-builtins/blob/3dea633a80d32da75e923a940d16ce98cce74822/src/float/trunc.rs#L4
macro_rules! impl_float_truncate {
    ($src:ty => [$($dst:ty),* $(,)?]) => {
        $(
            impl_float_truncate!($src => $dst);
        )*
    };
    (F64 => F32) => {
        impl FpTruncate<F32> for F64 {
            fn truncate(self) -> (F64, F32) {
                let value: f64 = self.into();

                let dst_val = value as f32;
                let src_val = dst_val as f64;

                (F64::from(src_val), F32::from(dst_val))
            }
        }
    };
    ($src:ty => $dst:ty) => {
        impl FpTruncate<$dst> for $src {
            fn truncate(self) -> ($src, $dst) {
                type Src = $src;
                type Dst = $dst;

                type SrcBits = <Src as FpRepr>::Bits;
                type DstBits = <Dst as FpRepr>::Bits;

                let src = self;

                let src_bits: u32 = Src::BITS;

                let src_exp_bias: SrcBits = Src::EXPONENT_BIAS;
                let dst_exp_bias: DstBits = Dst::EXPONENT_BIAS;

                let src_infinity: SrcBits = Src::EXPONENT_MASK;

                let src_abs_mask: SrcBits = Src::SIGN_MASK - 1;
                let round_mask: SrcBits = (1 << (Src::SIGNIFICAND_BITS - Dst::SIGNIFICAND_BITS)) - 1;
                let halfway: SrcBits = 1 << (Src::SIGNIFICAND_BITS - Dst::SIGNIFICAND_BITS - 1);
                let src_qnan: SrcBits = 1 << (Src::SIGNIFICAND_BITS - 1);
                let src_nan_code: SrcBits = src_qnan - 1;

                let src_inf_exp: SrcBits = Src::EXPONENT_MAX;
                let dst_inf_exp: DstBits = Dst::EXPONENT_MAX;

                let dst_qnan: DstBits = 1 << (Dst::SIGNIFICAND_BITS - 1);
                let dst_nan_code: DstBits = dst_qnan - 1;

                let underflow_exponent: SrcBits = src_exp_bias + 1 - (dst_exp_bias as SrcBits);
                let overflow_exponent: SrcBits = src_exp_bias + (dst_inf_exp - dst_exp_bias) as SrcBits;

                let underflow: SrcBits = underflow_exponent << Src::SIGNIFICAND_BITS;
                let overflow: SrcBits = overflow_exponent << Src::SIGNIFICAND_BITS;

                let bits_delta: u32 = Src::BITS - Dst::BITS;
                let significand_bits_delta: u32 = Src::SIGNIFICAND_BITS - Dst::SIGNIFICAND_BITS;

                let bits: SrcBits = src.to_bits();
                let src_abs: SrcBits = bits & src_abs_mask;

                let src_sign: SrcBits = bits & Src::SIGN_MASK;
                let mut src_exponent: SrcBits = bits & Src::EXPONENT_MASK;
                let mut src_significand: SrcBits = bits & Src::SIGNIFICAND_MASK;

                let dst_sign: DstBits = (src_sign >> bits_delta) as DstBits;
                let dst_exponent: DstBits;
                let mut dst_significand: DstBits;

                let exp_bias_delta: SrcBits = src_exp_bias.wrapping_sub(dst_exp_bias as SrcBits);
                let shifted_exp_bias_delta: SrcBits = exp_bias_delta << Src::SIGNIFICAND_BITS;

                if src_abs.wrapping_sub(underflow) < src_abs.wrapping_sub(overflow) {
                    // The value remains normal.

                    // The exponent is within the range of normal numbers in the
                    // destination format.  We can convert by simply right-shifting with
                    // rounding and adjusting the exponent.

                    dst_exponent = (src_exponent.wrapping_sub(shifted_exp_bias_delta) >> significand_bits_delta) as DstBits;
                    dst_significand = (src_significand >> significand_bits_delta) as DstBits;

                    let round_bits = src_significand & round_mask;

                    if round_bits > halfway {
                        // Round significand to nearest.
                        dst_significand += 1;
                    } else if round_bits == halfway {
                        // Tie significand to even.
                        dst_significand += dst_significand & 1;
                    }

                    src_significand = ((dst_significand as SrcBits) << significand_bits_delta) & Src::SIGNIFICAND_MASK;
                } else if src_abs > src_infinity {
                    // The value is NaN.

                    // Conjure the result by beginning with infinity, setting the qNaN
                    // bit and inserting the (truncated) trailing NaN field.

                    dst_exponent = dst_inf_exp << Dst::SIGNIFICAND_BITS;

                    dst_significand = dst_qnan | dst_nan_code & ((src_significand & src_nan_code) >> significand_bits_delta) as DstBits;
                } else if src_abs >= overflow {
                    // Value overflows to infinity.

                    dst_exponent = dst_inf_exp << Dst::SIGNIFICAND_BITS;
                    src_exponent = src_inf_exp << Src::SIGNIFICAND_BITS;

                    dst_significand = 0;
                    src_significand = 0;
                } else {
                    // Value underflows on conversion to the destination type
                    // or is an exact zero. The result may be a denormal or zero.

                    // Extract the exponent to get the shift amount for the denormalization.

                    let src_exp = src_abs >> Src::SIGNIFICAND_BITS;
                    let shift: u32 = (src_exp_bias - dst_exp_bias as SrcBits + 1 - src_exp) as u32;

                    let significand: SrcBits = (bits & Src::SIGNIFICAND_MASK) | Src::IMPLICIT_BIT;

                    if shift >= Src::SIGNIFICAND_BITS {
                        // Value underflows to zero.

                        dst_exponent = 0;
                        src_exponent = 0;

                        dst_significand = 0;
                        src_significand = 0;
                    } else {
                        // Value underflows to denormal.

                        dst_exponent = 0;

                        let sticky: SrcBits = if (significand << (src_bits - shift)) != 0 {
                            1
                        } else {
                            0
                        };

                        // Right shift by the denormalization amount with sticky.
                        let denormalized: SrcBits = (significand >> shift) | sticky;
                        dst_significand = (denormalized >> significand_bits_delta) as DstBits;

                        let round_bits = denormalized & round_mask;
                        let round_bit: DstBits = 1;

                        if round_bits > halfway {
                            // Round to nearest
                            dst_significand += round_bit;
                        } else if round_bits == halfway {
                            // Ties to even
                            dst_significand += dst_significand & round_bit;
                        };

                        dst_significand &= Dst::SIGNIFICAND_MASK;

                        if dst_significand == 0 {
                            src_exponent = 0;
                            src_significand = 0;
                        } else {
                            let scale = dst_significand.leading_zeros() - Dst::IMPLICIT_BIT.leading_zeros();

                            src_exponent = (exp_bias_delta - (scale as SrcBits) + 1) << Src::SIGNIFICAND_BITS;
                            src_significand = (dst_significand as SrcBits).wrapping_shl(significand_bits_delta + scale);

                            src_exponent &= Src::EXPONENT_MASK;
                            src_significand &= Src::SIGNIFICAND_MASK;
                        }
                    }
                }

                // src_exponent &= Src::EXPONENT_MASK;

                let src_result_bits: SrcBits = src_sign | src_exponent | src_significand;
                let dst_result_bits: DstBits = dst_sign | dst_exponent | dst_significand;

                let src_val = Src::from_bits(src_result_bits);
                let dst_val = Dst::from_bits(dst_result_bits);

                (src_val, dst_val)
            }
        }
    };
}

#[cfg(feature = "full")]
impl_float_truncate!(F8 => []);
#[cfg(feature = "full")]
impl_float_truncate!(F16 => [F8]);
#[cfg(feature = "full")]
impl_float_truncate!(F24 => [F8, F16]);

impl_float_truncate!(F32 => [F8, F16, F24]);
#[cfg(feature = "full")]
impl_float_truncate!(F40 => [F8, F16, F24, F32]);
#[cfg(feature = "full")]
impl_float_truncate!(F48 => [F8, F16, F24, F32, F40]);
#[cfg(feature = "full")]
impl_float_truncate!(F56 => [F8, F16, F24, F32, F40, F48]);

impl_float_truncate!(F64 => [F8, F16, F24, F32, F40, F48, F56]);

#[cfg(test)]
mod tests {
    use std::num::FpCategory;

    use proptest::prelude::*;

    use crate::FpClassify as _;

    use super::*;

    fn assert_valid_category(before: FpCategory, after: FpCategory) -> Result<(), TestCaseError> {
        match before {
            FpCategory::Nan => {
                prop_assert_eq!(after, FpCategory::Nan);
            }
            FpCategory::Infinite => {
                prop_assert_eq!(after, FpCategory::Infinite);
            }
            FpCategory::Zero => {
                prop_assert_eq!(after, FpCategory::Zero);
            }
            FpCategory::Subnormal => {
                prop_assert!(matches!(
                    after,
                    FpCategory::Zero | FpCategory::Subnormal | FpCategory::Infinite
                ));
            }
            FpCategory::Normal => {
                prop_assert!(matches!(
                    after,
                    FpCategory::Zero
                        | FpCategory::Subnormal
                        | FpCategory::Normal
                        | FpCategory::Infinite
                ));
            }
        }

        Ok(())
    }

    proptest! {
        // MARK: - F32

        #[test]
        fn truncate_f32_to_f8(native in f32::arbitrary()) {
            let subject = F32::from(native);
            let (src_actual, dst_actual): (F32, F8) = subject.truncate();

            let category_before = subject.classify();
            let src_category_after = src_actual.classify();
            let dst_category_after = dst_actual.classify();

            assert_valid_category(category_before, src_category_after)?;
            assert_valid_category(category_before, dst_category_after)?;
        }

        #[test]
        fn truncate_f32_to_f16(native in f32::arbitrary()) {
            let subject = F32::from(native);
            let (src_actual, dst_actual): (F32, F16) = subject.truncate();

            let category_before = subject.classify();
            let src_category_after = src_actual.classify();
            let dst_category_after = dst_actual.classify();

            assert_valid_category(category_before, src_category_after)?;
            assert_valid_category(category_before, dst_category_after)?;

            #[cfg(feature = "native-f16")]
            {
                let dst_native = native as f16;
                let src_native = dst_native as f32;

                let dst_expected = F16::from(dst_native);
                let src_expected = F32::from(src_native);

                prop_assert_eq!(dst_actual, dst_expected);
                prop_assert_eq!(src_actual, src_expected);
            }

            #[cfg(not(feature = "native-f16"))]
            let _ = (src_actual, dst_actual);
        }

        #[test]
        fn truncate_f32_to_f24(native in f32::arbitrary()) {
            let subject = F32::from(native);
            let (src_actual, dst_actual): (F32, F24) = subject.truncate();

            let category_before = subject.classify();
            let src_category_after = src_actual.classify();
            let dst_category_after = dst_actual.classify();

            assert_valid_category(category_before, src_category_after)?;
            assert_valid_category(category_before, dst_category_after)?;
        }

        // MARK: - F64

        #[test]
        fn truncate_f64_to_f8(native in f64::arbitrary()) {
            let subject = F64::from(native);
            let (src_actual, dst_actual): (F64, F8) = subject.truncate();

            let category_before = subject.classify();
            let src_category_after = src_actual.classify();
            let dst_category_after = dst_actual.classify();

            assert_valid_category(category_before, src_category_after)?;
            assert_valid_category(category_before, dst_category_after)?;
        }

        #[test]
        fn truncate_f64_to_f16(native in f64::arbitrary()) {
            let subject = F64::from(native);
            let (src_actual, dst_actual): (F64, F16) = subject.truncate();

            let category_before = subject.classify();
            let src_category_after = src_actual.classify();
            let dst_category_after = dst_actual.classify();

            assert_valid_category(category_before, src_category_after)?;
            assert_valid_category(category_before, dst_category_after)?;
        }

        #[test]
        fn truncate_f64_to_f24(native in f64::arbitrary()) {
            let subject = F64::from(native);
            let (src_actual, dst_actual): (F64, F24) = subject.truncate();

            let category_before = subject.classify();
            let src_category_after = src_actual.classify();
            let dst_category_after = dst_actual.classify();

            assert_valid_category(category_before, src_category_after)?;
            assert_valid_category(category_before, dst_category_after)?;
        }

        #[test]
        fn truncate_f64_to_f32(native in f64::arbitrary()) {
            let subject = F64::from(native);
            let (src_actual, dst_actual): (F64, F32) = subject.truncate();

            let category_before = subject.classify();
            let src_category_after = src_actual.classify();
            let dst_category_after = dst_actual.classify();

            assert_valid_category(category_before, src_category_after)?;
            assert_valid_category(category_before, dst_category_after)?;

            let dst_native = native as f32;
            let src_native = dst_native as f64;

            let dst_expected = F32::from(dst_native);
            let src_expected = F64::from(src_native);

            prop_assert_eq!(dst_actual, dst_expected);
            prop_assert_eq!(src_actual, src_expected);
        }

        #[test]
        fn truncate_f64_to_f40(native in f64::arbitrary()) {
            let subject = F64::from(native);
            let (src_actual, dst_actual): (F64, F40) = subject.truncate();

            let category_before = subject.classify();
            let src_category_after = src_actual.classify();
            let dst_category_after = dst_actual.classify();

            assert_valid_category(category_before, src_category_after)?;
            assert_valid_category(category_before, dst_category_after)?;
        }

        #[test]
        fn truncate_f64_to_f48(native in f64::arbitrary()) {
            let subject = F64::from(native);
            let (src_actual, dst_actual): (F64, F48) = subject.truncate();

            let category_before = subject.classify();
            let src_category_after = src_actual.classify();
            let dst_category_after = dst_actual.classify();

            assert_valid_category(category_before, src_category_after)?;
            assert_valid_category(category_before, dst_category_after)?;
        }

        #[test]
        fn truncate_f64_to_f56(native in f64::arbitrary()) {
            let subject = F64::from(native);
            let (src_actual, dst_actual): (F64, F56) = subject.truncate();

            let category_before = subject.classify();
            let src_category_after = src_actual.classify();
            let dst_category_after = dst_actual.classify();

            assert_valid_category(category_before, src_category_after)?;
            assert_valid_category(category_before, dst_category_after)?;
        }
    }
}
