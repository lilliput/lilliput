mod byte;
mod byte_slice;

use num_traits::{PrimInt, ToBytes};

pub(crate) use self::{byte::*, byte_slice::*};

pub(crate) fn required_bytes_for_prim_int<T, const N: usize>(int: T) -> usize
where
    T: PrimInt + ToBytes<Bytes = [u8; N]>,
{
    ((N as u32) - (int.leading_zeros() / u8::BITS)) as usize
}
