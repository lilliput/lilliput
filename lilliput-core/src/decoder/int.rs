use core::num::TryFromIntError;

use num_traits::{Signed, Unsigned};

use crate::{
    error::{Error, Result},
    header::IntHeader,
    num::FromZigZag,
    value::{IntValue, SignedIntValue, UnsignedIntValue},
};

use super::{Decoder, Read};

impl<'de, R> Decoder<R>
where
    R: Read<'de>,
{
    pub fn decode_u8(&mut self) -> Result<u8> {
        let header: IntHeader = self.pull_header()?;
        self.decode_unsigned_headed_by(header)
    }

    pub fn decode_u16(&mut self) -> Result<u16> {
        let header: IntHeader = self.pull_header()?;
        self.decode_unsigned_headed_by(header)
    }

    pub fn decode_u32(&mut self) -> Result<u32> {
        let header: IntHeader = self.pull_header()?;
        self.decode_unsigned_headed_by(header)
    }

    pub fn decode_u64(&mut self) -> Result<u64> {
        let header: IntHeader = self.pull_header()?;
        self.decode_unsigned_headed_by(header)
    }

    pub fn decode_i8(&mut self) -> Result<i8> {
        let header: IntHeader = self.pull_header()?;
        self.decode_signed_headed_by(header)
    }

    pub fn decode_i16(&mut self) -> Result<i16> {
        let header: IntHeader = self.pull_header()?;
        self.decode_signed_headed_by(header)
    }

    pub fn decode_i32(&mut self) -> Result<i32> {
        let header: IntHeader = self.pull_header()?;
        self.decode_signed_headed_by(header)
    }

    pub fn decode_i64(&mut self) -> Result<i64> {
        let header: IntHeader = self.pull_header()?;
        self.decode_signed_headed_by(header)
    }

    pub fn decode_signed_int_value(&mut self) -> Result<SignedIntValue> {
        let header: IntHeader = self.pull_header()?;
        self.decode_signed_int_value_headed_by(header)
    }

    pub fn decode_unsigned_int_value(&mut self) -> Result<UnsignedIntValue> {
        let header: IntHeader = self.pull_header()?;
        self.decode_unsigned_int_value_headed_by(header)
    }

    pub fn decode_int_value(&mut self) -> Result<IntValue> {
        let header: IntHeader = self.pull_header()?;
        self.decode_int_value_headed_by(header)
    }

    fn decode_signed_headed_by<T>(&mut self, header: IntHeader) -> Result<T>
    where
        T: Signed + TryFrom<SignedIntValue, Error = TryFromIntError>,
    {
        let pos = self.pos;

        let value = self.decode_int_value_headed_by(header)?;
        let signed = value
            .to_signed()
            .and_then(|value| value.try_into())
            .map_err(|_| Error::number_out_of_range(Some(pos)))?;

        Ok(signed)
    }

    fn decode_unsigned_headed_by<T>(&mut self, header: IntHeader) -> Result<T>
    where
        T: Unsigned + TryFrom<UnsignedIntValue, Error = TryFromIntError>,
    {
        let pos = self.pos;

        let value = self.decode_int_value_headed_by(header)?;
        let unsigned = value
            .to_unsigned()
            .and_then(|value| value.try_into())
            .map_err(|_| Error::number_out_of_range(Some(pos)))?;

        Ok(unsigned)
    }

    fn decode_signed_int_value_headed_by(&mut self, header: IntHeader) -> Result<SignedIntValue> {
        let pos = self.pos;

        let value = self.decode_int_value_headed_by(header)?;
        let signed = value
            .to_signed()
            .map_err(|_| Error::number_out_of_range(Some(pos)))?;

        Ok(signed)
    }

    fn decode_unsigned_int_value_headed_by(
        &mut self,
        header: IntHeader,
    ) -> Result<UnsignedIntValue> {
        let pos = self.pos;

        let value = self.decode_int_value_headed_by(header)?;
        let unsigned = value
            .to_unsigned()
            .map_err(|_| Error::number_out_of_range(Some(pos)))?;

        Ok(unsigned)
    }

    pub(super) fn decode_int_value_headed_by(&mut self, header: IntHeader) -> Result<IntValue> {
        match header {
            IntHeader::CompactSigned { value } => Ok(IntValue::Signed(SignedIntValue::I8(value))),
            IntHeader::CompactUnsigned { value } => {
                Ok(IntValue::Unsigned(UnsignedIntValue::U8(value)))
            }
            IntHeader::Extended { is_signed, width } => {
                debug_assert!(width <= 8);
                self.pull_extended_value(width, is_signed)
            }
        }
    }

    fn pull_extended_value(&mut self, width: usize, is_signed: bool) -> Result<IntValue> {
        if is_signed {
            self.pull_signed_extended_value(width).map(IntValue::Signed)
        } else {
            self.pull_unsigned_extended_value(width)
                .map(IntValue::Unsigned)
        }
    }

    pub(super) fn pull_signed_extended_value(&mut self, width: usize) -> Result<SignedIntValue> {
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

    pub(super) fn pull_unsigned_extended_value(
        &mut self,
        width: usize,
    ) -> Result<UnsignedIntValue> {
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
