use core::num::TryFromIntError;

use num_traits::{FromBytes, PrimInt, Signed, Unsigned};

use crate::{
    num::{FromZigZag, TryFromInt, TryIntoInt as _},
    value::{IntValue, SignedIntValue, UnsignedIntValue, ValueType},
};

use super::{Decoder, DecoderError};

#[derive(Eq, PartialEq, Debug, thiserror::Error)]
pub enum IntDecoderError {
    #[error("integer value out of bounds")]
    OutOfBounds(#[from] TryFromIntError),
}

#[derive(Debug)]
pub struct IntDecoder<'a, 'de> {
    inner: &'de mut Decoder<'a>,
}

impl<'a, 'de> IntDecoder<'a, 'de> {
    pub(super) fn with(inner: &'de mut Decoder<'a>) -> Self {
        Self { inner }
    }

    pub(super) fn decode_signed<T>(&mut self) -> Result<T, DecoderError>
    where
        T: Signed + TryFromInt<i8> + TryFromInt<i16> + TryFromInt<i32> + TryFromInt<i64>,
    {
        let value = match self.decode_signed_value()? {
            SignedIntValue::I8(value) => value.try_into_int(),
            SignedIntValue::I16(value) => value.try_into_int(),
            SignedIntValue::I32(value) => value.try_into_int(),
            SignedIntValue::I64(value) => value.try_into_int(),
        }
        .map_err(IntDecoderError::from)?;

        Ok(value)
    }

    pub(super) fn decode_unsigned<T>(&mut self) -> Result<T, DecoderError>
    where
        T: Unsigned + TryFromInt<u8> + TryFromInt<u16> + TryFromInt<u32> + TryFromInt<u64>,
    {
        let value = match self.decode_unsigned_value()? {
            UnsignedIntValue::U8(value) => value.try_into_int(),
            UnsignedIntValue::U16(value) => value.try_into_int(),
            UnsignedIntValue::U32(value) => value.try_into_int(),
            UnsignedIntValue::U64(value) => value.try_into_int(),
        }
        .map_err(IntDecoderError::from)?;

        Ok(value)
    }

    pub(super) fn decode_signed_value(&mut self) -> Result<SignedIntValue, DecoderError> {
        let byte = self.inner.pull_byte_expecting_type(ValueType::Int)?;

        if byte & IntValue::SIGNEDNESS_BIT == 0b0 {
            return Err(DecoderError::Other);
        }

        let is_long = byte & IntValue::VARIANT_BIT != 0b0;

        if is_long {
            let is_valid = byte & IntValue::LONG_RESERVED_BITS == 0b0;
            assert!(is_valid, "padding bits should be zero");

            let size_len = (byte & IntValue::LONG_WIDTH_BITS) as usize + 1;
            let bytes = self.inner.pull_bytes(size_len)?;

            let signed = match Self::unsigned_from_be_bytes(bytes) {
                UnsignedIntValue::U8(unsigned) => SignedIntValue::I8(i8::from_zig_zag(unsigned)),
                UnsignedIntValue::U16(unsigned) => SignedIntValue::I16(i16::from_zig_zag(unsigned)),
                UnsignedIntValue::U32(unsigned) => SignedIntValue::I32(i32::from_zig_zag(unsigned)),
                UnsignedIntValue::U64(unsigned) => SignedIntValue::I64(i64::from_zig_zag(unsigned)),
            };

            Ok(signed)
        } else {
            Err(DecoderError::IncompatibleProfile)
        }
    }

    pub(super) fn decode_unsigned_value(&mut self) -> Result<UnsignedIntValue, DecoderError> {
        let byte = self.inner.pull_byte_expecting_type(ValueType::Int)?;

        if byte & IntValue::SIGNEDNESS_BIT != 0b0 {
            return Err(DecoderError::Other);
        }

        let is_long = byte & IntValue::VARIANT_BIT != 0b0;

        if is_long {
            let is_valid = byte & IntValue::LONG_RESERVED_BITS == 0b0;
            assert!(is_valid, "padding bits should be zero");

            let size_len = (byte & IntValue::LONG_WIDTH_BITS) as usize + 1;
            let bytes = self.inner.pull_bytes(size_len)?;

            let unsigned = Self::unsigned_from_be_bytes(bytes);

            Ok(unsigned)
        } else {
            Err(DecoderError::IncompatibleProfile)
        }
    }

    pub(super) fn decode_int_value(&mut self) -> Result<IntValue, DecoderError> {
        let byte = self.inner.peek_byte_expecting_type(ValueType::Int)?;
        let is_signed = byte & IntValue::SIGNEDNESS_BIT != 0b0;

        if is_signed {
            Ok(IntValue::Signed(self.decode_signed_value()?))
        } else {
            Ok(IntValue::Unsigned(self.decode_unsigned_value()?))
        }
    }

    fn unsigned_from_be_bytes(bytes: &[u8]) -> UnsignedIntValue {
        debug_assert!(bytes.len() <= 8);

        #[inline(always)]
        fn from_be_bytes<T, const N: usize>(bytes: &[u8]) -> T
        where
            T: Unsigned + PrimInt + FromBytes<Bytes = [u8; N]>,
        {
            let bytes_len = bytes.len();
            debug_assert!(bytes_len <= N);

            let mut padded_bytes: [u8; N] = [0b0; N];
            padded_bytes[(N - bytes_len)..].copy_from_slice(bytes);

            T::from_be_bytes(&padded_bytes)
        }

        match bytes.len() {
            1..=1 => UnsignedIntValue::U8(from_be_bytes(bytes)),
            2..=2 => UnsignedIntValue::U16(from_be_bytes(bytes)),
            3..=4 => UnsignedIntValue::U32(from_be_bytes(bytes)),
            5..=8 => UnsignedIntValue::U64(from_be_bytes(bytes)),
            _ => unreachable!(),
        }
    }
}
