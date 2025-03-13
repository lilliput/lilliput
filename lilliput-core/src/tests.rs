use proptest::prelude::*;

use crate::{Profile, decoder::Decoder, encoder::Encoder, value::Value};

mod bool;
mod bytes;
mod float;
mod int;
mod map;
mod null;
mod seq;
mod string;
