pub mod encoder;
pub mod value;

mod fmt;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Profile {
    None = 0,
}
