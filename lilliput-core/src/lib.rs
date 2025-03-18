extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

pub mod decoder;
pub mod encoder;
pub mod error;
pub mod header;
pub mod io;
pub mod value;

mod binary;
mod num;

#[derive(Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Profile {
    None = 0,
    #[default]
    Weak = 1,
}
