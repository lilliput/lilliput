use float::FloatEncoderConfig;
use int::IntEncoderConfig;
use length::LengthEncoderConfig;

mod float;
mod int;
mod length;

#[cfg_attr(any(test, feature = "testing"), derive(proptest_derive::Arbitrary))]
#[derive(Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[repr(u8)]
pub enum PackingMode {
    None = 0,
    Native = 1,
    #[default]
    Optimal = 2,
}

impl PackingMode {
    pub(crate) fn is_optimal(self) -> bool {
        self == Self::Optimal
    }
}
#[cfg_attr(any(test, feature = "testing"), derive(proptest_derive::Arbitrary))]
#[derive(Default, Clone, PartialEq, Debug)]
pub struct EncoderConfig {
    pub lengths: LengthEncoderConfig,
    pub ints: IntEncoderConfig,
    pub floats: FloatEncoderConfig,
}

impl EncoderConfig {
    pub fn with_packing(mut self, packing: PackingMode) -> Self {
        self.lengths = self.lengths.with_packing(packing);
        self.ints = self.ints.with_packing(packing);
        self.floats = self.floats.with_packing(packing);
        self
    }
}

#[cfg_attr(any(test, feature = "testing"), derive(proptest_derive::Arbitrary))]
#[derive(Default, Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct DecoderConfig {}
