use crate::{FpTruncate, PackedFloat, PackedFloatValidator, F16, F24, F32, F40, F48, F56, F64, F8};

pub trait FpPack {
    type Validator;

    fn pack_native(self, validator: &Self::Validator) -> PackedFloat;
    fn pack_optimal(self, validator: &Self::Validator) -> PackedFloat;
}

macro_rules! truncate_validated {
    ($src:ty => $dst:ty, $native:expr, $validate:expr) => {{
        let (native, validate) = ($native, $validate);

        let non_packed: $src = native.into();

        FpTruncate::<$dst>::try_truncate(non_packed)
            .ok()
            .and_then(|(truncated, packed)| {
                if (validate)(non_packed, truncated) {
                    Some(packed)
                } else {
                    None
                }
            })
    }};
}

impl FpPack for F32 {
    type Validator = PackedFloatValidator<f32>;

    #[inline]
    fn pack_native(self, validator: &Self::Validator) -> PackedFloat {
        #[allow(unused_variables)]
        let non_packed: f32 = self.into();

        #[allow(unused_variables)]
        let validate = |value: F32, packed: F32| {
            let value: f32 = value.into();
            let packed: f32 = packed.into();
            validator.validate(value, packed)
        };

        #[cfg(feature = "native-f16")]
        if let Some(packed) = truncate_validated!(F32 => F16, non_packed, validate) {
            PackedFloat::F16(packed)
        } else {
            PackedFloat::F32(self)
        }

        #[cfg(not(feature = "native-f16"))]
        PackedFloat::F32(self)
    }

    #[inline]
    fn pack_optimal(self, validator: &Self::Validator) -> PackedFloat {
        let non_packed: f32 = self.into();

        let validate = |value: F32, packed: F32| {
            let value: f32 = value.into();
            let packed: f32 = packed.into();
            validator.validate(value, packed)
        };

        if let Some(packed) = truncate_validated!(F32 => F16, non_packed, validate) {
            if let Some(packed) = truncate_validated!(F32 => F8, non_packed, validate) {
                PackedFloat::F8(packed)
            } else {
                PackedFloat::F16(packed)
            }
        } else {
            #[allow(clippy::collapsible_else_if)]
            if let Some(packed) = truncate_validated!(F32 => F24, non_packed, validate) {
                PackedFloat::F24(packed)
            } else {
                PackedFloat::F32(self)
            }
        }
    }
}

impl FpPack for F64 {
    type Validator = PackedFloatValidator<f64>;

    #[inline]
    fn pack_native(self, validator: &Self::Validator) -> PackedFloat {
        let non_packed: f64 = self.into();

        let validate = |value: F64, packed: F64| {
            let value: f64 = value.into();
            let packed: f64 = packed.into();
            validator.validate(value, packed)
        };

        if let Some(packed) = truncate_validated!(F64 => F32, non_packed, validate) {
            #[cfg(feature = "native-f16")]
            if let Some(packed) = truncate_validated!(F64 => F16, non_packed, validate) {
                PackedFloat::F16(packed)
            } else {
                PackedFloat::F32(packed)
            }

            #[cfg(not(feature = "native-f16"))]
            PackedFloat::F32(packed)
        } else {
            PackedFloat::F64(self)
        }
    }

    #[inline]
    fn pack_optimal(self, validator: &Self::Validator) -> PackedFloat {
        let non_packed: f64 = self.into();

        let validate = |value: F64, packed: F64| {
            let value: f64 = value.into();
            let packed: f64 = packed.into();
            validator.validate(value, packed)
        };

        if let Some(packed) = truncate_validated!(F64 => F32, non_packed, validate) {
            if let Some(packed) = truncate_validated!(F64 => F16, non_packed, validate) {
                if let Some(packed) = truncate_validated!(F64 => F8, non_packed, validate) {
                    PackedFloat::F8(packed)
                } else {
                    PackedFloat::F16(packed)
                }
            } else if let Some(packed) = truncate_validated!(F64 => F24, non_packed, validate) {
                PackedFloat::F24(packed)
            } else {
                PackedFloat::F32(packed)
            }
        } else if let Some(packed) = truncate_validated!(F64 => F48, non_packed, validate) {
            if let Some(packed) = truncate_validated!(F64 => F40, non_packed, validate) {
                PackedFloat::F40(packed)
            } else {
                PackedFloat::F48(packed)
            }
        } else if let Some(packed) = truncate_validated!(F64 => F56, non_packed, validate) {
            PackedFloat::F56(packed)
        } else {
            PackedFloat::F64(self)
        }
    }
}
