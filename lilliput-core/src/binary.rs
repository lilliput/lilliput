mod byte;
mod byte_slice;

pub(crate) use self::byte_slice::*;

/// Conditionally sets bits (branch-less).
#[inline]
pub(crate) fn bits_if(bits: u8, condition: bool) -> u8 {
    bits & all_bits_if(condition)
}

#[inline]
pub(crate) fn all_bits_if(condition: bool) -> u8 {
    !(condition as u8).wrapping_sub(1)
}
