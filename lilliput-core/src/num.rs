use packed_float::{FpToBeBytes, FpTruncate, F16, F24, F32, F40, F48, F56, F64, F8};

use crate::config::PackingMode;

pub mod int;
pub mod zigzag;

use zigzag::ToZigZag;

pub trait WithPackedBeBytes {
    fn with_be_bytes<T, F>(&self, f: F) -> T
    where
        F: FnOnce(&[u8]) -> T;

    fn with_native_packed_be_bytes<T, F>(&self, f: F) -> T
    where
        F: FnOnce(&[u8]) -> T;

    fn with_optimal_packed_be_bytes<T, F>(&self, f: F) -> T
    where
        F: FnOnce(&[u8]) -> T;

    #[inline]
    fn with_packed_be_bytes<T, F>(&self, packing_mode: PackingMode, f: F) -> T
    where
        F: FnOnce(&[u8]) -> T,
    {
        match packing_mode {
            PackingMode::None => self.with_be_bytes(f),
            PackingMode::Native => self.with_native_packed_be_bytes(f),
            PackingMode::Optimal => self.with_optimal_packed_be_bytes(f),
        }
    }
}

impl WithPackedBeBytes for f32 {
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

    #[inline]
    fn with_native_packed_be_bytes<T, F>(&self, f: F) -> T
    where
        F: FnOnce(&[u8]) -> T,
    {
        // FIXME: add support for `f16` on nightly
        self.with_be_bytes(f)
    }

    #[inline]
    fn with_optimal_packed_be_bytes<T, F>(&self, f: F) -> T
    where
        F: FnOnce(&[u8]) -> T,
    {
        let native = F32::from(*self);

        if let Ok(optimal) = FpTruncate::<F8>::try_truncate(native) {
            f(&optimal.to_be_bytes())
        } else if let Ok(optimal) = FpTruncate::<F16>::try_truncate(native) {
            f(&optimal.to_be_bytes())
        } else if let Ok(optimal) = FpTruncate::<F24>::try_truncate(native) {
            f(&optimal.to_be_bytes())
        } else {
            f(&native.to_be_bytes())
        }
    }
}

impl WithPackedBeBytes for f64 {
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

    #[inline]
    fn with_native_packed_be_bytes<T, F>(&self, f: F) -> T
    where
        F: FnOnce(&[u8]) -> T,
    {
        // FIXME: add support for `f16` on nightly
        let as_f32 = *self as f32;

        if as_f32 as f64 == *self {
            as_f32.with_native_packed_be_bytes(f)
        } else {
            self.with_be_bytes(f)
        }
    }

    #[inline]
    fn with_optimal_packed_be_bytes<T, F>(&self, f: F) -> T
    where
        F: FnOnce(&[u8]) -> T,
    {
        let native = F64::from(*self);

        if let Ok(optimal) = FpTruncate::<F8>::try_truncate(native) {
            f(&optimal.to_be_bytes())
        } else if let Ok(optimal) = FpTruncate::<F16>::try_truncate(native) {
            f(&optimal.to_be_bytes())
        } else if let Ok(optimal) = FpTruncate::<F24>::try_truncate(native) {
            f(&optimal.to_be_bytes())
        } else if let Ok(optimal) = FpTruncate::<F32>::try_truncate(native) {
            f(&optimal.to_be_bytes())
        } else if let Ok(optimal) = FpTruncate::<F40>::try_truncate(native) {
            f(&optimal.to_be_bytes())
        } else if let Ok(optimal) = FpTruncate::<F48>::try_truncate(native) {
            f(&optimal.to_be_bytes())
        } else if let Ok(optimal) = FpTruncate::<F56>::try_truncate(native) {
            f(&optimal.to_be_bytes())
        } else {
            f(&native.to_be_bytes())
        }
    }
}

macro_rules! impl_with_packed_be_bytes_for_unsigned_int {
    ($t:ty) => {
        impl WithPackedBeBytes for $t {
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
        impl WithPackedBeBytes for $t
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
