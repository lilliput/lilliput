use crate::bits::{FpFromBits, FpToBits};
use crate::floats::{F16, F24, F32, F40, F48, F56, F64, F8};
use crate::repr::FpRepr;
use crate::sealed::Sealed;

pub trait FpExtend<T>: Sealed {
    fn extend(self) -> T;
}

// Source: https://github.com/rust-lang/compiler-builtins/blob/3dea633a80d32da75e923a940d16ce98cce74822/src/float/extend.rs#L4
macro_rules! impl_float_extend {
    ($src:ty => [$($dst:ty),* $(,)?]) => {
        $(
            impl_float_extend!($src => $dst);
        )*
    };
    (F32 => F64) => {
        impl FpExtend<F64> for F32 {
            fn extend(self) -> F64 {
                let value: f32 = self.into();

                F64::from(value as f64)
            }
        }
    };
    ($src:ty => $dst:ty) => {
        impl FpExtend<$dst> for $src {
            fn extend(self) -> $dst {
                type SrcBits = <$src as FpRepr>::Bits;
                type DstBits = <$dst as FpRepr>::Bits;

                let src_bits: u32 = <$src>::BITS;
                let src_sign_bits: u32 = <$src>::SIGNIFICAND_BITS;
                let src_exp_bias: SrcBits = <$src>::EXPONENT_BIAS;
                let src_min_normal: SrcBits = <$src>::IMPLICIT_BIT;
                let src_infinity: SrcBits = <$src>::EXPONENT_MASK;
                let src_sign_mask: SrcBits = <$src>::SIGN_MASK;
                let src_abs_mask: SrcBits = src_sign_mask - 1;
                let src_qnan: SrcBits = <$src>::SIGNIFICAND_MASK;
                let src_nan_code: SrcBits = src_qnan - 1;

                let dst_bits: u32 = <$dst>::BITS;
                let dst_sign_bits: u32 = <$dst>::SIGNIFICAND_BITS;
                let dst_inf_exp: DstBits = <$dst>::EXPONENT_MAX;
                let dst_exp_bias: DstBits = <$dst>::EXPONENT_BIAS;
                let dst_min_normal: DstBits = <$dst>::IMPLICIT_BIT;

                let bits = self.to_bits();

                let sign_bits_delta: u32 = dst_sign_bits - src_sign_bits;
                let exp_bias_delta: DstBits = dst_exp_bias - src_exp_bias as DstBits;
                let src_abs: SrcBits = bits & src_abs_mask;
                let mut abs_result: DstBits = 0;

                if src_abs.wrapping_sub(src_min_normal) < src_infinity.wrapping_sub(src_min_normal)
                {
                    // `src` is a normal number.
                    //
                    // Extend to the destination type by shifting the significand and
                    // exponent into the proper position and re-biasing the exponent.
                    let abs_dst: DstBits = src_abs as DstBits;
                    let bias_dst: DstBits = exp_bias_delta;
                    abs_result = abs_dst.wrapping_shl(sign_bits_delta);
                    abs_result += bias_dst.wrapping_shl(dst_sign_bits);
                } else if src_abs >= src_infinity {
                    // `src` is NaN or infinity.
                    //
                    // Conjure the result by beginning with infinity, then setting the qNaN
                    // bit (if needed) and right-aligning the rest of the trailing NaN
                    // payload field.
                    let qnan_dst: DstBits = (src_abs & src_qnan) as DstBits;
                    let nan_code_dst: DstBits = (src_abs & src_nan_code) as DstBits;
                    let inf_exp_dst: DstBits = dst_inf_exp;

                    abs_result = inf_exp_dst.wrapping_shl(dst_sign_bits);
                    abs_result |= qnan_dst.wrapping_shl(sign_bits_delta);
                    abs_result |= nan_code_dst.wrapping_shl(sign_bits_delta);
                } else if src_abs != 0 {
                    // `src` is subnormal.
                    //
                    // Renormalize the significand and clear the leading bit, then insert
                    // the correct adjusted exponent in the destination type.
                    let scale: u32 = src_abs.leading_zeros() - src_min_normal.leading_zeros();
                    // Safety: The number of bits in a native int will fit in all native integer types:
                    let scale_dst: DstBits = scale as DstBits;
                    let abs_dst: DstBits = src_abs as DstBits;
                    let bias_dst: DstBits = if exp_bias_delta != 0 {
                        exp_bias_delta + 1 - scale_dst
                    } else {
                        0
                    };
                    abs_result = abs_dst.wrapping_shl((sign_bits_delta as u32) + (scale as u32));
                    abs_result =
                        (abs_result ^ dst_min_normal) | (bias_dst.wrapping_shl(dst_sign_bits));
                }

                let sign_result: DstBits = (bits & src_sign_mask) as DstBits;
                let result: DstBits =
                    abs_result | (sign_result.wrapping_shl(dst_bits - src_bits));

                <$dst>::from_bits(result)
            }
        }
    };
}

impl_float_extend!(F8 => [F16, F24, F32, F40, F48, F56, F64]);
impl_float_extend!(F16 => [F24, F32, F40, F48, F56, F64]);
impl_float_extend!(F24 => [F32, F40, F48, F56, F64]);
impl_float_extend!(F32 => [F40, F48, F56, F64]);
impl_float_extend!(F40 => [F48, F56, F64]);
impl_float_extend!(F48 => [F56, F64]);
impl_float_extend!(F56 => [F64]);
impl_float_extend!(F64 => []);

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use super::*;

    proptest! {
        #[test]
        fn f32_to_f64_matches_native_behavior(native in f32::arbitrary()) {
            let subject = F32::from(native);
            let actual: F64 = subject.extend();
            let expected = F64::from(native as f64);
            prop_assert_eq!(actual, expected);
        }

        #[test]
        fn extend_f32_to_f40(native in f32::arbitrary()) {
            let subject = F32::from(native);
            let _: F40 = subject.extend();
        }

        #[test]
        fn extend_f32_to_f48(native in f32::arbitrary()) {
            let subject = F32::from(native);
            let _: F48 = subject.extend();
        }

        #[test]
        fn extend_f32_to_f56(native in f32::arbitrary()) {
            let subject = F32::from(native);
            let _: F56 = subject.extend();
        }

        #[test]
        fn extend_f32_to_f64(native in f32::arbitrary()) {
            let subject = F32::from(native);
            let _: F64 = subject.extend();
        }
    }
}
