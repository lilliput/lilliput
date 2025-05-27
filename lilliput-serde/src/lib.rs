extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

pub mod value {
    pub use lilliput_core::value::*;
}

pub mod config;
pub mod de;
pub mod error;
pub mod ser;

#[cfg(test)]
mod tests;
