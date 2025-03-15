use num_traits::{PrimInt, Signed, ToBytes, Unsigned};

use crate::{
    num::ToZigZag,
    value::{IntValue, SignedIntValue, UnsignedIntValue},
};

use super::{Encoder, EncoderError};

#[derive(Debug)]
pub(super) struct IntEncoder<'en> {
    inner: &'en mut Encoder,
}

impl<'en> IntEncoder<'en> {
    pub(super) fn with(inner: &'en mut Encoder) -> Self {
        Self { inner }
    }

    pub(super) fn encode_signed<S, U, const N: usize>(
        &mut self,
        value: S,
    ) -> Result<(), EncoderError>
    where
        S: Signed + ToZigZag<ZigZag = U>,
        U: ToBytes<Bytes = [u8; N]>,
    {
        // Push the value's metadata:
        let mut head_byte = IntValue::PREFIX_BIT;
        head_byte |= IntValue::SIGNEDNESS_BIT;

        let unsigned = value.to_zig_zag();
        let bytes = unsigned.to_be_bytes();

        // Push the value's actual bytes:
        head_byte |= (N as u8) - 1; // width of T in bytes, minus 1
        self.inner.push_byte(head_byte)?;

        self.inner.push_bytes(&bytes)?;

        Ok(())
    }

    pub(super) fn encode_unsigned<T, const N: usize>(
        &mut self,
        value: T,
    ) -> Result<(), EncoderError>
    where
        T: Unsigned + PrimInt + ToBytes<Bytes = [u8; N]>,
    {
        // Push the value's metadata:
        let mut head_byte = IntValue::PREFIX_BIT;

        let unsigned = value;
        let bytes = unsigned.to_be_bytes();

        // Push the value's actual bytes:
        head_byte |= (N as u8) - 1; // width of T in bytes, minus 1
        self.inner.push_byte(head_byte)?;

        self.inner.push_bytes(&bytes)?;

        Ok(())
    }

    pub(super) fn encode_int_value(&mut self, value: &IntValue) -> Result<(), EncoderError> {
        match value {
            IntValue::Signed(value) => match *value {
                SignedIntValue::I8(value) => self.encode_signed(value),
                SignedIntValue::I16(value) => self.encode_signed(value),
                SignedIntValue::I32(value) => self.encode_signed(value),
                SignedIntValue::I64(value) => self.encode_signed(value),
            },
            IntValue::Unsigned(value) => match *value {
                UnsignedIntValue::U8(value) => self.encode_unsigned(value),
                UnsignedIntValue::U16(value) => self.encode_unsigned(value),
                UnsignedIntValue::U32(value) => self.encode_unsigned(value),
                UnsignedIntValue::U64(value) => self.encode_unsigned(value),
            },
        }
    }
}
