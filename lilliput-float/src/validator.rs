use std::num::FpCategory;

#[derive(Clone, PartialEq, Debug)]
pub enum PackedFloatValidator<T> {
    Relative(T),
    Absolute(T),
    Custom(fn(T, T) -> bool),
}

macro_rules! impl_packed_float_validator {
    ($t:ty) => {
        impl Default for PackedFloatValidator<$t> {
            fn default() -> Self {
                Self::Absolute(0.0)
            }
        }

        impl PackedFloatValidator<$t> {
            pub fn validate(&self, before: $t, after: $t) -> bool {
                match *self {
                    Self::Relative(relative_max_eps) => {
                        Self::validate_relative(before, after, relative_max_eps)
                    }
                    Self::Absolute(absolute_max_eps) => {
                        Self::validate_absolute(before, after, absolute_max_eps)
                    }
                    Self::Custom(custom_fn) => Self::validate_custom(before, after, custom_fn),
                }
            }

            fn validate_relative(before: $t, after: $t, relative_max_eps: $t) -> bool {
                let max_eps = before * relative_max_eps;
                Self::validate_absolute(before, after, max_eps)
            }

            fn validate_absolute(before: $t, after: $t, max_eps: $t) -> bool {
                let is_normal_or_subnormal = matches!(
                    before.classify(),
                    FpCategory::Normal | FpCategory::Subnormal
                );

                if is_normal_or_subnormal {
                    (before - after).abs() <= max_eps.abs()
                } else {
                    true
                }
            }

            fn validate_custom(before: $t, after: $t, custom_fn: fn($t, $t) -> bool) -> bool {
                let is_normal_or_subnormal = matches!(
                    before.classify(),
                    FpCategory::Normal | FpCategory::Subnormal
                );

                if is_normal_or_subnormal {
                    (custom_fn)(before, after)
                } else {
                    true
                }
            }
        }
    };
}

impl_packed_float_validator!(f32);
impl_packed_float_validator!(f64);
