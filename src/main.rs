use std::io::{Read, Write};

use anyhow::{anyhow, Context, Result};
use clap::Parser;

mod dummy;
mod ftdi;
mod progress;

const ACK: u8 = 0x06;

fn run<D: Read + Write, W: Read>(dev: D, mut stream: W) -> Result<usize> {
    let mut x = xmodem::Xmodem::new();
    x.block_length = xmodem::BlockLength::OneK;
    let mut dev = progress::Progress::new(dev);
    x.send(&mut dev, &mut stream)
        .map_err(|e| anyhow!("{:?}", e))
}

/// Simple XMODEM sender
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Use a dummy backend
    #[clap(short, long)]
    dummy: bool,

    filename: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let file = std::fs::File::open(&args.filename)
        .context(format!("Could not open file '{}' to send", args.filename))?;
    if args.dummy {
        run(dummy::new(), &file)
    } else {
        run(ftdi::new()?, &file)
    }?;
    Ok(())
}
