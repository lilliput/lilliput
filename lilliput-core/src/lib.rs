pub mod decoder;
pub mod encoder;
pub mod value;

mod binary;
mod num;

#[cfg(test)]
mod tests;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Profile {
    None = 0,
}
