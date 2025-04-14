mod be_bytes;
mod bits;
mod classify;
mod cmp;
mod floats;
mod native;
mod repr;

pub use self::be_bytes::{FpFromBeBytes, FpToBeBytes};
pub use self::bits::{FpFromBits, FpToBits};
pub use self::classify::FpClassify;
pub use self::floats::{F16, F24, F32, F40, F48, F56, F64, F8};
pub use self::repr::FpRepr;

/// A packed representation of floating-point numbers.
#[derive(Copy, Clone, PartialEq, Debug)]
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

impl PartialOrd for PackedFloat {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Self::F8(lhs), Self::F8(rhs)) => lhs.partial_cmp(rhs),
            (Self::F16(lhs), Self::F16(rhs)) => lhs.partial_cmp(rhs),
            (Self::F24(lhs), Self::F24(rhs)) => lhs.partial_cmp(rhs),
            (Self::F32(lhs), Self::F32(rhs)) => lhs.partial_cmp(rhs),
            (Self::F40(lhs), Self::F40(rhs)) => lhs.partial_cmp(rhs),
            (Self::F48(lhs), Self::F48(rhs)) => lhs.partial_cmp(rhs),
            (Self::F56(lhs), Self::F56(rhs)) => lhs.partial_cmp(rhs),
            (Self::F64(lhs), Self::F64(rhs)) => lhs.partial_cmp(rhs),
            _ => None,
        }
    }
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
