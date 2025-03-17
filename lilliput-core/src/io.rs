pub trait Read {
    type Error: std::error::Error;

    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error>;
}

pub trait BufRead: Read {
    fn fill_buf(&mut self) -> Result<&[u8], Self::Error>;
    fn consume(&mut self, amt: usize);

    fn has_data_left(&mut self) -> Result<bool, Self::Error> {
        self.fill_buf().map(|b| !b.is_empty())
    }
}

pub struct StdIoBufReader<R>(pub R);

impl<R> Read for StdIoBufReader<R>
where
    R: std::io::BufRead,
{
    type Error = std::io::Error;

    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        self.0.read(buf)
    }
}

impl<R> BufRead for StdIoBufReader<R>
where
    R: std::io::BufRead,
{
    fn fill_buf(&mut self) -> Result<&[u8], Self::Error> {
        self.0.fill_buf()
    }

    fn consume(&mut self, amt: usize) {
        self.0.consume(amt)
    }
}

pub trait Write {
    type Error: std::error::Error;

    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error>;
    fn flush(&mut self) -> Result<(), Self::Error>;
}

pub struct StdIoWriter<W>(pub W);

impl<W> Write for StdIoWriter<W>
where
    W: std::io::Write,
{
    type Error = std::io::Error;

    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        self.0.write(buf)
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}
