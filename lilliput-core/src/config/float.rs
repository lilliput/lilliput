use super::PackingMode;

#[derive(Clone, PartialEq, Debug)]
pub struct MaxFloatEpsilon {
    pub max_eps_f32: fn(f32) -> f32,
    pub max_eps_f64: fn(f64) -> f64,
}

impl Default for MaxFloatEpsilon {
    fn default() -> Self {
        Self::exact()
    }
}

impl MaxFloatEpsilon {
    pub fn new(max_eps_f32: fn(f32) -> f32, max_eps_f64: fn(f64) -> f64) -> Self {
        Self {
            max_eps_f32,
            max_eps_f64,
        }
    }

    pub fn exact() -> Self {
        Self {
            max_eps_f32: |_| 0.0,
            max_eps_f64: |_| 0.0,
        }
    }
}

#[cfg_attr(any(test, feature = "testing"), derive(proptest_derive::Arbitrary))]
#[derive(Default, Clone, PartialEq, Debug)]
pub struct FloatEncoderConfig {
    pub packing: PackingMode,
    #[cfg_attr(
        any(test, feature = "testing"),
        proptest(value = "MaxFloatEpsilon::exact()")
    )]
    pub max_epsilon: MaxFloatEpsilon,
}

impl FloatEncoderConfig {
    pub fn with_packing(mut self, packing: PackingMode) -> Self {
        self.packing = packing;
        self
    }

    pub fn with_max_epsilon(mut self, max_epsilon: MaxFloatEpsilon) -> Self {
        self.max_epsilon = max_epsilon;
        self
    }
}
