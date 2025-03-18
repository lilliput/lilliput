use std::ops::Deref;

use crate::error::{Error, Result};

pub struct Position {
    pub byte: usize,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Reference<'b, 'c, T>
where
    T: ?Sized + 'static,
{
    Borrowed(&'b T),
    Copied(&'c T),
}

impl<T> Deref for Reference<'_, '_, T>
where
    T: ?Sized + 'static,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match *self {
            Self::Borrowed(b) => b,
            Self::Copied(c) => c,
        }
    }
}

// MARK: - Read

pub trait Read<'r> {
    fn read<'s>(
        &'s mut self,
        len: usize,
        scratch: &'s mut Vec<u8>,
    ) -> Result<Reference<'r, 's, [u8]>>;

    fn read_into(&mut self, buf: &mut [u8]) -> Result<()>;

    fn read_one(&mut self) -> Result<u8> {
        let mut bytes: [u8; 1] = [0b0];
        self.read_into(&mut bytes)?;
        Ok(bytes[0])
    }
}

// MARK: - StdIoReader

pub struct StdIoReader<R> {
    reader: R,
}

impl<R> StdIoReader<R> {
    pub fn new(reader: R) -> Self {
        Self { reader }
    }
}

impl<'r, R> Read<'r> for StdIoReader<R>
where
    R: std::io::Read,
{
    fn read<'s>(
        &'s mut self,
        len: usize,
        scratch: &'s mut Vec<u8>,
    ) -> Result<Reference<'r, 's, [u8]>> {
        // Copied from the default buffer length of `std::io::BufReader`:
        const MAX_CHUNK_LENGTH: usize = 8192;

        let mut total_read = 0;

        while total_read < len {
            let remaining = len - total_read;
            let to_read = remaining.min(MAX_CHUNK_LENGTH);

            let old_len = scratch.len();
            scratch.resize(old_len + to_read, 0);

            let read = self
                .reader
                .read(&mut scratch[old_len..])
                .map_err(Error::io)?;

            if read < to_read {
                return Err(Error::end_of_file());
            }

            total_read += read;
        }

        Ok(Reference::Copied(scratch))
    }

    fn read_into(&mut self, buf: &mut [u8]) -> Result<()> {
        self.reader.read_exact(buf).map_err(Error::io)
    }
}

// MARK: - SliceReader

pub struct SliceReader<'r> {
    slice: &'r [u8],
}

impl<'r> SliceReader<'r> {
    pub fn new(slice: &'r [u8]) -> Self {
        Self { slice }
    }
}

impl<'r> Read<'r> for SliceReader<'r> {
    #[inline]
    fn read<'s>(
        &'s mut self,
        len: usize,
        _scratch: &'s mut Vec<u8>,
    ) -> Result<Reference<'r, 's, [u8]>> {
        if len > self.slice.len() {
            return Err(Error::end_of_file());
        }

        let (prefix, suffix) = self.slice.split_at(len);

        self.slice = suffix;

        Ok(Reference::Borrowed(prefix))
    }

    fn read_into(&mut self, buf: &mut [u8]) -> Result<()> {
        let len = buf.len();

        if len > self.slice.len() {
            return Err(Error::end_of_file());
        }

        let (prefix, suffix) = self.slice.split_at(len);

        self.slice = suffix;

        buf.copy_from_slice(prefix);
        Ok(())
    }
}

// MARK: - Write

pub trait Write {
    type Error: std::error::Error;

    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn flush(&mut self) -> Result<()>;
}

// MARK: - MutSliceWriter

pub struct MutSliceWriter<'w> {
    slice: &'w mut [u8],
    pos: usize,
}

impl<'w> MutSliceWriter<'w> {
    pub fn new(slice: &'w mut Vec<u8>) -> Self {
        Self { slice, pos: 0 }
    }
}

impl Write for MutSliceWriter<'_> {
    type Error = Error;

    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let len = buf.len();

        if len > self.slice.len() - self.pos {
            return Err(Error::end_of_file());
        }

        let range = self.pos..(self.pos + len);
        self.slice[range].copy_from_slice(buf);

        self.pos += len;

        Ok(len)
    }

    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}

// MARK: - VecWriter

pub struct VecWriter<'w> {
    vec: &'w mut Vec<u8>,
}

impl<'w> VecWriter<'w> {
    pub fn new(vec: &'w mut Vec<u8>) -> Self {
        Self { vec }
    }

    pub fn vec(&self) -> &[u8] {
        self.vec
    }
}

impl Write for VecWriter<'_> {
    type Error = Error;

    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.vec.extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}

// MARK: - StdIoBufWriter

pub struct StdIoWriter<W> {
    writer: W,
}

impl<W> StdIoWriter<W> {
    pub fn new(writer: W) -> Self {
        Self { writer }
    }

    pub fn into_writer(self) -> W {
        self.writer
    }
}

impl<W> Write for StdIoWriter<W>
where
    W: std::io::Write,
{
    type Error = Error;

    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.writer.write(buf).map_err(Error::io)
    }

    fn flush(&mut self) -> Result<()> {
        self.writer.flush().map_err(Error::io)
    }
}

#[cfg(test)]
mod test {
    use crate::error::ErrorCode;

    use super::*;

    mod std_io_reader {
        use super::*;

        #[test]
        fn read() {
            let slice: &[u8] = &[1, 2, 3, 4, 5];
            let mut reader = StdIoReader::new(slice);
            let mut scratch = Vec::new();

            match reader.read(1, &mut scratch).unwrap() {
                Reference::Borrowed(_) => {
                    panic!("reader should always copy");
                }
                Reference::Copied(bytes) => {
                    assert_eq!(bytes, &[1]);
                }
            }

            scratch.clear();

            match reader.read(2, &mut scratch).unwrap() {
                Reference::Borrowed(_) => {
                    panic!("reader should always copy");
                }
                Reference::Copied(bytes) => {
                    assert_eq!(bytes, &[2, 3]);
                }
            }

            scratch.clear();

            assert_eq!(
                reader.read(3, &mut scratch).err().unwrap().code(),
                ErrorCode::UnexpectedEndOfFile
            );
        }

        #[test]
        fn read_into() {
            let slice: &[u8] = &[1, 2, 3, 4, 5];
            let mut reader = StdIoReader::new(slice);
            let mut scratch = Vec::new();

            let bytes = &mut [0];
            reader.read_into(bytes).unwrap();
            assert_eq!(bytes, &[1]);

            scratch.clear();

            let bytes = &mut [0, 0];
            reader.read_into(bytes).unwrap();
            assert_eq!(bytes, &[2, 3]);

            scratch.clear();

            assert_eq!(
                reader.read(3, &mut scratch).err().unwrap().code(),
                ErrorCode::UnexpectedEndOfFile
            );
        }
    }

    mod slice_reader {
        use super::*;

        #[test]
        fn read() {
            let slice: &[u8] = &[1, 2, 3, 4, 5];
            let mut reader = SliceReader::new(slice);
            let mut scratch = Vec::new();

            match reader.read(1, &mut scratch).unwrap() {
                Reference::Borrowed(bytes) => {
                    assert_eq!(bytes, &[1]);
                }
                Reference::Copied(_) => {
                    panic!("reader should always borrow");
                }
            }

            scratch.clear();

            match reader.read(2, &mut scratch).unwrap() {
                Reference::Borrowed(bytes) => {
                    assert_eq!(bytes, &[2, 3]);
                }
                Reference::Copied(_) => {
                    panic!("reader should always borrow");
                }
            }

            scratch.clear();

            assert_eq!(
                reader.read(3, &mut scratch).err().unwrap().code(),
                ErrorCode::UnexpectedEndOfFile
            );
        }

        #[test]
        fn read_into() {
            let slice: &[u8] = &[1, 2, 3, 4, 5];
            let mut reader = SliceReader::new(slice);
            let mut scratch = Vec::new();

            let bytes = &mut [0];
            reader.read_into(bytes).unwrap();
            assert_eq!(bytes, &[1]);

            scratch.clear();

            let bytes = &mut [0, 0];
            reader.read_into(bytes).unwrap();
            assert_eq!(bytes, &[2, 3]);

            scratch.clear();

            assert_eq!(
                reader.read(3, &mut scratch).err().unwrap().code(),
                ErrorCode::UnexpectedEndOfFile
            );
        }
    }
}
