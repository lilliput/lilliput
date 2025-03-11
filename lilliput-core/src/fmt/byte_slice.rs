use super::byte::Byte;

pub struct BytesSliceIter<'a>(&'a [u8]);

impl<'a> Iterator for BytesSliceIter<'a> {
    type Item = Byte;

    fn next(&mut self) -> Option<Self::Item> {
        let Some((item, slice)) = self.0.split_first() else {
            return None;
        };

        self.0 = slice;

        Some(Byte(*item))
    }
}

pub struct BytesSlice<'a>(pub &'a [u8]);

impl<'a> BytesSlice<'a> {
    pub fn iter(&self) -> BytesSliceIter<'a> {
        BytesSliceIter(self.0)
    }
}

impl<'a> IntoIterator for BytesSlice<'a> {
    type Item = <BytesSliceIter<'a> as Iterator>::Item;

    type IntoIter = BytesSliceIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a> std::fmt::Display for BytesSlice<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for (index, byte) in self.iter().enumerate() {
            if index > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{:0>2x}", byte)?;
        }
        write!(f, "]")?;
        Ok(())
    }
}

impl<'a> std::fmt::Debug for BytesSlice<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for (index, byte) in self.iter().enumerate() {
            if index > 0 {
                write!(f, ", ")?;
            }
            if f.alternate() {
                write!(f, "{:#08b}", byte)?;
            } else {
                write!(f, "{:08b}", byte)?;
            }
        }
        write!(f, "]")?;
        Ok(())
    }
}

impl<'a> std::fmt::LowerHex for BytesSlice<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            write!(f, "0x ")?;
        }
        for (index, byte) in self.iter().enumerate() {
            if index > 0 {
                write!(f, " ")?;
            }
            write!(f, "{:0>2x}", byte)?;
        }
        Ok(())
    }
}

impl<'a> std::fmt::UpperHex for BytesSlice<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            write!(f, "0x ")?;
        }
        for (index, byte) in self.iter().enumerate() {
            if index > 0 {
                write!(f, " ")?;
            }
            write!(f, "{:0>2X}", byte)?;
        }
        Ok(())
    }
}

impl<'a> std::fmt::Octal for BytesSlice<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            write!(f, "0o ")?;
        }
        for (index, byte) in self.iter().enumerate() {
            if index > 0 {
                write!(f, " ")?;
            }
            write!(f, "{:0>3o}", byte)?;
        }
        Ok(())
    }
}

impl<'a> std::fmt::Binary for BytesSlice<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            write!(f, "0b ")?;
        }
        for (index, byte) in self.iter().enumerate() {
            if index > 0 {
                write!(f, " ")?;
            }
            write!(f, "{:08b}", byte)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn debug() {
        let bytes = BytesSlice(&[42, 13, 37]);

        assert_eq!(format!("{bytes:?}"), "[00101010, 00001101, 00100101]");
        assert_eq!(
            format!("{bytes:#?}"),
            "[0b00101010, 0b00001101, 0b00100101]"
        );
    }

    #[test]
    fn display() {
        let bytes = BytesSlice(&[42, 13, 37]);

        assert_eq!(format!("{bytes}"), "[2a, 0d, 25]");
    }

    #[test]
    fn lower_hex() {
        let bytes = BytesSlice(&[42, 13, 37]);

        assert_eq!(format!("{bytes:x}"), "2a 0d 25");
        assert_eq!(format!("{bytes:#x}"), "0x 2a 0d 25");
    }

    #[test]
    fn upper_hex() {
        let bytes = BytesSlice(&[42, 13, 37]);

        assert_eq!(format!("{bytes:X}"), "2A 0D 25");
        assert_eq!(format!("{bytes:#X}"), "0x 2A 0D 25");
    }

    #[test]
    fn octal() {
        let bytes = BytesSlice(&[42, 13, 37]);

        assert_eq!(format!("{bytes:o}"), "052 015 045");
        assert_eq!(format!("{bytes:#o}"), "0o 052 015 045");
    }

    #[test]
    fn binary() {
        let bytes = BytesSlice(&[42, 13, 37]);

        assert_eq!(format!("{bytes:b}"), "00101010 00001101 00100101");
        assert_eq!(format!("{bytes:#b}"), "0b 00101010 00001101 00100101");
    }
}
