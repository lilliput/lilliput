use num_traits::float::FloatCore;

pub(crate) trait FromFloat<T>: FloatCore
where
    T: FloatCore,
{
    fn from_float(float: T) -> Self;
}

impl FromFloat<f32> for f64 {
    fn from_float(float: f32) -> Self {
        float as Self
    }
}

impl FromFloat<f64> for f32 {
    fn from_float(float: f64) -> Self {
        float as Self
    }
}

impl<T> FromFloat<T> for T
where
    T: FloatCore,
{
    fn from_float(float: Self) -> Self {
        float
    }
}

pub(crate) trait IntoFloat<T>: FloatCore
where
    T: FloatCore,
{
    // Required method
    fn into_float(self) -> T;
}

impl<T, U> IntoFloat<U> for T
where
    T: FloatCore,
    U: FloatCore + FromFloat<T>,
{
    fn into_float(self) -> U {
        U::from_float(self)
    }
}
