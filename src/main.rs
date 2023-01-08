use clap::Parser;
use rip8::app::App;
use rip8::cpu::Cpu;
use std::error::Error;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

#[derive(Parser)]
struct Args {
    /// a ROM file
    path: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let rom = read_binary_file(args.path)?;
    let cpu = Cpu::with_rom(&rom);

    App::new(cpu)?.run();
    Ok(())
}

fn read_binary_file<P: AsRef<Path>>(path: P) -> io::Result<Vec<u8>> {
    let mut file = File::open(path)?;
    let mut buf = Vec::new();

    file.read_to_end(&mut buf)?;

    Ok(buf)
}
