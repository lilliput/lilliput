#[derive(Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub(crate) struct Byte(pub u8);

impl From<u8> for Byte {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl From<Byte> for u8 {
    fn from(value: Byte) -> Self {
        value.0
    }
}

impl std::fmt::Display for Byte {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            write!(f, "0x")?;
        }
        write!(f, "{:0>2x}", self.0)
    }
}

impl std::fmt::Debug for Byte {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            write!(f, "0b")?;
        }
        write!(f, "{:08b}", self.0)
    }
}

impl std::fmt::LowerHex for Byte {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            write!(f, "0x")?;
        }
        write!(f, "{:0>2x}", self.0)
    }
}

impl std::fmt::UpperHex for Byte {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            write!(f, "0x")?;
        }
        write!(f, "{:0>2X}", self.0)
    }
}

impl std::fmt::Octal for Byte {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            write!(f, "0o")?;
        }
        write!(f, "{:0>3o}", self.0)
    }
}

impl std::fmt::Binary for Byte {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            write!(f, "0b")?;
        }
        write!(f, "{:08b}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display() {
        let byte = Byte(42);

        assert_eq!(format!("{byte}"), "2a");
    }

    #[test]
    fn debug() {
        let byte = Byte(42);

        assert_eq!(format!("{byte:?}"), "00101010");
        assert_eq!(format!("{byte:#?}"), "0b00101010");
    }

    #[test]
    fn lower_hex() {
        let byte = Byte(42);

        assert_eq!(format!("{byte:x}"), "2a");
        assert_eq!(format!("{byte:#x}"), "0x2a");
    }

    #[test]
    fn upper_hex() {
        let byte = Byte(42);

        assert_eq!(format!("{byte:X}"), "2A");
        assert_eq!(format!("{byte:#X}"), "0x2A");
    }

    #[test]
    fn octal() {
        let byte = Byte(42);

        assert_eq!(format!("{byte:o}"), "052");
        assert_eq!(format!("{byte:#o}"), "0o052");
    }

    #[test]
    fn binary() {
        let byte = Byte(42);

        assert_eq!(format!("{byte:b}"), "00101010");
        assert_eq!(format!("{byte:#b}"), "0b00101010");
    }
}
