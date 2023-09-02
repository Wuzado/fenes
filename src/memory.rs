use crate::cpu::CPU;

pub struct Memory {
    internal_ram: Box<[u8; 0x0800]>, // 2 KB of internal RAM
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            internal_ram: Box::new([0u8; 0x0800]), // Internal memory does not have a reliable state at startup. Opting to zero it out.
        }
    }

    pub fn fetch(&self, addr: u16) -> u8 {
        // Okay, so a little bit of explanation.
        // 6502 or the NES memory map uses addresses from 0x0000 to 0x07FF to address the 2 KB of the internal RAM.
        // Additionally, there are also three mirrors of these addresses, up to 0x2000.
        // What we do here, is we find a remainder of 0x0800 (2 KB) and use it to fetch a value from the memory.
        // Casting to usize is required to index the slice - such an operation should not be lossy.
        // (Unless you're running on an 8 bit address space target, in which case you have much larger issues.)
        // Note that we COULD get the value in a much more concise way using a slice, however a more explicit error is very welcome.
        if addr < 0x2000 {
            return *self.internal_ram.get((addr % 0x0800) as usize).expect("Tried fetching an address larger than 0x0800, despite the address being the remainder of 0x0800.");
        }

        panic!("Tried fetching outside of internal RAM")
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        if addr < 0x2000 {
            let mem_ref = self.internal_ram.get_mut((addr % 0x0800) as usize).expect("Tried writing to an address larger than 0x0800, despite the address being the remainder of 0x0800.");
            *mem_ref = value;
        }

        assert_eq!(self.fetch(addr), value);
        panic!("Tried writing outside of internal RAM");
    }
}
