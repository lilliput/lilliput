use crate::value::{IntValue, SignedIntValue, UnsignedIntValue};

pub trait Sealed {}

macro_rules! impl_sealed {
    ($($t:ty),+ $(,)?) => {
        $(
            impl Sealed for $t {}
        )+
    }
}

impl_sealed!(f32, f64);
impl_sealed!(i8, i16, i32, i64, isize);
impl_sealed!(u8, u16, u32, u64, usize);
impl_sealed!(SignedIntValue, UnsignedIntValue, IntValue);
