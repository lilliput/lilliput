use crate::{config::PackingMode, sealed::Sealed};

pub mod float;
pub mod int;
pub mod zigzag;

use zigzag::ToZigZag;

pub trait PackedBeBytes: Sealed {
    fn native_width() -> u8;
    fn native_packed_width(&self) -> u8;
    fn optimal_packed_width(&self) -> u8;

    #[inline]
    fn packed_width(&self, packing_mode: PackingMode) -> u8 {
        match packing_mode {
            PackingMode::None => Self::native_width(),
            PackingMode::Native => self.native_packed_width(),
            PackingMode::Optimal => self.optimal_packed_width(),
        }
    }
}

pub trait WithPackedBeBytes: PackedBeBytes {
    fn with_be_bytes<T, F>(&self, f: F) -> T
    where
        F: FnOnce(u8, &[u8]) -> T;

    fn with_native_packed_be_bytes<T, F>(&self, f: F) -> T
    where
        F: FnOnce(u8, &[u8]) -> T;

    fn with_optimal_packed_be_bytes<T, F>(&self, f: F) -> T
    where
        F: FnOnce(u8, &[u8]) -> T;

    #[inline]
    fn with_packed_be_bytes<T, F>(&self, packing_mode: PackingMode, f: F) -> T
    where
        F: FnOnce(u8, &[u8]) -> T,
    {
        match packing_mode {
            PackingMode::None => self.with_be_bytes(f),
            PackingMode::Native => self.with_native_packed_be_bytes(f),
            PackingMode::Optimal => self.with_optimal_packed_be_bytes(f),
        }
    }
}

impl PackedBeBytes for f32 {
    fn native_width() -> u8 {
        4
    }

    fn native_packed_width(&self) -> u8 {
        Self::native_width()
    }

    fn optimal_packed_width(&self) -> u8 {
        // FIXME: add support for `f16` on nightly
        self.native_packed_width()
    }
}

impl WithPackedBeBytes for f32 {
    #[inline]
    fn with_be_bytes<T, F>(&self, f: F) -> T
    where
        F: FnOnce(u8, &[u8]) -> T,
    {
        let bytes = self.to_be_bytes();
        let width = bytes.len();
        debug_assert_eq!(width, bytes.len());
        debug_assert_eq!(width as u8, self.native_packed_width());

        f(width as u8, &bytes)
    }

    #[inline]
    fn with_native_packed_be_bytes<T, F>(&self, f: F) -> T
    where
        F: FnOnce(u8, &[u8]) -> T,
    {
        // FIXME: add support for `f16` on nightly
        self.with_be_bytes(f)
    }

    #[inline]
    fn with_optimal_packed_be_bytes<T, F>(&self, f: F) -> T
    where
        F: FnOnce(u8, &[u8]) -> T,
    {
        // FIXME: add support for optimized var-floats
        self.with_native_packed_be_bytes(f)
    }
}

impl PackedBeBytes for f64 {
    fn native_width() -> u8 {
        8
    }

    fn native_packed_width(&self) -> u8 {
        Self::native_width()
    }

    fn optimal_packed_width(&self) -> u8 {
        // FIXME: add support for `f16` on nightly
        self.native_packed_width()
    }
}

impl WithPackedBeBytes for f64 {
    #[inline]
    fn with_be_bytes<T, F>(&self, f: F) -> T
    where
        F: FnOnce(u8, &[u8]) -> T,
    {
        let bytes = self.to_be_bytes();
        let width = bytes.len();
        debug_assert_eq!(width, bytes.len());

        f(width as u8, &bytes)
    }

    #[inline]
    fn with_native_packed_be_bytes<T, F>(&self, f: F) -> T
    where
        F: FnOnce(u8, &[u8]) -> T,
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
        F: FnOnce(u8, &[u8]) -> T,
    {
        // FIXME: add support for optimized var-floats
        self.with_native_packed_be_bytes(f)
    }
}

macro_rules! impl_with_packed_be_bytes_for_unsigned_int {
    ($t:ty) => {
        impl PackedBeBytes for $t {
            fn native_width() -> u8 {
                (Self::BITS / u8::BITS) as u8
            }

            fn native_packed_width(&self) -> u8 {
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
            }

            fn optimal_packed_width(&self) -> u8 {
                let leading_zero_bytes = (self.leading_zeros() / u8::BITS) as u8;

                (Self::native_width() - leading_zero_bytes).max(1)
            }
        }

        impl WithPackedBeBytes for $t {
            #[inline]
            fn with_be_bytes<T, F>(&self, f: F) -> T
            where
                F: FnOnce(u8, &[u8]) -> T,
            {
                let bytes = self.to_be_bytes();
                let width = bytes.len();
                debug_assert_eq!(width, bytes.len());

                f(width as u8, &bytes)
            }

            #[inline]
            fn with_native_packed_be_bytes<T, F>(&self, f: F) -> T
            where
                F: FnOnce(u8, &[u8]) -> T,
            {
                let be_bytes = self.to_be_bytes();
                let width: u8 = self.native_packed_width();
                let bytes: &[u8] = &be_bytes[(be_bytes.len() - (width as usize))..];

                f(width, bytes)
            }

            #[inline]
            fn with_optimal_packed_be_bytes<T, F>(&self, f: F) -> T
            where
                F: FnOnce(u8, &[u8]) -> T,
            {
                let be_bytes = self.to_be_bytes();
                let width = self.optimal_packed_width() as usize;
                let bytes: &[u8] = &be_bytes[(be_bytes.len() - (width as usize))..];

                f(width as u8, bytes)
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
        impl PackedBeBytes for $t
        where
            $t: ToZigZag,
        {
            fn native_width() -> u8 {
                <Self as ToZigZag>::ZigZag::native_width()
            }

            fn native_packed_width(&self) -> u8 {
                self.to_zig_zag().native_packed_width()
            }

            fn optimal_packed_width(&self) -> u8 {
                self.to_zig_zag().optimal_packed_width()
            }
        }

        impl WithPackedBeBytes for $t
        where
            $t: ToZigZag,
        {
            #[inline]
            fn with_be_bytes<T, F>(&self, f: F) -> T
            where
                F: FnOnce(u8, &[u8]) -> T,
            {
                self.to_zig_zag().with_be_bytes(f)
            }

            #[inline]
            fn with_native_packed_be_bytes<T, F>(&self, f: F) -> T
            where
                F: FnOnce(u8, &[u8]) -> T,
            {
                self.to_zig_zag().with_native_packed_be_bytes(f)
            }

            #[inline]
            fn with_optimal_packed_be_bytes<T, F>(&self, f: F) -> T
            where
                F: FnOnce(u8, &[u8]) -> T,
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
