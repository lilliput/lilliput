use num_traits::{PrimInt, Signed, ToBytes, Unsigned};

use crate::{
    binary::required_bytes_for_prim_int,
    header::{EncodeHeader as _, IntHeader},
    io::Write,
    num::ToZigZag,
    value::{IntValue, SignedIntValue, UnsignedIntValue},
    Profile,
};

use super::{Encoder, EncoderError};

#[derive(Debug)]
pub(super) struct IntEncoder<'en, W> {
    inner: &'en mut Encoder<W>,
}

impl<'en, W> IntEncoder<'en, W>
where
    W: Write,
{
    pub(super) fn with(inner: &'en mut Encoder<W>) -> Self {
        Self { inner }
    }

    pub(super) fn encode_signed<S, U, const N: usize>(
        &mut self,
        value: S,
    ) -> Result<(), EncoderError>
    where
        S: Signed + ToZigZag<ZigZag = U>,
        U: PrimInt + ToBytes<Bytes = [u8; N]>,
    {
        let value = value.to_zig_zag();

        // Push the value's header:
        let header = match self.inner.profile {
            Profile::Weak => IntHeader::Extended {
                is_signed: true,
                width: required_bytes_for_prim_int(value),
            },
            Profile::None => IntHeader::Extended {
                is_signed: true,
                width: N,
            },
        };
        self.inner.push_bytes(&[header.encode()])?;

        // Push the value's extension:
        if let IntHeader::Extended { width, .. } = header {
            let bytes = value.to_be_bytes();
            let bytes_start = bytes.len() - width;
            self.inner.push_bytes(&bytes[bytes_start..])?;
        }

        Ok(())
    }

    pub(super) fn encode_unsigned<T, const N: usize>(
        &mut self,
        value: T,
    ) -> Result<(), EncoderError>
    where
        T: Unsigned + PrimInt + ToBytes<Bytes = [u8; N]>,
    {
        // Push the value's header:
        let header = match self.inner.profile {
            Profile::Weak => IntHeader::Extended {
                is_signed: false,
                width: required_bytes_for_prim_int(value),
            },
            Profile::None => IntHeader::Extended {
                is_signed: false,
                width: N,
            },
        };
        self.inner.push_bytes(&[header.encode()])?;

        // Push the value's extension:
        if let IntHeader::Extended { width, .. } = header {
            let bytes = value.to_be_bytes();
            let bytes_start = bytes.len() - width;
            self.inner.push_bytes(&bytes[bytes_start..])?;
        }

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
