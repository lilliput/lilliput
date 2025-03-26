use core::num::TryFromIntError;
use std::ops::Range;

use num_traits::{Signed, Unsigned};

use crate::{
    error::{Error, Result},
    header::{IntHeader, IntHeaderRepr},
    num::zigzag::FromZigZag,
    value::{IntValue, SignedIntValue, UnsignedIntValue},
};

use super::{Decoder, Read};

impl<'de, R> Decoder<R>
where
    R: Read<'de>,
{
    pub fn decode_u8(&mut self) -> Result<u8> {
        self.decode_unsigned_int()
    }

    pub fn decode_u16(&mut self) -> Result<u16> {
        self.decode_unsigned_int()
    }

    pub fn decode_u32(&mut self) -> Result<u32> {
        self.decode_unsigned_int()
    }

    pub fn decode_u64(&mut self) -> Result<u64> {
        self.decode_unsigned_int()
    }

    pub fn decode_i8(&mut self) -> Result<i8> {
        self.decode_signed_int()
    }

    pub fn decode_i16(&mut self) -> Result<i16> {
        self.decode_signed_int()
    }

    pub fn decode_i32(&mut self) -> Result<i32> {
        self.decode_signed_int()
    }

    pub fn decode_i64(&mut self) -> Result<i64> {
        self.decode_signed_int()
    }

    pub fn decode_signed_int<T>(&mut self) -> Result<T>
    where
        T: Signed + TryFrom<SignedIntValue, Error = TryFromIntError>,
    {
        let (value, range) = self.decode_int_value_and_range()?;

        let signed = value
            .to_signed()
            .and_then(|value| value.try_into())
            .map_err(|_| Error::number_out_of_range(Some(range.start)))?;

        Ok(signed)
    }

    pub fn decode_unsigned_int<T>(&mut self) -> Result<T>
    where
        T: Unsigned + TryFrom<UnsignedIntValue, Error = TryFromIntError>,
    {
        let (value, range) = self.decode_int_value_and_range()?;

        let unsigned = value
            .to_unsigned()
            .and_then(|value| value.try_into())
            .map_err(|_| Error::number_out_of_range(Some(range.start)))?;

        Ok(unsigned)
    }

    pub fn decode_signed_int_value(&mut self) -> Result<SignedIntValue> {
        let (value, range) = self.decode_int_value_and_range()?;

        let signed = value
            .to_signed()
            .map_err(|_| Error::number_out_of_range(Some(range.start)))?;

        Ok(signed)
    }

    pub fn decode_unsigned_int_value(&mut self) -> Result<UnsignedIntValue> {
        let (value, range) = self.decode_int_value_and_range()?;

        let unsigned = value
            .to_unsigned()
            .map_err(|_| Error::number_out_of_range(Some(range.start)))?;

        Ok(unsigned)
    }

    pub fn decode_int_value(&mut self) -> Result<IntValue> {
        let (value, _) = self.decode_int_value_and_range()?;

        Ok(value)
    }

    pub(super) fn decode_int_value_and_range(&mut self) -> Result<(IntValue, Range<usize>)> {
        let header: IntHeader = self.pull_header()?;

        let pos = self.pos;

        match header.repr() {
            IntHeaderRepr::Compact { is_signed, bits } => {
                let int_value = if is_signed {
                    IntValue::Signed(SignedIntValue::I8(i8::from_zig_zag(bits)))
                } else {
                    IntValue::Unsigned(UnsignedIntValue::U8(bits))
                };
                Ok((int_value, pos..(pos + 1)))
            }
            IntHeaderRepr::Extended { is_signed, width } => {
                let int_value = self.pull_extended_value(width, is_signed)?;
                Ok((int_value, pos..(pos + usize::from(width))))
            }
        }
    }

    fn pull_extended_value(&mut self, width: u8, is_signed: bool) -> Result<IntValue> {
        if is_signed {
            self.pull_signed_extended_value(width).map(IntValue::Signed)
        } else {
            self.pull_unsigned_extended_value(width)
                .map(IntValue::Unsigned)
        }
    }

    pub(super) fn pull_signed_extended_value(&mut self, width: u8) -> Result<SignedIntValue> {
        let width: usize = width.into();

        const MAX_WIDTH: usize = 8;
        debug_assert!(width <= MAX_WIDTH);

        let mut padded_bytes: [u8; MAX_WIDTH] = [0; MAX_WIDTH];
        let padding: usize = MAX_WIDTH - width;
        let bytes = &mut padded_bytes[padding..];
        self.pull_bytes_into(bytes)?;

        let signed = match bytes.len() {
            1..=1 => SignedIntValue::I8(i8::from_zig_zag(u8::from_be_bytes(
                <[u8; 1]>::try_from(&padded_bytes[7..]).unwrap(),
            ))),
            2..=2 => SignedIntValue::I16(i16::from_zig_zag(u16::from_be_bytes(
                <[u8; 2]>::try_from(&padded_bytes[6..]).unwrap(),
            ))),
            3..=4 => SignedIntValue::I32(i32::from_zig_zag(u32::from_be_bytes(
                <[u8; 4]>::try_from(&padded_bytes[4..]).unwrap(),
            ))),
            5..=8 => SignedIntValue::I64(i64::from_zig_zag(u64::from_be_bytes(
                <[u8; 8]>::try_from(&padded_bytes[0..]).unwrap(),
            ))),
            _ => unreachable!(),
        };

        Ok(signed)
    }

    pub(super) fn pull_unsigned_extended_value(&mut self, width: u8) -> Result<UnsignedIntValue> {
        let width: usize = width.into();

        const MAX_WIDTH: usize = 8;
        debug_assert!(width <= MAX_WIDTH);

        let mut padded_bytes: [u8; MAX_WIDTH] = [0; MAX_WIDTH];
        let padding: usize = MAX_WIDTH - width;
        let bytes = &mut padded_bytes[padding..];
        self.pull_bytes_into(bytes)?;

        let unsigned = match bytes.len() {
            1..=1 => UnsignedIntValue::U8(u8::from_be_bytes(
                <[u8; 1]>::try_from(&padded_bytes[7..]).unwrap(),
            )),
            2..=2 => UnsignedIntValue::U16(u16::from_be_bytes(
                <[u8; 2]>::try_from(&padded_bytes[6..]).unwrap(),
            )),
            3..=4 => UnsignedIntValue::U32(u32::from_be_bytes(
                <[u8; 4]>::try_from(&padded_bytes[4..]).unwrap(),
            )),
            5..=8 => UnsignedIntValue::U64(u64::from_be_bytes(
                <[u8; 8]>::try_from(&padded_bytes[0..]).unwrap(),
            )),
            _ => unreachable!(),
        };

        Ok(unsigned)
    }
}
