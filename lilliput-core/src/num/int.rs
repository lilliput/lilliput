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
