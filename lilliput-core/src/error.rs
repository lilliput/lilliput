//! When encoding or decoding Lilliput goes wrong.

use alloc::boxed::Box;
use alloc::string::ToString;
use core::fmt::{self, Debug, Display};
use core::result;

/// Alias for a `Result` with the error type `Error`.
pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub struct Expectation<U, E = U> {
    pub unexpected: U,
    pub expected: E,
}

pub struct Error {
    kind: Box<ErrorKind>,
    pos: Option<usize>,
}

impl Error {
    #[cold]
    pub(crate) fn new(kind: Box<ErrorKind>, pos: Option<usize>) -> Self {
        Self { kind, pos }
    }

    /// EOF while parsing.
    #[cold]
    pub fn end_of_file() -> Self {
        Self::new(Box::new(ErrorKind::end_of_file()), None)
    }

    /// A mismatch occurred between the decoded and expected value types.
    #[cold]
    pub fn invalid_type(unexpected: String, expected: String, pos: Option<usize>) -> Self {
        Self::new(Box::new(ErrorKind::invalid_type(unexpected, expected)), pos)
    }

    /// The enclosed I/O error occurred while trying to read the encoded
    /// MessagePack data.
    #[cold]
    pub fn invalid_value(unexpected: String, expected: String, pos: Option<usize>) -> Self {
        Self::new(
            Box::new(ErrorKind::invalid_value(unexpected, expected)),
            pos,
        )
    }

    /// A decoded sequence/map did not have the enclosed expected length.
    #[cold]
    pub fn invalid_length(unexpected: String, expected: String, pos: Option<usize>) -> Self {
        Self::new(
            Box::new(ErrorKind::invalid_length(unexpected, expected)),
            pos,
        )
    }

    /// An encoded sequence/map did not provide a length.
    #[cold]
    pub fn unknown_length() -> Self {
        Self::new(Box::new(ErrorKind::unknown_length()), None)
    }

    /// A numeric cast failed due to an out-of-range error.
    #[cold]
    pub fn number_out_of_range(pos: Option<usize>) -> Self {
        Self::new(Box::new(ErrorKind::number_out_of_range()), pos)
    }

    /// An otherwise uncategorized error occurred.
    #[cold]
    pub fn uncategorized(msg: impl Display, pos: Option<usize>) -> Self {
        Self::new(Box::new(ErrorKind::uncategorized(msg)), pos)
    }

    /// The depth limit was exceeded.
    #[cold]
    pub fn depth_limit_exceeded(pos: Option<usize>) -> Self {
        Self::new(Box::new(ErrorKind::depth_limit_exceeded()), pos)
    }

    /// An encoded string could not be parsed as UTF-8.
    #[cold]
    pub fn utf8(err: core::str::Utf8Error, pos: Option<usize>) -> Self {
        Self::new(Box::new(ErrorKind::utf8(err)), pos)
    }

    #[cfg(feature = "std")]
    pub fn io(err: std::io::Error) -> Self {
        Self::new(Box::new(ErrorKind::io(err)), None)
    }

    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }

    pub fn pos(&self) -> Option<usize> {
        self.pos
    }

    pub fn code(&self) -> ErrorCode {
        self.kind.as_code()
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Humans often end up seeing this representation because it is what `.unwrap()` shows.
        if let Some(pos) = self.pos {
            write!(f, "Error({:?}, position: {pos:?})", self.kind.to_string())
        } else {
            write!(f, "Error({:?})", self.kind.to_string())
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Humans often end up seeing this representation because it is what `.unwrap()` shows.
        if let Some(pos) = self.pos {
            write!(f, "{:?}, at position: {pos:?}", self.kind.to_string())
        } else {
            write!(f, "{:?}", self.kind.to_string(),)
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &*self.kind {
            ErrorKind::UnexpectedEndOfFile => None,
            ErrorKind::InvalidType(_) => None,
            ErrorKind::InvalidValue(_) => None,
            ErrorKind::InvalidLength(_) => None,
            ErrorKind::UnknownLength => None,
            ErrorKind::NumberOutOfRange => None,
            ErrorKind::Uncategorized(_) => None,
            ErrorKind::DepthLimitExceeded => None,
            ErrorKind::Utf8(err) => Some(err),
            #[cfg(feature = "std")]
            ErrorKind::StdIo(err) => Some(err),
        }
    }
}

#[cfg(feature = "serde")]
impl serde::de::Error for Error {
    #[cold]
    fn custom<T>(msg: T) -> Error
    where
        T: Display,
    {
        Error::uncategorized(msg, None)
    }

    #[cold]
    fn invalid_type(unexp: serde::de::Unexpected, exp: &dyn serde::de::Expected) -> Self {
        Error::invalid_type(unexp.to_string(), exp.to_string(), None)
    }

    #[cold]
    fn invalid_value(unexp: serde::de::Unexpected, exp: &dyn serde::de::Expected) -> Self {
        Error::invalid_value(unexp.to_string(), exp.to_string(), None)
    }

    #[cold]
    fn invalid_length(len: usize, exp: &dyn serde::de::Expected) -> Self {
        Error::invalid_length(len.to_string(), exp.to_string(), None)
    }
}

#[cfg(feature = "serde")]
impl serde::ser::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Error::uncategorized(msg, None)
    }
}

/// This type represents all possible errors that can occur when serializing or
/// deserializing Lilliput data.
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum ErrorCode {
    /// Unexpected EOF while parsing.
    UnexpectedEndOfFile = 1,
    /// A mismatch occurred between the decoded and expected value types.
    InvalidType = 11,
    /// The enclosed I/O error occurred while trying to read the encoded
    /// MessagePack data.
    InvalidValue = 21,
    /// A decoded sequence/map did not have the enclosed expected length.
    InvalidLength = 31,
    /// An encoded sequence/map did not provide a length.
    UnknownLength = 41,
    /// A numeric cast failed due to an out-of-range error.
    NumberOutOfRange = 51,
    /// An otherwise uncategorized error occurred.
    Uncategorized = 61,
    /// The depth limit was exceeded.
    DepthLimitExceeded = 71,
    /// An encoded string could not be parsed as UTF-8.
    Utf8 = 81,
    #[cfg(feature = "std")]
    StdIo = 255,
}

/// This type represents all possible errors that can occur when serializing or
/// deserializing Lilliput data.
#[derive(Debug)]
pub enum ErrorKind {
    /// Unexpected EOF while parsing.
    UnexpectedEndOfFile,
    /// A mismatch occurred between the decoded and expected value types.
    InvalidType(Expectation<String>),
    /// The enclosed I/O error occurred while trying to read the encoded
    /// MessagePack data.
    InvalidValue(Expectation<String>),
    /// A decoded sequence/map did not have the enclosed expected length.
    InvalidLength(Expectation<String>),
    /// An encoded sequence/map did not provide a length.
    UnknownLength,
    /// A numeric cast failed due to an out-of-range error.
    NumberOutOfRange,
    /// An otherwise uncategorized error occurred.
    Uncategorized(String),
    /// The depth limit was exceeded.
    DepthLimitExceeded,
    /// An encoded string could not be parsed as UTF-8.
    Utf8(core::str::Utf8Error),
    #[cfg(feature = "std")]
    StdIo(std::io::Error),
}

impl ErrorKind {
    /// EOF while parsing.
    fn end_of_file() -> Self {
        Self::UnexpectedEndOfFile
    }

    /// A mismatch occurred between the decoded and expected value types.
    fn invalid_type(unexpected: String, expected: String) -> Self {
        Self::InvalidType(Expectation {
            unexpected,
            expected,
        })
    }

    /// The enclosed I/O error occurred while trying to read the encoded
    /// MessagePack data.
    fn invalid_value(unexpected: String, expected: String) -> Self {
        Self::InvalidValue(Expectation {
            unexpected,
            expected,
        })
    }

    /// A decoded sequence/map did not have the enclosed expected length.
    fn invalid_length(unexpected: String, expected: String) -> Self {
        Self::InvalidLength(Expectation {
            unexpected,
            expected,
        })
    }

    /// An encoded sequence/map did not provide a length.
    #[cold]
    pub fn unknown_length() -> Self {
        Self::UnknownLength
    }

    /// A numeric cast failed due to an out-of-range error.
    fn number_out_of_range() -> Self {
        Self::NumberOutOfRange
    }

    /// An otherwise uncategorized error occurred.
    fn uncategorized(msg: impl Display) -> Self {
        Self::Uncategorized(msg.to_string())
    }

    /// The depth limit was exceeded.
    fn depth_limit_exceeded() -> Self {
        Self::DepthLimitExceeded
    }

    /// An encoded string could not be parsed as UTF-8.
    fn utf8(err: core::str::Utf8Error) -> Self {
        Self::Utf8(err)
    }

    #[cfg(feature = "std")]
    fn io(err: std::io::Error) -> Self {
        if err.kind() == std::io::ErrorKind::UnexpectedEof {
            return Self::UnexpectedEndOfFile;
        }

        Self::StdIo(err)
    }

    pub fn as_code(&self) -> ErrorCode {
        match self {
            ErrorKind::UnexpectedEndOfFile => ErrorCode::UnexpectedEndOfFile,
            ErrorKind::InvalidType(_) => ErrorCode::InvalidType,
            ErrorKind::InvalidValue(_) => ErrorCode::InvalidValue,
            ErrorKind::InvalidLength(_) => ErrorCode::InvalidLength,
            ErrorKind::UnknownLength => ErrorCode::UnknownLength,
            ErrorKind::NumberOutOfRange => ErrorCode::NumberOutOfRange,
            ErrorKind::Uncategorized(_) => ErrorCode::Uncategorized,
            ErrorKind::DepthLimitExceeded => ErrorCode::DepthLimitExceeded,
            ErrorKind::Utf8(_) => ErrorCode::Utf8,
            ErrorKind::StdIo(_) => ErrorCode::StdIo,
        }
    }
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::UnexpectedEndOfFile => f.write_str("unexpected EOF while parsing"),
            Self::InvalidType(unexpected) => {
                write!(
                    f,
                    "expected type {}, found type {}",
                    unexpected.expected, unexpected.unexpected
                )
            }
            Self::InvalidValue(unexpected) => {
                write!(
                    f,
                    "expected data {}, found type {}",
                    unexpected.expected, unexpected.unexpected
                )
            }
            Self::InvalidLength(unexpected) => {
                write!(
                    f,
                    "expected length {}, found length {}",
                    unexpected.expected, unexpected.unexpected
                )
            }
            Self::UnknownLength => f.write_str("unknown length"),
            Self::NumberOutOfRange => f.write_str("unexpected EOF while parsing"),
            Self::Uncategorized(msg) => f.write_str(msg),
            Self::DepthLimitExceeded => {
                f.write_str("a numeric cast failed due to an out-of-range error")
            }
            Self::Utf8(err) => Display::fmt(err, f),
            #[cfg(feature = "std")]
            Self::StdIo(err) => Display::fmt(err, f),
        }
    }
}
