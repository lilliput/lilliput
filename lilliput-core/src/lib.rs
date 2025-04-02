extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

pub mod config;
pub mod decoder;
pub mod encoder;
pub mod error;
pub mod header;
pub mod io;
pub mod marker;
pub mod value;

mod binary;
mod num;
mod sealed;

#[derive(Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Profile {
    None = 0,
    #[default]
    Weak = 1,
}
