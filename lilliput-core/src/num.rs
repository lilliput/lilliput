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

pub trait WithPackedBeBytesIf {
    fn with_be_bytes<T, F>(&self, f: F) -> T
    where
        F: FnOnce(&[u8]) -> T;

    fn with_native_packed_be_bytes_if<T, P, F>(&self, predicate: P, f: F) -> T
    where
        P: Fn(&Self, &Self) -> bool,
        F: FnOnce(&[u8]) -> T;

    fn with_optimal_packed_be_bytes_if<T, P, F>(&self, predicate: P, f: F) -> T
    where
        P: Fn(&Self, &Self) -> bool,
        F: FnOnce(&[u8]) -> T;

    #[inline]
    fn with_packed_be_bytes_if<T, P, F>(&self, packing_mode: PackingMode, predicate: P, f: F) -> T
    where
        P: Fn(&Self, &Self) -> bool,
        F: FnOnce(&[u8]) -> T,
    {
        match packing_mode {
            PackingMode::None => self.with_be_bytes(f),
            PackingMode::Native => self.with_native_packed_be_bytes_if(predicate, f),
            PackingMode::Optimal => self.with_optimal_packed_be_bytes_if(predicate, f),
        }
    }
}

impl WithPackedBeBytesIf for f32 {
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
    fn with_native_packed_be_bytes_if<T, P, F>(&self, predicate: P, f: F) -> T
    where
        P: Fn(&Self, &Self) -> bool,
        F: FnOnce(&[u8]) -> T,
    {
        let non_packed = F32::from(*self);

        #[allow(unused_variables)]
        let predicate = |value: F32, packed: F32| {
            let value: f32 = value.into();
            let packed: f32 = packed.into();
            predicate(&value, &packed)
        };

        #[cfg(feature = "native-f16")]
        if let Some(packed) = FpTruncate::<F16>::truncate_if(non_packed, predicate) {
            f(&packed.to_be_bytes())
        } else {
            f(&non_packed.to_be_bytes())
        }

        #[cfg(not(feature = "native-f16"))]
        f(&non_packed.to_be_bytes())
    }

    #[inline]
    fn with_optimal_packed_be_bytes_if<T, P, F>(&self, predicate: P, f: F) -> T
    where
        P: Fn(&Self, &Self) -> bool,
        F: FnOnce(&[u8]) -> T,
    {
        let non_packed = F32::from(*self);

        let predicate = |value: F32, packed: F32| {
            let value: f32 = value.into();
            let packed: f32 = packed.into();
            predicate(&value, &packed)
        };

        if let Some(packed) = FpTruncate::<F16>::truncate_if(non_packed, predicate) {
            if let Some(packed) = FpTruncate::<F8>::truncate_if(non_packed, predicate) {
                f(&packed.to_be_bytes())
            } else {
                f(&packed.to_be_bytes())
            }
        } else if let Some(packed) = FpTruncate::<F24>::truncate_if(non_packed, predicate) {
            f(&packed.to_be_bytes())
        } else {
            f(&non_packed.to_be_bytes())
        }
    }
}

impl WithPackedBeBytesIf for f64 {
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
    fn with_native_packed_be_bytes_if<T, P, F>(&self, predicate: P, f: F) -> T
    where
        P: Fn(&Self, &Self) -> bool,
        F: FnOnce(&[u8]) -> T,
    {
        let non_packed = F64::from(*self);

        let predicate = |value: F64, packed: F64| {
            let value: f64 = value.into();
            let packed: f64 = packed.into();
            predicate(&value, &packed)
        };

        if let Some(packed) = FpTruncate::<F32>::truncate_if(non_packed, predicate) {
            #[cfg(feature = "native-f16")]
            if let Some(packed) = FpTruncate::<F16>::truncate_if(non_packed, predicate) {
                f(&packed.to_be_bytes())
            } else {
                f(&packed.to_be_bytes())
            }

            #[cfg(not(feature = "native-f16"))]
            f(&packed.to_be_bytes())
        } else {
            f(&non_packed.to_be_bytes())
        }
    }

    #[inline]
    fn with_optimal_packed_be_bytes_if<T, P, F>(&self, predicate: P, f: F) -> T
    where
        P: Fn(&Self, &Self) -> bool,
        F: FnOnce(&[u8]) -> T,
    {
        let non_packed = F64::from(*self);

        let predicate = |value: F64, packed: F64| {
            let value: f64 = value.into();
            let packed: f64 = packed.into();
            predicate(&value, &packed)
        };

        if let Some(packed) = FpTruncate::<F32>::truncate_if(non_packed, predicate) {
            if let Some(packed) = FpTruncate::<F16>::truncate_if(non_packed, predicate) {
                if let Some(packed) = FpTruncate::<F8>::truncate_if(non_packed, predicate) {
                    f(&packed.to_be_bytes())
                } else {
                    f(&packed.to_be_bytes())
                }
            } else if let Some(packed) = FpTruncate::<F24>::truncate_if(non_packed, predicate) {
                f(&packed.to_be_bytes())
            } else {
                f(&packed.to_be_bytes())
            }
        } else if let Some(packed) = FpTruncate::<F48>::truncate_if(non_packed, predicate) {
            if let Some(packed) = FpTruncate::<F40>::truncate_if(non_packed, predicate) {
                f(&packed.to_be_bytes())
            } else {
                f(&packed.to_be_bytes())
            }
        } else if let Some(packed) = FpTruncate::<F56>::truncate_if(non_packed, predicate) {
            f(&packed.to_be_bytes())
        } else {
            f(&non_packed.to_be_bytes())
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
