//! A serializer and deserializer of the lilliput data format, for serde.

#![warn(missing_docs)]

extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

/// Values.
pub mod value {
    pub use lilliput_core::value::*;
}

pub mod config;
pub mod de;
pub mod error;
pub mod ser;

/// The crates's prelude.
pub mod prelude {
    pub use crate::{config::*, de::*, error::Error, ser::*, value::*};
}

#[cfg(test)]
mod tests;
