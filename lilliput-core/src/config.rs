#[cfg_attr(any(test, feature = "testing"), derive(proptest_derive::Arbitrary))]
#[derive(Default, Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum PackingMode {
    None,
    Native,
    #[default]
    Optimal,
}

#[cfg_attr(any(test, feature = "testing"), derive(proptest_derive::Arbitrary))]
#[derive(Default, Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct EncodingConfig {
    pub len_packing: PackingMode,
    pub int_packing: PackingMode,
    pub float_packing: PackingMode,
}

#[cfg_attr(any(test, feature = "testing"), derive(proptest_derive::Arbitrary))]
#[derive(Default, Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct DecodingConfig {}
