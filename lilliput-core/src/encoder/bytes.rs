use crate::{
    config::PackingMode, error::Result, header::BytesHeader, io::Write,
    num::WithPackedBeBytes as _, value::BytesValue,
};

use super::Encoder;

impl<W> Encoder<W>
where
    W: Write,
{
    // MARK: - Value

    /// Encodes a byte array value, from a slice reference.
    pub fn encode_bytes(&mut self, value: &[u8]) -> Result<()> {
        self.encode_bytes_header(&BytesHeader::for_len(value.len()))?;

        // Push the value's actual bytes:
        self.push_bytes(value)?;

        Ok(())
    }

    /// Encodes a byte array value, from a `BytesValue`.
    pub fn encode_bytes_value(&mut self, value: &BytesValue) -> Result<()> {
        self.encode_bytes(&value.0)
    }

    // MARK: - Header

    /// Encodes a byte array value's header.
    pub fn encode_bytes_header(&mut self, header: &BytesHeader) -> Result<()> {
        let len = header.len();

        // The bytes header only supports native packing:
        let packing_mode = self.config.lengths.packing.min(PackingMode::Native);

        len.with_packed_be_bytes(packing_mode, |bytes| {
            let width = bytes.len();

            debug_assert!(width.count_ones() == 1, "must be a power of two");

            let mut byte = BytesHeader::TYPE_BITS;

            const EXPONENT: [u8; 8] = [0, 1, 2, 2, 3, 3, 3, 3];
            let exponent = EXPONENT[width - 1];

            byte |= exponent & BytesHeader::LEN_WIDTH_EXPONENT_BITS;

            #[cfg(feature = "tracing")]
            tracing::debug!(
                byte = crate::binary::fmt_byte(byte),
                bytes = format!("{:b}", crate::binary::BytesSlice(bytes)),
                len = len
            );

            // Push the value's header:
            self.push_byte(byte)?;

            // Push the value's length:
            self.push_bytes(bytes)
        })
    }

    /// Creates a header for a byte array value, from its length.
    pub fn header_for_bytes_len(&self, len: usize) -> BytesHeader {
        BytesHeader::for_len(len)
    }
}
