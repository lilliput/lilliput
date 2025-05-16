use num_traits::PrimInt;

pub(crate) trait TryFromInt<T>: PrimInt
where
    T: PrimInt,
{
    fn try_from_int(int: T) -> Result<Self, core::num::TryFromIntError>;
}

pub(crate) trait TryIntoInt<T>: PrimInt
where
    T: PrimInt,
{
    fn try_into_int(self) -> Result<T, core::num::TryFromIntError>;
}

impl<T, U> TryIntoInt<U> for T
where
    T: PrimInt,
    U: PrimInt + TryFromInt<T>,
{
    #[inline]
    fn try_into_int(self) -> Result<U, core::num::TryFromIntError> {
        U::try_from_int(self)
    }
}
