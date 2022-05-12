use anyhow::{bail, Result};
use libftdi1_sys as ffi;

pub fn new() -> Result<ftdi::Device> {
    let mut device = ftdi::find_by_vid_pid(0x0403, 0x6011)
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
    Ok(device)
}
