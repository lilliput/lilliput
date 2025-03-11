#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum ValueType {
    Reserved = 0b00000000,
}

impl ValueType {
    pub fn detect(byte: u8) -> Self {
        match byte.leading_zeros() {
            // 0b00000000
            _ => Self::Reserved,
        }
    }
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Value {}

impl Value {
    pub fn as_type(&self) -> ValueType {
        match self {
            _ => unreachable!(),
        }
    }

    pub fn has_type(&self, value_type: ValueType) -> bool {
        self.as_type() == value_type
    }
}
