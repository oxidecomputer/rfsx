use crate::ACK;
use indicatif::ProgressBar;
use std::io::{Read, Result, Write};

pub struct Progress<D>(D, ProgressBar);

impl<D: Read> Read for Progress<D> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let i = self.0.read(buf)?;
        self.1
            .inc(buf[0..i].iter().filter(|c| **c == ACK).count() as u64);
        Ok(i)
    }
}

impl<D: Write> Write for Progress<D> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.0.write(buf)
    }
    fn flush(&mut self) -> Result<()> {
        self.0.flush()
    }
}

impl<D> Progress<D> {
    pub fn new(d: D, blocks: u64) -> Self {
        let bar = ProgressBar::new(blocks);
        Self(d, bar)
    }
}
