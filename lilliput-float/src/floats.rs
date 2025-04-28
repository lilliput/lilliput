/// A bit-level representation of a 8-bit floating-point number.
///
/// The bits are laid out as follows:
/// - Sign bit: 1 bit
/// - Exponent width: 4 bits
/// - Significand precision: 4 bits (3 explicitly stored)
///
/// ```plain
///  MSB   ...   LSB
/// ┌─┬─┬─┬─┬─┬─┬─┬─┐
/// └─┴─┴─┴─┴─┴─┴─┴─┘
///  │ ├─────┘ ├───┘
///  │ │       └ Significand (3 bits)
///  │ └ Exponent (4 bits)
///  └ Sign (1 bit)
///  ```
#[derive(Default, Copy, Clone)]
#[repr(transparent)]
pub struct F8(pub(crate) u8);

impl std::fmt::Debug for F8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:08b}", self.0)
    }
}

/// A bit-level representation of a 16-bit floating-point number.
///
/// The bits are laid out as follows:
/// - Sign bit: 1 bit
/// - Exponent width: 5 bits
/// - Significand precision: 11 bits (10 explicitly stored)
///
/// ```plain
///  MSB           ...           LSB
/// ┌─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┐
/// └─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┘
///  │ ├───────┘ ├─────────────────┘
///  │ │         └ Significand (10 bits)
///  │ └ Exponent (5 bits)
///  └ Sign (1 bit)
///  ```
#[derive(Default, Copy, Clone)]
#[repr(transparent)]
pub struct F16(pub(crate) u16);

impl std::fmt::Debug for F16 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:016b}", self.0)
    }
}

/// A bit-level representation of a 24-bit floating-point number.
///
/// The bits are laid out as follows:
/// - Sign bit: 1 bit
/// - Exponent width: 7 bits
/// - Significand precision: 17 bits (16 explicitly stored)
///
/// ```plain
///  MSB                      ...                     LSB
/// ┌─┬╴╴┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┐
/// └─┴╴╴┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┘
///  ├╴╴┘ │ ├───────────┘ ├─────────────────────────────┘
///  │    │ │             └ Significand (16 bits)
///  │    │ └ Exponent (7 bits)
///  │    └ Sign (1 bit)
///  └ Padding (8 bits)
///  ```
#[derive(Default, Copy, Clone)]
#[repr(transparent)]
pub struct F24(pub(crate) u32);

impl std::fmt::Debug for F24 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:024b}", self.0)
    }
}

/// A bit-level representation of a 32-bit floating-point number.
///
/// The bits are laid out as follows:
/// - Sign bit: 1 bit
/// - Exponent width: 8 bits
/// - Significand precision: 24 bits (23 explicitly stored)
///
/// ```plain
///  MSB                              ...                             LSB
/// ┌─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┐
/// └─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┘
///  │ ├─────────────┘ ├───────────────────────────────────────────┘
///  │ │               └ Significand (23 bits)
///  │ └ Exponent (8 bits)
///  └ Sign (1 bit)
///  ```
#[derive(Default, Copy, Clone)]
#[repr(transparent)]
pub struct F32(pub(crate) u32);

impl std::fmt::Debug for F32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:032b}", self.0)
    }
}

/// A bit-level representation of a 40-bit floating-point number.
///
/// The bits are laid out as follows:
/// - Sign bit: 1 bit
/// - Exponent width: 8 bits
/// - Significand precision: 32 bits (31 explicitly stored)
///
/// ```plain
///  MSB                              ...                             LSB
/// ┌─┬╴╴┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬╴╴╴┬─┐
/// └─┴╴╴┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴╴╴╴┴─┘
///  ├╴╴┘ │ ├─────────────┘ ├───────────────────────────────────────╴╴╴─┘
///  │    │ │               └ Significand (31 bits)
///  │    │ └ Exponent (8 bits)
///  │    └ Sign (1 bit)
///  └ Padding (24 bits)
///  ```
#[derive(Default, Copy, Clone)]
#[repr(transparent)]
pub struct F40(pub(crate) u64);

impl std::fmt::Debug for F40 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:040b}", self.0)
    }
}

/// A bit-level representation of a 48-bit floating-point number.
///
/// The bits are laid out as follows:
/// - Sign bit: 1 bit
/// - Exponent width: 9 bits
/// - Significand precision: 39 bits (38 explicitly stored)
///
/// ```plain
///  MSB                              ...                             LSB
/// ┌─┬╴╴┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬╴╴╴┬─┐
/// └─┴╴╴┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴╴╴╴┴─┘
///  ├╴╴┘ │ ├───────────────┘ ├─────────────────────────────────────╴╴╴─┘
///  │    │ │                 └ Significand (38 bits)
///  │    │ └ Exponent (9 bits)
///  │    └ Sign (1 bit)
///  └ Padding (16 bits)
///  ```
#[derive(Default, Copy, Clone)]
#[repr(transparent)]
pub struct F48(pub(crate) u64);

impl std::fmt::Debug for F48 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:048b}", self.0)
    }
}

/// A bit-level representation of a 56-bit floating-point number.
///
/// The bits are laid out as follows:
/// - Sign bit: 1 bit
/// - Exponent width: 10 bits
/// - Significand precision: 46 bits (45 explicitly stored)
///
/// ```plain
///  MSB                              ...                             LSB
/// ┌─┬╴╴┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬╴╴╴┬─┐
/// └─┴╴╴┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴╴╴╴┴─┘
///  ├╴╴┘ │ ├─────────────────┘ ├───────────────────────────────────╴╴╴─┘
///  │    │ │                   └ Significand (45 bits)
///  │    │ └ Exponent (10 bits)
///  │    └ Sign (1 bit)
///  └ Padding (8 bits)
///  ```
#[derive(Default, Copy, Clone)]
#[repr(transparent)]
pub struct F56(pub(crate) u64);

impl std::fmt::Debug for F56 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:056b}", self.0)
    }
}

/// A bit-level representation of a 64-bit floating-point number.
///
/// The bits are laid out as follows:
/// - Sign bit: 1 bit
/// - Exponent width: 1 bits
/// - Significand precision: 53 bits (52 explicitly stored)
///
/// ```plain
///  MSB                           ...                           LSB
/// ┌─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬╴╴╴┬─┐
/// └─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴╴╴╴┴─┘
///  │ ├─────────────────┘ ├───────────────────────────────────╴╴╴─┘
///  │ │                   └ Significand (45 bits)
///  │ └ Exponent (10 bits)
///  └ Sign (1 bit)
///  ```
#[derive(Default, Copy, Clone)]
#[repr(transparent)]
pub struct F64(pub(crate) u64);

impl std::fmt::Debug for F64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:064b}", self.0)
    }
}
