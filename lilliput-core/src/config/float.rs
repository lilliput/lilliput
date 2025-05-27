pub use lilliput_float::PackedFloatValidator;

use super::PackingMode;

/// Validation for float-packing.
#[derive(Default, Clone, PartialEq, Debug)]
pub struct PackedFloatValidation {
    /// Validation for float-packing of `f32` values.
    pub f32: PackedFloatValidator<f32>,
    /// Validation for float-packing of `f64` values.
    pub f64: PackedFloatValidator<f64>,
}

impl PackedFloatValidation {
    /// Sets validation for float-packing of `f32` values, returning `self`.
    pub fn with_f32(mut self, validator: PackedFloatValidator<f32>) -> Self {
        self.f32 = validator;
        self
    }

    /// Sets validation for float-packing of `f64` values, returning `self`.
    pub fn with_f64(mut self, validator: PackedFloatValidator<f64>) -> Self {
        self.f64 = validator;
        self
    }

    /// Sets validation for float-packing values based on relative maximum epsilon, returning `self`.
    pub fn with_relative(self, max_eps: f64) -> Self {
        self.with_f32(PackedFloatValidator::Relative(max_eps as f32))
            .with_f64(PackedFloatValidator::Relative(max_eps))
    }

    /// Sets validation for float-packing values based on absolute maximum epsilon, returning `self`.
    pub fn with_absolute(self, max_eps: f64) -> Self {
        self.with_f32(PackedFloatValidator::Absolute(max_eps as f32))
            .with_f64(PackedFloatValidator::Absolute(max_eps))
    }
}

/// Configuration used for encoding integer values.
#[cfg_attr(any(test, feature = "testing"), derive(proptest_derive::Arbitrary))]
#[derive(Default, Clone, PartialEq, Debug)]
pub struct FloatEncoderConfig {
    /// Packing mode for encoding.
    pub packing: PackingMode,
    #[cfg_attr(
        any(test, feature = "testing"),
        proptest(value = "PackedFloatValidation::default()")
    )]
    /// Validation for float-packing.
    pub validation: PackedFloatValidation,
}

impl FloatEncoderConfig {
    /// Sets packing-modes to `packing`, returning `self`.
    pub fn with_packing(mut self, packing: PackingMode) -> Self {
        self.packing = packing;
        self
    }

    /// Sets float-validation to `validation`, returning `self`.
    pub fn with_validation(mut self, validation: PackedFloatValidation) -> Self {
        self.validation = validation;
        self
    }
}
