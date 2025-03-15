use num_traits::{float::FloatCore, PrimInt, Signed, Unsigned};

pub(crate) trait ToZigZag: Signed {
    type ZigZag: Unsigned;

    fn to_zig_zag(self) -> Self::ZigZag;
}

pub(crate) trait FromZigZag {
    type ZigZag;

    fn from_zig_zag(zig_zag: Self::ZigZag) -> Self;
}

macro_rules! impl_zig_zag {
    (signed: $s:ty, unsigned: $u:ty) => {
        impl ToZigZag for $s {
            type ZigZag = $u;

            fn to_zig_zag(self) -> Self::ZigZag {
                (self >> (Self::BITS as usize) - 1) as Self::ZigZag ^ (self << 1) as Self::ZigZag
            }
        }

        impl FromZigZag for $s {
            type ZigZag = $u;

            fn from_zig_zag(zig_zag: Self::ZigZag) -> Self {
                (zig_zag >> 1) as Self ^ -((zig_zag & 1) as Self)
            }
        }
    };
}

impl_zig_zag!(signed: i8, unsigned: u8);
impl_zig_zag!(signed: i16, unsigned: u16);
impl_zig_zag!(signed: i32, unsigned: u32);
impl_zig_zag!(signed: i64, unsigned: u64);

// The `core`/`std` libraries do not provide implementations
// of `From<T>`/`Into<U>` for casting between `f32` and f64`,
// even though doing so is safe according to the Rust reference:
//
// > - Casting from an f32 to an f64 is perfect and lossless.
// > - Casting from an f64 to an f32 will produce the closest possible f32
// >   - if necessary, rounding is according to roundTiesToEven mode
// >   - on overflow, infinity (of the same sign as the input) is produced
//
// https://doc.rust-lang.org/reference/expressions/operator-expr.html#semantics
//
// The `FromFloat<T>`/`IntoFloat<T>` traits aim to bridge this gap.

pub(crate) trait FromFloat<T>: FloatCore
where
    T: FloatCore,
{
    fn from_float(float: T) -> Self;
}

impl FromFloat<f32> for f64 {
    fn from_float(float: f32) -> Self {
        float as Self
    }
}

impl FromFloat<f64> for f32 {
    fn from_float(float: f64) -> Self {
        float as Self
    }
}

impl<T> FromFloat<T> for T
where
    T: FloatCore,
{
    fn from_float(float: Self) -> Self {
        float
    }
}

pub(crate) trait IntoFloat<T>: FloatCore
where
    T: FloatCore,
{
    // Required method
    fn into_float(self) -> T;
}

impl<T, U> IntoFloat<U> for T
where
    T: FloatCore,
    U: FloatCore + FromFloat<T>,
{
    fn into_float(self) -> U {
        U::from_float(self)
    }
}

// Integer types implement `TryFrom<Self, Error == Infallible>`,
// which is rather annoying, when trying to unify conversion
// over `core::num::TryFromIntError` as common error type.
// The `TryFromInt<T>` and `TryIntoInt<T>` traits help with this.

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
    #[inline(always)]
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
                #[inline(always)]
                fn try_from_int(int: $i) -> Result<$t, core::num::TryFromIntError> {
                    Ok(int.try_into().unwrap())
                }
            }
        )*
    };
    ($t:ty, fallible: [$($f:ty),* $(,)?]) => {
        $(
            impl TryFromInt<$f> for $t {
                #[inline(always)]
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

impl_try_from_int!(u8, infallible: [u8], fallible: [u16, u32, u64, i8, i16, i32, i64]);
impl_try_from_int!(u16, infallible: [u8, u16], fallible: [u32, u64, i8, i16, i32, i64]);
impl_try_from_int!(u32, infallible: [u8, u16, u32], fallible: [u64, i8, i16, i32, i64]);
impl_try_from_int!(u64, infallible: [u8, u16, u32, u64], fallible: [i8, i16, i32, i64]);

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use super::*;

    proptest! {
        #[test]
        fn zig_zag_roundtrip(before in i8::MIN..=i8::MAX) {
            let zig_zag = before.to_zig_zag();
            let after = i8::from_zig_zag(zig_zag);

            prop_assert_eq!(&before, &after);
        }
    }
}
