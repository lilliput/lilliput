pub mod decoder;
pub mod encoder;
pub mod value;

mod fmt;

#[cfg(test)]
mod tests;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Profile {
    None = 0,
}
