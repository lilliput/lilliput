use lilliput_float::{FpPack as _, FpToBeBytes as _, PackedFloat, PackedFloatValidator, F32, F64};

use super::{WithBeBytes, WithValidatedPackedBeBytes};

impl WithBeBytes for f32 {
    #[inline]
    fn with_be_bytes<T, F>(&self, f: F) -> T
    where
        F: FnOnce(&[u8]) -> T,
    {
        let bytes = self.to_be_bytes();
        let width = bytes.len();
        debug_assert_eq!(width, bytes.len());

        f(&bytes)
    }
}

impl WithValidatedPackedBeBytes for f32 {
    type Validator = PackedFloatValidator<f32>;

    #[inline]
    fn with_validated_native_packed_be_bytes<T, F>(&self, validator: &Self::Validator, f: F) -> T
    where
        F: FnOnce(&[u8]) -> T,
    {
        match F32::from(*self).pack_native(validator) {
            PackedFloat::F8(packed) => f(&packed.to_be_bytes()),
            PackedFloat::F16(packed) => f(&packed.to_be_bytes()),
            PackedFloat::F24(packed) => f(&packed.to_be_bytes()),
            PackedFloat::F32(packed) => f(&packed.to_be_bytes()),
            _ => unreachable!(),
        }
    }

    #[inline]
    fn with_validated_optimal_packed_be_bytes<T, F>(&self, validator: &Self::Validator, f: F) -> T
    where
        F: FnOnce(&[u8]) -> T,
    {
        match F32::from(*self).pack_optimal(validator) {
            PackedFloat::F8(packed) => f(&packed.to_be_bytes()),
            PackedFloat::F16(packed) => f(&packed.to_be_bytes()),
            PackedFloat::F24(packed) => f(&packed.to_be_bytes()),
            PackedFloat::F32(packed) => f(&packed.to_be_bytes()),
            _ => unreachable!(),
        }
    }
}

impl WithBeBytes for f64 {
    #[inline]
    fn with_be_bytes<T, F>(&self, f: F) -> T
    where
        F: FnOnce(&[u8]) -> T,
    {
        let bytes = self.to_be_bytes();
        let width = bytes.len();
        debug_assert_eq!(width, bytes.len());

        f(&bytes)
    }
}

impl WithValidatedPackedBeBytes for f64 {
    type Validator = PackedFloatValidator<f64>;

    #[inline]
    fn with_validated_native_packed_be_bytes<T, F>(&self, validator: &Self::Validator, f: F) -> T
    where
        F: FnOnce(&[u8]) -> T,
    {
        match F64::from(*self).pack_native(validator) {
            PackedFloat::F8(packed) => f(&packed.to_be_bytes()),
            PackedFloat::F16(packed) => f(&packed.to_be_bytes()),
            PackedFloat::F24(packed) => f(&packed.to_be_bytes()),
            PackedFloat::F32(packed) => f(&packed.to_be_bytes()),
            PackedFloat::F40(packed) => f(&packed.to_be_bytes()),
            PackedFloat::F48(packed) => f(&packed.to_be_bytes()),
            PackedFloat::F56(packed) => f(&packed.to_be_bytes()),
            PackedFloat::F64(packed) => f(&packed.to_be_bytes()),
        }
    }

    #[inline]
    fn with_validated_optimal_packed_be_bytes<T, F>(&self, validator: &Self::Validator, f: F) -> T
    where
        F: FnOnce(&[u8]) -> T,
    {
        match F64::from(*self).pack_optimal(validator) {
            PackedFloat::F8(packed) => f(&packed.to_be_bytes()),
            PackedFloat::F16(packed) => f(&packed.to_be_bytes()),
            PackedFloat::F24(packed) => f(&packed.to_be_bytes()),
            PackedFloat::F32(packed) => f(&packed.to_be_bytes()),
            PackedFloat::F40(packed) => f(&packed.to_be_bytes()),
            PackedFloat::F48(packed) => f(&packed.to_be_bytes()),
            PackedFloat::F56(packed) => f(&packed.to_be_bytes()),
            PackedFloat::F64(packed) => f(&packed.to_be_bytes()),
        }
    }
}
