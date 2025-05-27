//! Configuration used for encoding integer values.

use super::PackingMode;

/// Configuration used for encoding integer values.
#[cfg_attr(any(test, feature = "testing"), derive(proptest_derive::Arbitrary))]
#[derive(Default, Clone, PartialEq, Debug)]
pub struct IntEncoderConfig {
    /// Packing mode for encoding.
    pub packing: PackingMode,
}

impl IntEncoderConfig {
    /// Sets packing-modes to `packing`, returning `self`.
    pub fn with_packing(mut self, packing: PackingMode) -> Self {
        self.packing = packing;
        self
    }
}
