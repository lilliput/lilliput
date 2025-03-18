use num_traits::{PrimInt, Signed, ToBytes, Unsigned};

use crate::{
    binary::trailing_non_zero_bytes,
    error::Result,
    header::{EncodeHeader as _, IntHeader},
    io::Write,
    num::ToZigZag,
    value::{IntValue, SignedIntValue, UnsignedIntValue},
    Profile,
};

use super::Encoder;

impl<W> Encoder<W>
where
    W: Write,
{
    pub fn encode_i8(&mut self, value: i8) -> Result<()> {
        self.encode_signed(value)
    }

    pub fn encode_i16(&mut self, value: i16) -> Result<()> {
        self.encode_signed(value)
    }

    pub fn encode_i32(&mut self, value: i32) -> Result<()> {
        self.encode_signed(value)
    }

    pub fn encode_i64(&mut self, value: i64) -> Result<()> {
        self.encode_signed(value)
    }

    pub fn encode_u8(&mut self, value: u8) -> Result<()> {
        self.encode_unsigned(value)
    }

    pub fn encode_u16(&mut self, value: u16) -> Result<()> {
        self.encode_unsigned(value)
    }

    pub fn encode_u32(&mut self, value: u32) -> Result<()> {
        self.encode_unsigned(value)
    }

    pub fn encode_u64(&mut self, value: u64) -> Result<()> {
        self.encode_unsigned(value)
    }

    pub fn encode_signed_int_value(&mut self, value: &SignedIntValue) -> Result<()> {
        match *value {
            SignedIntValue::I8(value) => self.encode_signed(value),
            SignedIntValue::I16(value) => self.encode_signed(value),
            SignedIntValue::I32(value) => self.encode_signed(value),
            SignedIntValue::I64(value) => self.encode_signed(value),
        }
    }

    pub fn encode_unsigned_int_value(&mut self, value: &UnsignedIntValue) -> Result<()> {
        match *value {
            UnsignedIntValue::U8(value) => self.encode_unsigned(value),
            UnsignedIntValue::U16(value) => self.encode_unsigned(value),
            UnsignedIntValue::U32(value) => self.encode_unsigned(value),
            UnsignedIntValue::U64(value) => self.encode_unsigned(value),
        }
    }

    pub fn encode_int_value(&mut self, value: &IntValue) -> Result<()> {
        match value {
            IntValue::Signed(value) => self.encode_signed_int_value(value),
            IntValue::Unsigned(value) => self.encode_unsigned_int_value(value),
        }
    }

    pub(super) fn encode_signed<S, U, const N: usize>(&mut self, value: S) -> Result<()>
    where
        S: Signed + ToZigZag<ZigZag = U>,
        U: PrimInt + ToBytes<Bytes = [u8; N]>,
    {
        let value = value.to_zig_zag();

        // Push the value's header:
        let header = match self.profile {
            Profile::Weak => IntHeader::Extended {
                is_signed: true,
                width: trailing_non_zero_bytes(value).max(1),
            },
            Profile::None => IntHeader::Extended {
                is_signed: true,
                width: N,
            },
        };
        self.push_bytes(&[header.encode()])?;

        // Push the value's extension:
        if let IntHeader::Extended { width, .. } = header {
            let bytes = value.to_be_bytes();
            let bytes_start = bytes.len() - width;
            self.push_bytes(&bytes[bytes_start..])?;
        }

        Ok(())
    }

    pub(super) fn encode_unsigned<T, const N: usize>(&mut self, value: T) -> Result<()>
    where
        T: Unsigned + PrimInt + ToBytes<Bytes = [u8; N]>,
    {
        // Push the value's header:
        let header = match self.profile {
            Profile::Weak => IntHeader::Extended {
                is_signed: false,
                width: trailing_non_zero_bytes(value).max(1),
            },
            Profile::None => IntHeader::Extended {
                is_signed: false,
                width: N,
            },
        };
        self.push_bytes(&[header.encode()])?;

        // Push the value's extension:
        if let IntHeader::Extended { width, .. } = header {
            let bytes = value.to_be_bytes();
            let bytes_start = bytes.len() - width;
            self.push_bytes(&bytes[bytes_start..])?;
        }

        Ok(())
    }
}
