use crate::floats::{F16, F24, F32, F40, F48, F56, F64, F8};

pub trait FpFromBeBytes {
    type Bytes;

    fn from_be_bytes(be_bytes: Self::Bytes) -> Self;
}

macro_rules! impl_float_from_be_bytes {
    ($t:ty => bytes: [u8; $bytes:expr], bits: $bits:ty) => {
        impl FpFromBeBytes for $t {
            type Bytes = [u8; $bytes];

            fn from_be_bytes(be_bytes: Self::Bytes) -> Self {
                const PADDED_BYTES: usize = (<$bits>::BITS / u8::BITS) as usize;
                const PADDING: usize = (PADDED_BYTES - $bytes) as usize;
                let zeroed_bits: $bits = 0b0;
                let mut padded_be_bytes: [u8; PADDED_BYTES] = zeroed_bits.to_be_bytes();
                padded_be_bytes[PADDING..].copy_from_slice(&be_bytes);
                Self(<$bits>::from_be_bytes(padded_be_bytes))
            }
        }
    };
}

impl_float_from_be_bytes!(F8 => bytes: [u8; 1], bits: u8);
impl_float_from_be_bytes!(F16 => bytes: [u8; 2], bits: u16);
impl_float_from_be_bytes!(F24 => bytes: [u8; 3], bits: u32);
impl_float_from_be_bytes!(F32 => bytes: [u8; 4], bits: u32);
impl_float_from_be_bytes!(F40 => bytes: [u8; 5], bits: u64);
impl_float_from_be_bytes!(F48 => bytes: [u8; 6], bits: u64);
impl_float_from_be_bytes!(F56 => bytes: [u8; 7], bits: u64);
impl_float_from_be_bytes!(F64 => bytes: [u8; 8], bits: u64);

pub trait FpToBeBytes {
    type Bytes;

    fn to_be_bytes(self) -> Self::Bytes;
}

macro_rules! impl_float_to_be_bytes {
    ($t:ty => bytes: [u8; $bytes:expr], bits: $bits:ty) => {
        impl FpToBeBytes for $t {
            type Bytes = [u8; $bytes];

            fn to_be_bytes(self) -> Self::Bytes {
                const PADDED_BYTES: usize = (<$bits>::BITS / u8::BITS) as usize;
                const PADDING: usize = (PADDED_BYTES - $bytes) as usize;
                let padded_be_bytes: [u8; PADDED_BYTES] = self.0.to_be_bytes();
                let mut be_bytes: [u8; $bytes] = [0b0; $bytes];
                be_bytes.copy_from_slice(&padded_be_bytes[PADDING..]);
                be_bytes
            }
        }
    };
}

impl_float_to_be_bytes!(F8 => bytes: [u8; 1], bits: u8);
impl_float_to_be_bytes!(F16 => bytes: [u8; 2], bits: u16);
impl_float_to_be_bytes!(F24 => bytes: [u8; 3], bits: u32);
impl_float_to_be_bytes!(F32 => bytes: [u8; 4], bits: u32);
impl_float_to_be_bytes!(F40 => bytes: [u8; 5], bits: u64);
impl_float_to_be_bytes!(F48 => bytes: [u8; 6], bits: u64);
impl_float_to_be_bytes!(F56 => bytes: [u8; 7], bits: u64);
impl_float_to_be_bytes!(F64 => bytes: [u8; 8], bits: u64);

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use super::*;

    proptest! {
        #[test]
        fn f8_from_to_be_bytes_roundtrip(be_bytes_before in <[u8; 1]>::arbitrary()) {
            let float = F8::from_be_bytes(be_bytes_before);
            let be_bytes_after = float.to_be_bytes();
            prop_assert_eq!(be_bytes_before, be_bytes_after);
        }

        #[test]
        fn f16_from_to_be_bytes_roundtrip(be_bytes_before in <[u8; 2]>::arbitrary()) {
            let float = F16::from_be_bytes(be_bytes_before);
            let be_bytes_after = float.to_be_bytes();
            prop_assert_eq!(be_bytes_before, be_bytes_after);
        }

        #[test]
        fn f24_from_to_be_bytes_roundtrip(be_bytes_before in <[u8; 3]>::arbitrary()) {
            let float = F24::from_be_bytes(be_bytes_before);
            let be_bytes_after = float.to_be_bytes();
            prop_assert_eq!(be_bytes_before, be_bytes_after);
        }

        #[test]
        fn f32_from_to_be_bytes_roundtrip(be_bytes_before in <[u8; 4]>::arbitrary()) {
            let float = F32::from_be_bytes(be_bytes_before);
            let be_bytes_after = float.to_be_bytes();
            prop_assert_eq!(be_bytes_before, be_bytes_after);
        }

        #[test]
        fn f40_from_to_be_bytes_roundtrip(be_bytes_before in <[u8; 5]>::arbitrary()) {
            let float = F40::from_be_bytes(be_bytes_before);
            let be_bytes_after = float.to_be_bytes();
            prop_assert_eq!(be_bytes_before, be_bytes_after);
        }

        #[test]
        fn f48_from_to_be_bytes_roundtrip(be_bytes_before in <[u8; 6]>::arbitrary()) {
            let float = F48::from_be_bytes(be_bytes_before);
            let be_bytes_after = float.to_be_bytes();
            prop_assert_eq!(be_bytes_before, be_bytes_after);
        }

        #[test]
        fn f56_from_to_be_bytes_roundtrip(be_bytes_before in <[u8; 7]>::arbitrary()) {
            let float = F56::from_be_bytes(be_bytes_before);
            let be_bytes_after = float.to_be_bytes();
            prop_assert_eq!(be_bytes_before, be_bytes_after);
        }

        #[test]
        fn f64_from_to_be_bytes_roundtrip(be_bytes_before in <[u8; 8]>::arbitrary()) {
            let float = F64::from_be_bytes(be_bytes_before);
            let be_bytes_after = float.to_be_bytes();
            prop_assert_eq!(be_bytes_before, be_bytes_after);
        }
    }
}
