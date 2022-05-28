use std::io::{Read, Write};

use anyhow::{anyhow, Context, Result};
use clap::Parser;

mod dummy;
mod ftdi;
mod progress;

const ACK: u8 = 0x06;

fn run<D: Read + Write>(dev: D, filename: &str) -> Result<usize> {
    let mut file = std::fs::File::open(filename)
        .context(format!("Could not open file '{}' to send", filename))?;
    let blocks = (file.metadata().unwrap().len() + 1023) / 1024;

    // Wrap the device in a snazzy progress bar
    let mut dev = progress::Progress::new(dev, blocks);

    let mut x = xmodem::Xmodem::new();
    x.block_length = xmodem::BlockLength::OneK;
    x.send(&mut dev, &mut file).map_err(|e| anyhow!("{:?}", e))
}

/// Simple XMODEM sender
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Use a dummy backend
    #[clap(short, long)]
    dummy: bool,

    /// Vendor ID of USB device
    #[clap(short, long, conflicts_with = "dummy",
           parse(try_from_str=parse_int::parse))]
    vid: Option<u16>,

    /// Product ID of USB device
    #[clap(short, long, conflicts_with = "dummy",
           parse(try_from_str=parse_int::parse))]
    pid: Option<u16>,

    filename: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    if args.dummy {
        run(dummy::new(), &args.filename)
    } else {
        run(ftdi::new(args.vid, args.pid)?, &args.filename)
    }?;
    Ok(())
}
