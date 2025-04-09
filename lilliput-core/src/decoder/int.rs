use core::num::TryFromIntError;

use num_traits::{Signed, Unsigned};

use crate::{
    error::{Error, Result},
    header::{CompactIntHeader, ExtendedIntHeader, IntHeader},
    marker::Marker,
    num::zigzag::FromZigZag,
    value::{IntValue, SignedIntValue, UnsignedIntValue},
};

use super::{Decoder, Read};

impl<'de, R> Decoder<R>
where
    R: Read<'de>,
{
    // MARK: - Value

    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
    pub fn decode_u8(&mut self) -> Result<u8> {
        self.decode_unsigned_int()
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
    pub fn decode_u16(&mut self) -> Result<u16> {
        self.decode_unsigned_int()
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
    pub fn decode_u32(&mut self) -> Result<u32> {
        self.decode_unsigned_int()
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
    pub fn decode_u64(&mut self) -> Result<u64> {
        self.decode_unsigned_int()
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
    pub fn decode_i8(&mut self) -> Result<i8> {
        self.decode_signed_int()
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
    pub fn decode_i16(&mut self) -> Result<i16> {
        self.decode_signed_int()
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
    pub fn decode_i32(&mut self) -> Result<i32> {
        self.decode_signed_int()
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
    pub fn decode_i64(&mut self) -> Result<i64> {
        self.decode_signed_int()
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
    pub fn decode_signed_int<T>(&mut self) -> Result<T>
    where
        T: Signed + TryFrom<SignedIntValue, Error = TryFromIntError>,
    {
        let pos = self.pos;

        self.decode_signed_int_value()?
            .try_into()
            .map_err(|_| Error::number_out_of_range(Some(pos)))
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
    pub fn decode_unsigned_int<T>(&mut self) -> Result<T>
    where
        T: Unsigned + TryFrom<UnsignedIntValue, Error = TryFromIntError>,
    {
        let pos = self.pos;

        self.decode_unsigned_int_value()?
            .try_into()
            .map_err(|_| Error::number_out_of_range(Some(pos)))
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
    pub fn decode_signed_int_value(&mut self) -> Result<SignedIntValue> {
        let pos = self.pos;

        self.decode_int_value()?
            .to_signed()
            .map_err(|_| Error::number_out_of_range(Some(pos)))
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
    pub fn decode_unsigned_int_value(&mut self) -> Result<UnsignedIntValue> {
        let pos = self.pos;

        self.decode_int_value()?
            .to_unsigned()
            .map_err(|_| Error::number_out_of_range(Some(pos)))
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
    pub fn decode_int_value(&mut self) -> Result<IntValue> {
        let header = self.decode_int_header()?;
        self.decode_int_value_of(header)
    }

    // MARK: - Header

    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
    pub fn decode_int_header(&mut self) -> Result<IntHeader> {
        let byte = self.pull_byte_expecting(Marker::Int)?;

        if (byte & IntHeader::COMPACT_VARIANT_BIT) != 0b0 {
            let is_signed = (byte & IntHeader::SIGNEDNESS_BIT) != 0b0;
            let bits = byte & IntHeader::COMPACT_VALUE_BITS;

            #[cfg(feature = "tracing")]
            tracing::debug!(
                byte = crate::binary::fmt_byte(byte),
                is_compact = true,
                is_signed = is_signed,
                bits = bits
            );

            Ok(IntHeader::Compact(CompactIntHeader { is_signed, bits }))
        } else {
            let is_signed = (byte & IntHeader::SIGNEDNESS_BIT) != 0b0;
            let width = 1 + (byte & IntHeader::EXTENDED_WIDTH_BITS);

            #[cfg(feature = "tracing")]
            tracing::debug!(
                byte = crate::binary::fmt_byte(byte),
                is_compact = false,
                is_signed = is_signed,
                width = width
            );

            Ok(IntHeader::Extended(ExtendedIntHeader { is_signed, width }))
        }
    }

    // MARK: - Body

    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
    pub fn decode_int_value_of(&mut self, header: IntHeader) -> Result<IntValue> {
        let (is_signed, width): (bool, usize) = match header {
            IntHeader::Compact(CompactIntHeader { is_signed, bits }) => {
                if is_signed {
                    let value = i8::from_zig_zag(bits);

                    #[cfg(feature = "tracing")]
                    tracing::debug!(value = value);

                    return Ok(IntValue::Signed(SignedIntValue::I8(value)));
                } else {
                    let value = bits;

                    #[cfg(feature = "tracing")]
                    tracing::debug!(value = value);

                    return Ok(IntValue::Unsigned(UnsignedIntValue::U8(value)));
                }
            }
            IntHeader::Extended(ExtendedIntHeader { is_signed, width }) => {
                (is_signed, width as usize)
            }
        };

        match width {
            1..=1 => {
                const MAX_WIDTH: usize = 1;
                let mut padded_be_bytes: [u8; MAX_WIDTH] = [0b0; MAX_WIDTH];
                self.pull_bytes_into(&mut padded_be_bytes[(MAX_WIDTH - width)..])?;

                #[cfg(feature = "tracing")]
                let bytes = crate::binary::fmt_bytes(&padded_be_bytes[(MAX_WIDTH - width)..]);

                let value = u8::from_be_bytes(padded_be_bytes);

                if is_signed {
                    let value = i8::from_zig_zag(value);

                    #[cfg(feature = "tracing")]
                    tracing::debug!(bytes = bytes, value = value);

                    Ok(IntValue::Signed(SignedIntValue::I8(value)))
                } else {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(bytes = bytes, value = value);

                    Ok(IntValue::Unsigned(UnsignedIntValue::U8(value)))
                }
            }
            2..=2 => {
                const MAX_WIDTH: usize = 2;
                let mut padded_be_bytes: [u8; MAX_WIDTH] = [0b0; MAX_WIDTH];
                self.pull_bytes_into(&mut padded_be_bytes[(MAX_WIDTH - width)..])?;

                #[cfg(feature = "tracing")]
                let bytes = crate::binary::fmt_bytes(&padded_be_bytes[(MAX_WIDTH - width)..]);

                let value = u16::from_be_bytes(padded_be_bytes);

                if is_signed {
                    let value = i16::from_zig_zag(value);

                    #[cfg(feature = "tracing")]
                    tracing::debug!(bytes = bytes, value = value);

                    Ok(IntValue::Signed(SignedIntValue::I16(value)))
                } else {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(bytes = bytes, value = value);

                    Ok(IntValue::Unsigned(UnsignedIntValue::U16(value)))
                }
            }
            3..=4 => {
                const MAX_WIDTH: usize = 4;
                let mut padded_be_bytes: [u8; MAX_WIDTH] = [0b0; MAX_WIDTH];
                self.pull_bytes_into(&mut padded_be_bytes[(MAX_WIDTH - width)..])?;

                #[cfg(feature = "tracing")]
                let bytes = crate::binary::fmt_bytes(&padded_be_bytes[(MAX_WIDTH - width)..]);

                let value = u32::from_be_bytes(padded_be_bytes);

                if is_signed {
                    let value = i32::from_zig_zag(value);

                    #[cfg(feature = "tracing")]
                    tracing::debug!(bytes = bytes, value = value);

                    Ok(IntValue::Signed(SignedIntValue::I32(value)))
                } else {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(bytes = bytes, value = value);

                    Ok(IntValue::Unsigned(UnsignedIntValue::U32(value)))
                }
            }
            5..=8 => {
                const MAX_WIDTH: usize = 8;
                let mut padded_be_bytes: [u8; MAX_WIDTH] = [0b0; MAX_WIDTH];
                self.pull_bytes_into(&mut padded_be_bytes[(MAX_WIDTH - width)..])?;

                #[cfg(feature = "tracing")]
                let bytes = crate::binary::fmt_bytes(&padded_be_bytes[(MAX_WIDTH - width)..]);

                let value = u64::from_be_bytes(padded_be_bytes);

                if is_signed {
                    let value = i64::from_zig_zag(value);

                    #[cfg(feature = "tracing")]
                    tracing::debug!(bytes = bytes, value = value);

                    Ok(IntValue::Signed(SignedIntValue::I64(value)))
                } else {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(bytes = bytes, value = value);

                    Ok(IntValue::Unsigned(UnsignedIntValue::U64(value)))
                }
            }
            _ => unreachable!(),
        }
    }
}
