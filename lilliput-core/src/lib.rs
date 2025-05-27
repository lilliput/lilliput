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

#[derive(Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Profile {
    None = 0,
    #[default]
    Weak = 1,
}

pub mod prelude {
    pub use crate::{
        config::*, decoder::*, encoder::*, error::Error, header::*, io::*, marker::*, value::*,
    };
}
