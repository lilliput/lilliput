pub mod decoder;
pub mod encoder;
pub mod io;
pub mod value;

mod binary;
mod header;
mod num;

#[cfg(test)]
mod tests;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Profile {
    None = 0,
    Weak = 1,
}
