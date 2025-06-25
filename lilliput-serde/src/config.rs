//! Configurations used for serializing values.

use lilliput_core::config::EncoderConfig;

/// The representation to serialize structs to.
#[derive(Default, Clone, Eq, PartialEq, Debug)]
pub enum StructRepr {
    /// Serialize as sequence of fields.
    #[default]
    Seq,
    /// Serialize as map of fields.
    Map,
}

/// The representation to serialize enums to.
#[derive(Default, Clone, Eq, PartialEq, Debug)]
pub enum EnumVariantRepr {
    /// Serialize variant index as discriminant.
    #[default]
    Index,
    /// Serialize variant name as discriminant.
    Name,
}

/// Configuration used for serializing values.
#[derive(Default, Clone, Debug)]
pub struct SerializerConfig {
    /// The representation to serialize structs to.
    pub struct_repr: StructRepr,
    /// The representation to serialize enums to.
    pub enum_variant_repr: EnumVariantRepr,
    /// Low-level configuration for encoding values.
    pub encoder: EncoderConfig,
}

impl SerializerConfig {
    /// Sets struct-repr to `struct_repr`, returning `self`.
    pub fn with_struct_repr(mut self, struct_repr: StructRepr) -> Self {
        self.struct_repr = struct_repr;
        self
    }

    /// Sets enum-variant-repr to `enum_variant_repr`, returning `self`.
    pub fn with_enum_variant_repr(mut self, enum_variant_repr: EnumVariantRepr) -> Self {
        self.enum_variant_repr = enum_variant_repr;
        self
    }

    /// Sets encoder to `encoder`, returning `self`.
    pub fn with_encoder(mut self, encoder: EncoderConfig) -> Self {
        self.encoder = encoder;
        self
    }
}
