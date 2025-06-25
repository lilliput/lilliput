//! Configurations for encoding/decoding.

pub use float::FloatEncoderConfig;
pub use int::IntEncoderConfig;
pub use length::LengthEncoderConfig;

mod float;
mod int;
mod length;

/// Mode used while packing values.
#[cfg_attr(any(test, feature = "testing"), derive(proptest_derive::Arbitrary))]
#[derive(Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[repr(u8)]
pub enum PackingMode {
    /// No packing.
    None = 0,
    /// Packing down to native representations.
    Native = 1,
    /// Packing down to most optimal representations.
    #[default]
    Optimal = 2,
}

impl PackingMode {
    pub(crate) fn is_optimal(self) -> bool {
        self == Self::Optimal
    }
}

/// Configuration used for encoding values.
#[cfg_attr(any(test, feature = "testing"), derive(proptest_derive::Arbitrary))]
#[derive(Default, Clone, Debug)]
pub struct EncoderConfig {
    /// Configuration used for encoding value lengths (in header extensions).
    pub lengths: LengthEncoderConfig,
    /// Configuration used for encoding integer values.
    pub ints: IntEncoderConfig,
    /// Configuration used for encoding floating-point values.
    pub floats: FloatEncoderConfig,
}

impl EncoderConfig {
    /// Sets packing-modes to `packing`, returning `self`.
    pub fn with_packing(mut self, packing: PackingMode) -> Self {
        self.lengths = self.lengths.with_packing(packing);
        self.ints = self.ints.with_packing(packing);
        self.floats = self.floats.with_packing(packing);
        self
    }
}

/// Configuration used for decoding values.
#[cfg_attr(any(test, feature = "testing"), derive(proptest_derive::Arbitrary))]
#[derive(Default, Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct DecoderConfig {}
