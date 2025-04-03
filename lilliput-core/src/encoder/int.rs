use num_traits::{Signed, Unsigned};

use crate::{
    binary::bits_if,
    error::Result,
    header::IntHeader,
    io::Write,
    num::WithPackedBeBytes,
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

    pub fn encode_signed_int_value(&mut self, value: &SignedIntValue) -> Result<()> {
        match value {
            SignedIntValue::I8(value) => self.encode_signed_int(*value),
            SignedIntValue::I16(value) => self.encode_signed_int(*value),
            SignedIntValue::I32(value) => self.encode_signed_int(*value),
            SignedIntValue::I64(value) => self.encode_signed_int(*value),
        }
    }

    pub fn encode_unsigned_int_value(&mut self, value: &UnsignedIntValue) -> Result<()> {
        match value {
            UnsignedIntValue::U8(value) => self.encode_unsigned_int(*value),
            UnsignedIntValue::U16(value) => self.encode_unsigned_int(*value),
            UnsignedIntValue::U32(value) => self.encode_unsigned_int(*value),
            UnsignedIntValue::U64(value) => self.encode_unsigned_int(*value),
        }
    }

    pub fn encode_int_value(&mut self, value: &IntValue) -> Result<()> {
        match value {
            IntValue::Signed(value) => self.encode_signed_int_value(value),
            IntValue::Unsigned(value) => self.encode_unsigned_int_value(value),
        }
    }

    pub fn encode_int_header(&mut self, header: &IntHeader) -> Result<()> {
        let mut header_byte = IntHeader::TYPE_BITS;

        match header {
            IntHeader::Compact(compact) => {
                header_byte |= IntHeader::COMPACT_VARIANT_BIT;
                header_byte |= bits_if(IntHeader::SIGNEDNESS_BIT, compact.is_signed());
                header_byte |= compact.bits() & IntHeader::COMPACT_VALUE_BITS;
            }
            IntHeader::Extended(extended) => {
                header_byte |= bits_if(IntHeader::SIGNEDNESS_BIT, extended.is_signed());
                header_byte |= (extended.width() - 1) & IntHeader::EXTENDED_WIDTH_BITS;
            }
        }

        self.push_byte(header_byte)
    }

    #[inline]
    fn encode_signed_int<S>(&mut self, value: S) -> Result<()>
    where
        S: Signed + WithPackedBeBytes,
    {
        let packing_mode = self.config.int_packing;

        value.with_packed_be_bytes(packing_mode, |_width, bytes| {
            let (header, is_compact) = IntHeader::be_bytes(true, bytes, packing_mode);
            self.encode_int_header(&header)?;

            if is_compact {
                return Ok(());
            }

            // Push the value' bytes:
            self.push_bytes(bytes)
        })
    }

    #[inline]
    fn encode_unsigned_int<U>(&mut self, value: U) -> Result<()>
    where
        U: Unsigned + WithPackedBeBytes,
    {
        let packing_mode = self.config.int_packing;

        value.with_packed_be_bytes(packing_mode, |_width, bytes| {
            let (header, is_compact) = IntHeader::be_bytes(false, bytes, packing_mode);
            self.encode_int_header(&header)?;

            if is_compact {
                return Ok(());
            }

            // Push the value' bytes:
            self.push_bytes(bytes)
        })
    }
}
