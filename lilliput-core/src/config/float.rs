pub use lilliput_float::PackedFloatValidator;

use super::PackingMode;

#[derive(Default, Clone, PartialEq, Debug)]
pub struct PackedFloatValidation {
    pub f32: PackedFloatValidator<f32>,
    pub f64: PackedFloatValidator<f64>,
}

impl PackedFloatValidation {
    pub fn with_f32(mut self, validator: PackedFloatValidator<f32>) -> Self {
        self.f32 = validator;
        self
    }

    pub fn with_f64(mut self, validator: PackedFloatValidator<f64>) -> Self {
        self.f64 = validator;
        self
    }

    pub fn with_relative(self, max_eps: f64) -> Self {
        self.with_f32(PackedFloatValidator::Relative(max_eps as f32))
            .with_f64(PackedFloatValidator::Relative(max_eps))
    }

    pub fn with_absolute(self, max_eps: f64) -> Self {
        self.with_f32(PackedFloatValidator::Absolute(max_eps as f32))
            .with_f64(PackedFloatValidator::Absolute(max_eps))
    }
}

#[cfg_attr(any(test, feature = "testing"), derive(proptest_derive::Arbitrary))]
#[derive(Default, Clone, PartialEq, Debug)]
pub struct FloatEncoderConfig {
    pub packing: PackingMode,
    #[cfg_attr(
        any(test, feature = "testing"),
        proptest(value = "PackedFloatValidation::default()")
    )]
    pub validation: PackedFloatValidation,
}

impl FloatEncoderConfig {
    pub fn with_packing(mut self, packing: PackingMode) -> Self {
        self.packing = packing;
        self
    }

    pub fn with_validation(mut self, validation: PackedFloatValidation) -> Self {
        self.validation = validation;
        self
    }
}
