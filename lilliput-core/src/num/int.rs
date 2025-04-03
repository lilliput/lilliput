use num_traits::PrimInt;

pub(crate) trait TryFromInt<T>: PrimInt
where
    T: PrimInt,
{
    fn try_from_int(int: T) -> Result<Self, core::num::TryFromIntError>;
}

pub(crate) trait TryIntoInt<T>: PrimInt
where
    T: PrimInt,
{
    fn try_into_int(self) -> Result<T, core::num::TryFromIntError>;
}

impl<T, U> TryIntoInt<U> for T
where
    T: PrimInt,
    U: PrimInt + TryFromInt<T>,
{
    #[inline]
    fn try_into_int(self) -> Result<U, core::num::TryFromIntError> {
        U::try_from_int(self)
    }
}

macro_rules! impl_try_from_int {
    ($t:ty, infallible: [$($i:ty),* $(,)?], fallible: [$($f:ty),* $(,)?]) => {
        impl_try_from_int!($t, infallible: [$($i),*]);
        impl_try_from_int!($t, fallible: [$($f),*]);
    };
    ($t:ty, infallible: [$($i:ty),* $(,)?]) => {
        $(
            impl TryFromInt<$i> for $t
            where
                $t: TryFrom<$i, Error = core::convert::Infallible>
            {
                #[inline]
                fn try_from_int(int: $i) -> Result<$t, core::num::TryFromIntError> {
                    Ok(int.try_into().unwrap())
                }
            }
        )*
    };
    ($t:ty, fallible: [$($f:ty),* $(,)?]) => {
        $(
            impl TryFromInt<$f> for $t {
                #[inline]
                fn try_from_int(int: $f) -> Result<Self, core::num::TryFromIntError> {
                    int.try_into()
                }
            }
        )*
    };
}

impl_try_from_int!(i8, infallible: [i8], fallible: [i16, i32, i64, u8, u16, u32, u64]);
impl_try_from_int!(i16, infallible: [i8, i16, u8], fallible: [i32, i64, u16, u32, u64]);
impl_try_from_int!(i32, infallible: [i8, i16, i32, u8, u16], fallible: [i64, u32, u64]);
impl_try_from_int!(i64, infallible: [i8, i16, i32, i64, u8, u16, u32], fallible: [u64]);
impl_try_from_int!(isize, infallible: [i8, i16, u8], fallible: [i32, i64, u16, u32, u64]);
impl_try_from_int!(u8, infallible: [u8], fallible: [u16, u32, u64, i8, i16, i32, i64]);
impl_try_from_int!(u16, infallible: [u8, u16], fallible: [u32, u64, i8, i16, i32, i64]);
impl_try_from_int!(u32, infallible: [u8, u16, u32], fallible: [u64, i8, i16, i32, i64]);
impl_try_from_int!(u64, infallible: [u8, u16, u32, u64], fallible: [i8, i16, i32, i64]);
impl_try_from_int!(usize, infallible: [u8, u16, usize], fallible: [u32, u64, i8, i16, i32, i64]);

// use num_traits::{ToBytes, Unsigned};

// use crate::config::PackingMode;

// use super::PackedWidth;

// macro_rules! impl_compact_int_width {
//     ($t:ty) => {
//         impl $crate::num::PackedWidth for $t {
//             #[inline]
//             fn native_width() -> u8 {
//                 (Self::BITS / u8::BITS) as u8
//             }

//             #[inline]
//             fn optimal_packed_width(self) -> u8 {
//                 let bytes = (Self::BITS / u8::BITS) as u8;
//                 let leading_zero_bytes = (self.leading_zeros() / u8::BITS) as u8;
//                 (bytes - leading_zero_bytes).max(1)
//             }

//             #[inline]
//             fn native_packed_width(self) -> u8 {
//                 let overflows_u8 = (self > u8::MAX as Self) as u8;
//                 let overflows_u16 = (self > u16::MAX as Self) as u8;
//                 let overflows_u32 = (self > u32::MAX as Self) as u8;

//                 (overflows_u32 << 2) + (overflows_u16 << 1) + overflows_u8 + 1
//             }

//             #[inline]
//             fn native_packed_width_exponent(self) -> u8 {
//                 let overflows_u8 = (self > u8::MAX as Self) as u8;
//                 let overflows_u16 = (self > u16::MAX as Self) as u8;
//                 let overflows_u32 = (self > u32::MAX as Self) as u8;

//                 overflows_u32 + overflows_u16 + overflows_u8
//             }
//         }
//     };
// }

// impl_compact_int_width!(u8);
// impl_compact_int_width!(u16);
// impl_compact_int_width!(u32);
// impl_compact_int_width!(u64);
// impl_compact_int_width!(usize);

// #[inline]
// pub(crate) fn with_packed_width_be_bytes<T, U, F, const N: usize>(
//     value: T,
//     packing_mode: PackingMode,
//     f: F,
// ) -> crate::error::Result<U>
// where
//     T: PackedWidth + Unsigned + ToBytes<Bytes = [u8; N]>,
//     F: FnOnce(bool, u8, &[u8]) -> crate::error::Result<U>,
// {
//     let be_bytes: [u8; N] = value.to_be_bytes();

//     let mut is_compact: bool = false;

//     let width = match packing_mode {
//         PackingMode::None => T::native_width(),
//         PackingMode::Native => value.native_packed_width(),
//         PackingMode::Optimal => value.optimal_packed_width(),
//     };

//     let bytes = &be_bytes[(N - width)..];

//     f(width, bytes)
// }

// #[inline]
// pub(crate) fn with_n_be_bytes<T, U, F, const N: usize>(
//     value: T,
//     n: usize,
//     f: F,
// ) -> crate::error::Result<U>
// where
//     T: Unsigned + ToBytes<Bytes = [u8; N]>,
//     F: FnOnce(&[u8]) -> crate::error::Result<U>,
// {
//     debug_assert!(n >= 1);
//     debug_assert!(n <= 8);

//     let be_bytes: [u8; N] = value.to_be_bytes();
//     let bytes = &be_bytes[(N - n)..];

//     f(bytes)
// }
