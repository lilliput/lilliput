//! Low-level implementation of encoding/decoding logic for lilliput format.

#![warn(missing_docs)]

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
mod sealed;

#[doc(hidden)]
pub(crate) mod num;

/// Internal names, not for external use.
///
/// # WARNING
///
/// The contents of this module are NOT subject to semver.
#[doc(hidden)]
pub mod plumbing {
    pub use super::num::*;
}

pub mod prelude {
    pub use crate::{
        config::*, decoder::*, encoder::*, error::Error, header::*, io::*, marker::*, value::*,
    };
}
