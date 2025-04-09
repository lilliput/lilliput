use num_traits::{Signed, Unsigned};

use crate::{
    binary::bits_if,
    error::Result,
    header::{CompactIntHeader, ExtendedIntHeader, IntHeader},
    io::Write,
    num::WithPackedBeBytes,
    value::{IntValue, SignedIntValue, UnsignedIntValue},
};

use super::Encoder;

impl<W> Encoder<W>
where
    W: Write,
{
    // MARK: - Value

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

    // MARK: - Header

    pub fn encode_int_header(&mut self, header: &IntHeader) -> Result<()> {
        let mut header_byte = IntHeader::TYPE_BITS;

        match header {
            IntHeader::Compact(CompactIntHeader { is_signed, bits }) => {
                header_byte |= IntHeader::COMPACT_VARIANT_BIT;
                header_byte |= bits_if(IntHeader::SIGNEDNESS_BIT, *is_signed);
                header_byte |= bits & IntHeader::COMPACT_VALUE_BITS;
            }
            IntHeader::Extended(ExtendedIntHeader { is_signed, width }) => {
                header_byte |= bits_if(IntHeader::SIGNEDNESS_BIT, *is_signed);
                header_byte |= (width - 1) & IntHeader::EXTENDED_WIDTH_BITS;
            }
        }

        // Push the header byte:
        self.push_byte(header_byte)
    }

    pub fn header_for_signed_int<T>(&self, value: T) -> IntHeader
    where
        T: Signed + WithPackedBeBytes,
    {
        IntHeader::for_signed(value, self.config.int_packing)
    }

    pub fn header_for_unsigned_int<T>(&self, value: T) -> IntHeader
    where
        T: Unsigned + WithPackedBeBytes,
    {
        IntHeader::for_unsigned(value, self.config.int_packing)
    }

    #[inline]
    fn encode_signed_int<S>(&mut self, value: S) -> Result<()>
    where
        S: Signed + WithPackedBeBytes,
    {
        value.with_packed_be_bytes(self.config.int_packing, |bytes| {
            let is_signed = true;
            let width = bytes.len();

            let mut header = IntHeader::extended(is_signed, width as u8);

            if width == 1 {
                let bits = bytes[width - 1];
                if bits <= IntHeader::COMPACT_VALUE_BITS {
                    header = IntHeader::compact(is_signed, bits);

                    return self.encode_int_header(&header);
                }
            }

            self.encode_int_header(&header)?;

            // Push the value bytes:
            self.push_bytes(bytes)
        })
    }

    #[inline]
    fn encode_unsigned_int<U>(&mut self, value: U) -> Result<()>
    where
        U: Unsigned + WithPackedBeBytes,
    {
        value.with_packed_be_bytes(self.config.int_packing, |bytes| {
            let is_signed = false;
            let width = bytes.len();

            let mut header = IntHeader::extended(is_signed, width as u8);

            if width == 1 {
                let bits = bytes[width - 1];
                if bits <= IntHeader::COMPACT_VALUE_BITS {
                    header = IntHeader::compact(is_signed, bits);

                    return self.encode_int_header(&header);
                }
            }

            self.encode_int_header(&header)?;

            // Push the value bytes:
            self.push_bytes(bytes)
        })
    }
}
