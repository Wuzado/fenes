mod cpu;
mod memory;
mod rom;
mod utils;

use std::fs::File;

fn main() -> std::io::Result<()> {
    let rom = rom::ROM::new(File::open("rom")?).load()?;
    let mut CPU = cpu::CPU::new();

    Ok(())
}
