mod bits;
mod floats;

pub use self::bits::{FpFromBits, FpToBits};
pub use self::floats::{F16, F24, F32, F40, F48, F56, F64, F8};

/// A packed representation of floating-point numbers.
#[derive(Copy, Clone, Debug)]
pub enum PackedFloat {
    F8(F8),
    F16(F16),
    F24(F24),
    F32(F32),
    F40(F40),
    F48(F48),
    F56(F56),
    F64(F64),
}

mod sealed {
    pub trait Sealed {}
}

pub(crate) use self::sealed::Sealed;

impl Sealed for F8 {}
impl Sealed for F16 {}
impl Sealed for F24 {}
impl Sealed for F32 {}
impl Sealed for F40 {}
impl Sealed for F48 {}
impl Sealed for F56 {}
impl Sealed for F64 {}

impl Sealed for PackedFloat {}
