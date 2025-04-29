mod be_bytes;
mod bits;
mod classify;
mod cmp;
mod extend;
mod floats;
mod native;
mod pack;
mod packed;
mod repr;
mod truncate;
mod validator;

pub use self::be_bytes::{FpFromBeBytes, FpToBeBytes};
pub use self::bits::{FpFromBits, FpToBits};
pub use self::classify::FpClassify;
pub use self::extend::FpExtend;
pub use self::floats::{F16, F24, F32, F40, F48, F56, F64, F8};
pub use self::pack::FpPack;
pub use self::packed::PackedFloat;
pub use self::repr::FpRepr;
pub use self::truncate::FpTruncate;
pub use self::validator::PackedFloatValidator;

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
