use lilliput_core::config::EncoderConfig;

#[derive(Default, Clone, Eq, PartialEq, Debug)]
pub enum StructRepr {
    #[default]
    Seq,
    Map,
}

#[derive(Default, Clone, Eq, PartialEq, Debug)]
pub enum EnumVariantRepr {
    #[default]
    Index,
    Name,
}

#[derive(Default, Clone, PartialEq, Debug)]
pub struct SerializerConfig {
    pub struct_repr: StructRepr,
    pub enum_variant_repr: EnumVariantRepr,
    pub encoder: EncoderConfig,
}

impl SerializerConfig {
    pub fn with_struct_repr(mut self, struct_repr: StructRepr) -> Self {
        self.struct_repr = struct_repr;
        self
    }

    pub fn with_enum_variant_repr(mut self, enum_variant_repr: EnumVariantRepr) -> Self {
        self.enum_variant_repr = enum_variant_repr;
        self
    }

    pub fn with_encoder(mut self, encoder: EncoderConfig) -> Self {
        self.encoder = encoder;
        self
    }
}
