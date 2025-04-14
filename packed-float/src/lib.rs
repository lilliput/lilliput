mod floats;

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
