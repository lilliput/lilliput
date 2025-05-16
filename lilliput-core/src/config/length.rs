use super::PackingMode;

#[cfg_attr(any(test, feature = "testing"), derive(proptest_derive::Arbitrary))]
#[derive(Default, Clone, PartialEq, Debug)]
pub struct LengthEncoderConfig {
    pub packing: PackingMode,
}

impl LengthEncoderConfig {
    pub fn with_packing(mut self, packing: PackingMode) -> Self {
        self.packing = packing;
        self
    }
}
