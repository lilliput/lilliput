use lilliput_float::{FpToBeBytes as _, FpTruncate, F16, F24, F32, F40, F48, F56, F64, F8};

use super::{WithBeBytes, WithPackedBeBytesIf};

impl WithBeBytes for f32 {
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

macro_rules! truncate {
    ($src:ty => $dst:ty, $native:expr, $predicate:expr) => {{
        let (native, predicate) = ($native, $predicate);

        let non_packed: $src = native.into();

        FpTruncate::<$dst>::try_truncate(non_packed)
            .ok()
            .and_then(|(truncated, packed)| {
                if (predicate)(non_packed, truncated) {
                    Some(packed)
                } else {
                    None
                }
            })
    }};
}

impl WithPackedBeBytesIf for f32 {
    #[inline]
    fn with_native_packed_be_bytes_if<T, P, F>(&self, predicate: P, f: F) -> T
    where
        P: Fn(&Self, &Self) -> bool,
        F: FnOnce(&[u8]) -> T,
    {
        let non_packed: f32 = *self;

        #[allow(unused_variables)]
        let predicate = |value: F32, packed: F32| {
            let value: f32 = value.into();
            let packed: f32 = packed.into();
            predicate(&value, &packed)
        };

        #[cfg(feature = "native-f16")]
        if let Some(packed) = truncate!(F32 => F16, non_packed, predicate) {
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
        let non_packed: f32 = *self;

        let predicate = |value: F32, packed: F32| {
            let value: f32 = value.into();
            let packed: f32 = packed.into();
            predicate(&value, &packed)
        };

        if let Some(packed) = truncate!(F32 => F16, non_packed, predicate) {
            if let Some(packed) = truncate!(F32 => F8, non_packed, predicate) {
                f(&packed.to_be_bytes())
            } else {
                f(&packed.to_be_bytes())
            }
        } else if let Some(packed) = truncate!(F32 => F24, non_packed, predicate) {
            f(&packed.to_be_bytes())
        } else {
            f(&non_packed.to_be_bytes())
        }
    }
}

impl WithBeBytes for f64 {
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

impl WithPackedBeBytesIf for f64 {
    #[inline]
    fn with_native_packed_be_bytes_if<T, P, F>(&self, predicate: P, f: F) -> T
    where
        P: Fn(&Self, &Self) -> bool,
        F: FnOnce(&[u8]) -> T,
    {
        let non_packed: f64 = *self;

        let predicate = |value: F64, packed: F64| {
            let value: f64 = value.into();
            let packed: f64 = packed.into();
            predicate(&value, &packed)
        };

        if let Some(packed) = truncate!(F64 => F32, non_packed, predicate) {
            #[cfg(feature = "native-f16")]
            if let Some(packed) = truncate!(F64 => F16, non_packed, predicate) {
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
        let non_packed: f64 = *self;

        let predicate = |value: F64, packed: F64| {
            let value: f64 = value.into();
            let packed: f64 = packed.into();
            predicate(&value, &packed)
        };

        if let Some(packed) = truncate!(F64 => F32, non_packed, predicate) {
            if let Some(packed) = truncate!(F64 => F16, non_packed, predicate) {
                if let Some(packed) = truncate!(F64 => F8, non_packed, predicate) {
                    f(&packed.to_be_bytes())
                } else {
                    f(&packed.to_be_bytes())
                }
            } else if let Some(packed) = truncate!(F64 => F24, non_packed, predicate) {
                f(&packed.to_be_bytes())
            } else {
                f(&packed.to_be_bytes())
            }
        } else if let Some(packed) = truncate!(F64 => F48, non_packed, predicate) {
            if let Some(packed) = truncate!(F64 => F40, non_packed, predicate) {
                f(&packed.to_be_bytes())
            } else {
                f(&packed.to_be_bytes())
            }
        } else if let Some(packed) = truncate!(F64 => F56, non_packed, predicate) {
            f(&packed.to_be_bytes())
        } else {
            f(&non_packed.to_be_bytes())
        }
    }
}
