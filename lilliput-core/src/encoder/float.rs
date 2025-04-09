use crate::{
    error::Result, header::FloatHeader, io::Write, num::WithPackedBeBytes, value::FloatValue,
};

use super::Encoder;

impl<W> Encoder<W>
where
    W: Write,
{
    pub fn encode_f32(&mut self, value: f32) -> Result<()> {
        value.with_packed_be_bytes(self.config.float_packing, |bytes| {
            self.encode_float_header(&FloatHeader::new(bytes.len() as u8))?;

            // Push the value itself:
            self.push_bytes(bytes)
        })
    }

    pub fn encode_f64(&mut self, value: f64) -> Result<()> {
        value.with_packed_be_bytes(self.config.float_packing, |bytes| {
            self.encode_float_header(&FloatHeader::new(bytes.len() as u8))?;

            // Push the value itself:
            self.push_bytes(bytes)
        })
    }

    pub fn encode_float_value(&mut self, value: &FloatValue) -> Result<()> {
        match value {
            FloatValue::F32(value) => self.encode_f32(*value),
            FloatValue::F64(value) => self.encode_f64(*value),
        }
    }

    pub fn encode_float_header(&mut self, header: &FloatHeader) -> Result<()> {
        let width = header.width();

        let mut header_byte = FloatHeader::TYPE_BITS;

        header_byte |= (width - 1) & FloatHeader::VALUE_WIDTH_BITS;

        // Push the value's header:
        self.push_byte(header_byte)
    }

    pub fn header_for_f32(&self, value: f32) -> FloatHeader {
        FloatHeader::for_f32(value, self.config.int_packing)
    }

    pub fn header_for_f64(&self, value: f64) -> FloatHeader {
        FloatHeader::for_f64(value, self.config.int_packing)
    }
}
