extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

pub use lilliput_core::error::{Error, Result};
pub use lilliput_core::value::Value;

pub mod value {
    pub use lilliput_core::value::{
        BoolValue, BytesValue, FloatValue, IntValue, Map, MapValue, NullValue, SeqValue,
        SignedIntValue, StringValue, UnsignedIntValue,
    };
}

mod de;
mod ser;

#[cfg(test)]
mod tests;

pub use de::{from_reader, from_slice, Deserializer};
pub use ser::{
    to_vec, to_vec_with_config, to_writer, to_writer_with_config, EncoderConfig, PackingMode,
    Serializer, SerializerConfig,
};
