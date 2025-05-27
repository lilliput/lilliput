//! Configuration used for encoding value lengths (in header extensions).

use super::PackingMode;

/// Configuration used for encoding value lengths (in header extensions).
#[cfg_attr(any(test, feature = "testing"), derive(proptest_derive::Arbitrary))]
#[derive(Default, Clone, PartialEq, Debug)]
pub struct LengthEncoderConfig {
    /// Packing mode for encoding.
    pub packing: PackingMode,
}

impl LengthEncoderConfig {
    /// Sets packing-modes to `packing`, returning `self`.
    pub fn with_packing(mut self, packing: PackingMode) -> Self {
        self.packing = packing;
        self
    }
}
