use crate::{
    error::Result, header::FloatHeader, io::Write, num::WithPackedBeBytes, value::FloatValue,
};

use super::Encoder;

impl<W> Encoder<W>
where
    W: Write,
{
    pub fn encode_f32(&mut self, value: f32) -> Result<()> {
        value.with_packed_be_bytes(self.config.float_packing, |width, bytes| {
            let mut header_byte = FloatHeader::TYPE_BITS;

            header_byte |= (width - 1) & FloatHeader::VALUE_WIDTH_BITS;

            // Push the value's header:
            self.push_byte(header_byte)?;

            // Push the value itself:
            self.push_bytes(bytes)
        })
    }

    pub fn encode_f64(&mut self, value: f64) -> Result<()> {
        self.encode_float_header(&FloatHeader::new(FloatValue::F64(value)))
    }

    pub fn encode_float_value(&mut self, value: &FloatValue) -> Result<()> {
        match value {
            FloatValue::F32(value) => self.encode_f32(*value),
            FloatValue::F64(value) => self.encode_f64(*value),
        }
    }

    pub fn encode_float_header(&mut self, header: &FloatHeader) -> Result<()> {
        let packing_mode = self.config.float_packing;

        let encode_float = |width: u8, bytes: &[u8]| {
            let mut header_byte = FloatHeader::TYPE_BITS;

            header_byte |= (width - 1) & FloatHeader::VALUE_WIDTH_BITS;

            // Push the value's header:
            self.push_byte(header_byte)?;

            // Push the value itself:
            self.push_bytes(bytes)
        };

        match header.value() {
            FloatValue::F32(value) => value.with_packed_be_bytes(packing_mode, encode_float),
            FloatValue::F64(value) => value.with_packed_be_bytes(packing_mode, encode_float),
        }
    }
}
