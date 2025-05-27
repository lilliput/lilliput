use crate::{
    error::Result, header::FloatHeader, io::Write, num::WithValidatedPackedBeBytes as _,
    value::FloatValue,
};

use super::Encoder;

impl<W> Encoder<W>
where
    W: Write,
{
    // MARK: - Value

    /// Encodes a 32-bit floating-point value.
    pub fn encode_f32(&mut self, value: f32) -> Result<()> {
        let validator = self.config.floats.validation.f32.clone();

        value.with_validated_packed_be_bytes(self.config.floats.packing, &validator, |bytes| {
            self.encode_float_header(&FloatHeader::new(bytes.len() as u8))?;

            // Push the value itself:
            self.push_bytes(bytes)
        })
    }

    /// Encodes a 64-bit floating-point value.
    pub fn encode_f64(&mut self, value: f64) -> Result<()> {
        let validator = self.config.floats.validation.f64.clone();

        value.with_validated_packed_be_bytes(self.config.floats.packing, &validator, |bytes| {
            self.encode_float_header(&FloatHeader::new(bytes.len() as u8))?;

            // Push the value itself:
            self.push_bytes(bytes)
        })
    }

    /// Encodes a floating-point value, from a `FloatValue`.
    pub fn encode_float_value(&mut self, value: &FloatValue) -> Result<()> {
        match value {
            FloatValue::F32(value) => self.encode_f32(*value),
            FloatValue::F64(value) => self.encode_f64(*value),
        }
    }

    // MARK: - Header

    /// Encodes a floating-point value's header.
    pub fn encode_float_header(&mut self, header: &FloatHeader) -> Result<()> {
        let width = header.width();

        let mut byte = FloatHeader::TYPE_BITS;

        byte |= (width - 1) & FloatHeader::VALUE_WIDTH_BITS;

        #[cfg(feature = "tracing")]
        tracing::debug!(byte = crate::binary::fmt_byte(byte), width = width);

        // Push the value's header:
        self.push_byte(byte)
    }
}
