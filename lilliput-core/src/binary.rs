mod byte;
mod byte_slice;

use num_traits::{PrimInt, ToBytes};

pub(crate) use self::{byte::*, byte_slice::*};

pub(crate) fn leading_zero_bytes<T>(int: T) -> usize
where
    T: PrimInt,
{
    (int.leading_zeros() / u8::BITS) as usize
}

pub(crate) fn trailing_non_zero_bytes<T, const N: usize>(int: T) -> usize
where
    T: PrimInt + ToBytes<Bytes = [u8; N]>,
{
    N - leading_zero_bytes(int)
}
