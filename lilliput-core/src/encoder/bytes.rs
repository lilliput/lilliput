use crate::{
    config::PackingMode, error::Result, header::BytesHeader, io::Write,
    num::WithPackedBeBytes as _, value::BytesValue,
};

use super::Encoder;

impl<W> Encoder<W>
where
    W: Write,
{
    pub fn encode_bytes(&mut self, value: &[u8]) -> Result<()> {
        self.encode_bytes_header(&BytesHeader::new(value.len()))?;

        // Push the value's actual bytes:
        self.push_bytes(value)?;

        Ok(())
    }

    pub fn encode_bytes_value(&mut self, value: &BytesValue) -> Result<()> {
        self.encode_bytes(&value.0)
    }

    pub fn encode_bytes_header(&mut self, header: &BytesHeader) -> Result<()> {
        let len = header.len();

        // The bytes header only supports native packing:
        let packing_mode = self.config.len_packing.min(PackingMode::Native);

        len.with_packed_be_bytes(packing_mode, |width, bytes| {
            debug_assert!(width.count_ones() == 1, "must be a power of two");

            let mut header_byte = BytesHeader::TYPE_BITS;

            const EXPONENT: [u8; 8] = [0, 1, 2, 2, 3, 3, 3, 3];
            let exponent = EXPONENT[(width as usize) - 1];

            header_byte |= exponent & BytesHeader::LEN_WIDTH_EXPONENT_BITS;

            // Push the value's header:
            self.push_byte(header_byte)?;

            // Push the value's length:
            self.push_bytes(bytes)
        })
    }
}
