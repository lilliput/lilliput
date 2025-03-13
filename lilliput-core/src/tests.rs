use proptest::prelude::*;

use crate::{decoder::Decoder, encoder::Encoder, value::Value, Profile};

mod bool;
mod bytes;
mod float;
mod map;
mod null;
mod seq;
