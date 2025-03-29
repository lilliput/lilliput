use num_traits::{Signed, ToBytes, Unsigned};

use crate::{
    error::Result,
    header::{EncodeHeader as _, IntHeader},
    io::Write,
    num::{
        int::{with_n_be_bytes, CompactWidth},
        zigzag::ToZigZag,
    },
    value::{IntValue, SignedIntValue, UnsignedIntValue},
};

use super::Encoder;

impl<W> Encoder<W>
where
    W: Write,
{
    pub fn encode_i8(&mut self, value: i8) -> Result<()> {
        self.encode_signed_int(value)
    }

    pub fn encode_i16(&mut self, value: i16) -> Result<()> {
        self.encode_signed_int(value)
    }

    pub fn encode_i32(&mut self, value: i32) -> Result<()> {
        self.encode_signed_int(value)
    }

    pub fn encode_i64(&mut self, value: i64) -> Result<()> {
        self.encode_signed_int(value)
    }

    pub fn encode_u8(&mut self, value: u8) -> Result<()> {
        self.encode_unsigned_int(value)
    }

    pub fn encode_u16(&mut self, value: u16) -> Result<()> {
        self.encode_unsigned_int(value)
    }

    pub fn encode_u32(&mut self, value: u32) -> Result<()> {
        self.encode_unsigned_int(value)
    }

    pub fn encode_u64(&mut self, value: u64) -> Result<()> {
        self.encode_unsigned_int(value)
    }

    pub fn encode_signed_int<S, U, const N: usize>(&mut self, value: S) -> Result<()>
    where
        S: Signed + ToZigZag<ZigZag = U>,
        U: Copy + Unsigned + CompactWidth + PartialOrd + From<u8> + ToBytes<Bytes = [u8; N]>,
    {
        let unsigned = value.to_zig_zag();

        let is_signed = true;

        let header = if self.config.compact_ints {
            IntHeader::optimal(is_signed, unsigned)
        } else {
            IntHeader::verbatim(is_signed, unsigned)
        };

        // Push the value's header:
        self.push_bytes(&[header.encode()])?;

        if let Some(len_width) = header.extension_width() {
            with_n_be_bytes(unsigned, len_width, |len_bytes| {
                // Push the value's extension:
                self.push_bytes(len_bytes)
            })?;
        }

        Ok(())
    }

    pub fn encode_unsigned_int<T, const N: usize>(&mut self, unsigned: T) -> Result<()>
    where
        T: Copy + Unsigned + CompactWidth + PartialOrd + From<u8> + ToBytes<Bytes = [u8; N]>,
    {
        let is_signed = false;

        let header = if self.config.compact_ints {
            IntHeader::optimal(is_signed, unsigned)
        } else {
            IntHeader::verbatim(is_signed, unsigned)
        };

        // Push the value's header:
        self.push_bytes(&[header.encode()])?;

        if let Some(len_width) = header.extension_width() {
            with_n_be_bytes(unsigned, len_width, |len_bytes| {
                // Push the value's extension:
                self.push_bytes(len_bytes)
            })?;
        }

        Ok(())
    }

    pub fn encode_signed_int_value(&mut self, value: &SignedIntValue) -> Result<()> {
        match *value {
            SignedIntValue::I8(value) => self.encode_signed_int(value),
            SignedIntValue::I16(value) => self.encode_signed_int(value),
            SignedIntValue::I32(value) => self.encode_signed_int(value),
            SignedIntValue::I64(value) => self.encode_signed_int(value),
        }
    }

    pub fn encode_unsigned_int_value(&mut self, value: &UnsignedIntValue) -> Result<()> {
        match *value {
            UnsignedIntValue::U8(value) => self.encode_unsigned_int(value),
            UnsignedIntValue::U16(value) => self.encode_unsigned_int(value),
            UnsignedIntValue::U32(value) => self.encode_unsigned_int(value),
            UnsignedIntValue::U64(value) => self.encode_unsigned_int(value),
        }
    }

    pub fn encode_int_value(&mut self, value: &IntValue) -> Result<()> {
        match value {
            IntValue::Signed(value) => self.encode_signed_int_value(value),
            IntValue::Unsigned(value) => self.encode_unsigned_int_value(value),
        }
    }
}
