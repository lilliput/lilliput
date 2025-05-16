use super::{ToZigZag, TryFromInt, WithBeBytes, WithPackedBeBytes};

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

macro_rules! impl_with_packed_be_bytes_for_unsigned_int {
    ($t:ty) => {
        impl WithBeBytes for $t {
            #[inline]
            fn with_be_bytes<T, F>(&self, f: F) -> T
            where
                F: FnOnce(&[u8]) -> T,
            {
                let bytes = self.to_be_bytes();
                let width = bytes.len();
                debug_assert_eq!(width, bytes.len());

                f(&bytes)
            }
        }

        impl WithPackedBeBytes for $t {
            #[inline]
            fn with_native_packed_be_bytes<T, F>(&self, f: F) -> T
            where
                F: FnOnce(&[u8]) -> T,
            {
                let be_bytes = self.to_be_bytes();
                let width: u8 = {
                    let overflows_u8 = if u8::BITS < Self::BITS {
                        (*self > u8::MAX as Self) as u8
                    } else {
                        0
                    };

                    let overflows_u16 = if u16::BITS < Self::BITS {
                        (*self > u16::MAX as Self) as u8
                    } else {
                        0
                    };

                    let overflows_u32 = if u32::BITS < Self::BITS {
                        (*self > u32::MAX as Self) as u8
                    } else {
                        0
                    };

                    (overflows_u32 << 2) + (overflows_u16 << 1) + overflows_u8 + 1
                };

                let bytes: &[u8] = &be_bytes[(be_bytes.len() - (width as usize))..];

                f(bytes)
            }

            #[inline]
            fn with_optimal_packed_be_bytes<T, F>(&self, f: F) -> T
            where
                F: FnOnce(&[u8]) -> T,
            {
                let be_bytes = self.to_be_bytes();
                let width: u8 = {
                    let leading_zero_bytes = (self.leading_zeros() / u8::BITS) as u8;
                    let native_width = (Self::BITS / u8::BITS) as u8;
                    (native_width - leading_zero_bytes).max(1)
                };
                let bytes: &[u8] = &be_bytes[(be_bytes.len() - (width as usize))..];

                f(bytes)
            }
        }
    };
}

impl_with_packed_be_bytes_for_unsigned_int!(u8);
impl_with_packed_be_bytes_for_unsigned_int!(u16);
impl_with_packed_be_bytes_for_unsigned_int!(u32);
impl_with_packed_be_bytes_for_unsigned_int!(u64);
impl_with_packed_be_bytes_for_unsigned_int!(usize);

macro_rules! impl_with_packed_be_bytes_for_signed_int {
    ($t:ty) => {
        impl WithBeBytes for $t
        where
            $t: ToZigZag,
        {
            #[inline]
            fn with_be_bytes<T, F>(&self, f: F) -> T
            where
                F: FnOnce(&[u8]) -> T,
            {
                self.to_zig_zag().with_be_bytes(f)
            }
        }

        impl WithPackedBeBytes for $t
        where
            $t: ToZigZag,
        {
            #[inline]
            fn with_native_packed_be_bytes<T, F>(&self, f: F) -> T
            where
                F: FnOnce(&[u8]) -> T,
            {
                self.to_zig_zag().with_native_packed_be_bytes(f)
            }

            #[inline]
            fn with_optimal_packed_be_bytes<T, F>(&self, f: F) -> T
            where
                F: FnOnce(&[u8]) -> T,
            {
                self.to_zig_zag().with_optimal_packed_be_bytes(f)
            }
        }
    };
}

impl_with_packed_be_bytes_for_signed_int!(i8);
impl_with_packed_be_bytes_for_signed_int!(i16);
impl_with_packed_be_bytes_for_signed_int!(i32);
impl_with_packed_be_bytes_for_signed_int!(i64);
impl_with_packed_be_bytes_for_signed_int!(isize);
