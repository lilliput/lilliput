use proptest::prelude::*;

use crate::{Profile, decoder::Decoder, encoder::Encoder, value::Value};

mod bool;
mod bytes;
mod null;
