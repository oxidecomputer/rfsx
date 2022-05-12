use crate::ACK;
use std::io::{Read, Result, Write};

pub struct Progress<D>(D, usize);

impl<D: Read> Read for Progress<D> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let i = self.0.read(buf)?;
        for c in buf[0..i].iter() {
            if *c == ACK {
                if self.1 % 100 == 0 {
                    print!("\rSent blocks: {} ", self.1 + 1);
                    std::io::stdout().flush().unwrap();
                }
                self.1 += 1;
            }
        }
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
    pub fn new(d: D) -> Self {
        Self(d, 0)
    }
}
