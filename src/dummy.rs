// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::io::{Read, Result, Write};

use crate::ACK;

pub enum Dummy {
    Start,
    Running,
}

pub fn new() -> Dummy {
    Dummy::Start
}

impl Read for Dummy {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        if buf.is_empty() {
            return Ok(0);
        }
        match self {
            Self::Start => buf[0] = b'C',
            Self::Running => buf[0] = ACK,
        }
        Ok(1)
    }
}

impl Write for Dummy {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        *self = Self::Running;
        Ok(buf.len())
    }
    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}
