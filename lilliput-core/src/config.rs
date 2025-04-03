#[cfg_attr(any(test, feature = "testing"), derive(proptest_derive::Arbitrary))]
#[derive(Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[repr(u8)]
pub enum PackingMode {
    None = 0,
    Native = 1,
    #[default]
    Optimal = 2,
}

#[cfg_attr(any(test, feature = "testing"), derive(proptest_derive::Arbitrary))]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct EncodingConfig {
    pub len_packing: PackingMode,
    pub int_packing: PackingMode,
    pub float_packing: PackingMode,
}

impl EncodingConfig {
    pub fn no_packing() -> Self {
        Self {
            len_packing: PackingMode::None,
            int_packing: PackingMode::None,
            float_packing: PackingMode::None,
        }
    }

    pub fn native_packing() -> Self {
        Self {
            len_packing: PackingMode::Native,
            int_packing: PackingMode::Native,
            float_packing: PackingMode::Native,
        }
    }

    pub fn optimal_packing() -> Self {
        Self {
            len_packing: PackingMode::Optimal,
            int_packing: PackingMode::Optimal,
            float_packing: PackingMode::Optimal,
        }
    }
}

impl Default for EncodingConfig {
    fn default() -> Self {
        Self {
            len_packing: PackingMode::Optimal,
            int_packing: PackingMode::Optimal,
            float_packing: PackingMode::Native,
        }
    }
}

#[cfg_attr(any(test, feature = "testing"), derive(proptest_derive::Arbitrary))]
#[derive(Default, Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct DecodingConfig {}
