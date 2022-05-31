// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::io::{ErrorKind, Read, Result, Write};
use std::time::{Duration, Instant};

use anyhow::bail;
use libftdi1_sys as ffi;

const TIMEOUT: Duration = Duration::from_secs(30);

/// Handle which wraps an FTDI device and adds retries / timeouts
pub struct Device(ftdi::Device, usize);

impl Read for Device {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let timeout = Instant::now() + TIMEOUT;
        let mut i = 0;

        while i < buf.len() {
            if let Ok(d) = self.0.read(&mut buf[i..]) {
                i += d;
            }
            if Instant::now() > timeout {
                return Err(ErrorKind::TimedOut.into());
            }
        }
        Ok(i)
    }
}

impl Write for Device {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let timeout = Instant::now() + TIMEOUT;
        let mut i = 0;

        while i < buf.len() {
            match self.0.write(&buf[i..]) {
                Ok(d) => i += d,
                Err(_) => continue,
            }
            if Instant::now() > timeout {
                return Err(ErrorKind::TimedOut.into());
            }
        }
        Ok(i)
    }
    fn flush(&mut self) -> Result<()> {
        self.0.flush()
    }
}

////////////////////////////////////////////////////////////////////////////////

pub fn new(vid: Option<u16>, pid: Option<u16>) -> anyhow::Result<Device> {
    let mut device =
        ftdi::find_by_vid_pid(vid.unwrap_or(0x0403), pid.unwrap_or(0x6011))
            .interface(ftdi::Interface::A)
            .open()?;

    device.usb_reset()?;
    device.configure(
        ftdi::Bits::Eight,
        ftdi::StopBits::One,
        ftdi::Parity::None,
    )?;
    device.usb_purge_buffers()?;
    device.set_baud_rate(3_000_000)?;
    device.set_latency_timer(1)?;
    device.set_flow_control(ftdi::FlowControl::RtsCts)?;
    device.set_bitmode(0xFF, ftdi::BitMode::Reset)?;
    if unsafe { ffi::ftdi_setdtr_rts(device.libftdi_context(), 1, 1) } != 0 {
        bail!("Could not set DTR / RTS");
    }
    Ok(Device(device, 0))
}
