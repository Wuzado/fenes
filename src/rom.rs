use std::{
    fs::File,
    io::{BufReader, Bytes, Read},
};

use crate::cpu::instructions::exec::InstructionPair;

pub mod decoder;
//pub mod ines;

#[allow(clippy::upper_case_acronyms)] // No, I don't care about the acronyms.
pub struct ROM {
    rom_file_iter: Bytes<BufReader<File>>,
    instruction_list: Vec<InstructionPair>,
}

impl ROM {
    pub fn new(file: File) -> ROM {
        ROM {
            rom_file_iter: BufReader::new(file).bytes(),
            instruction_list: Vec::new(),
        }
    }

    fn simplified_ines() {}

    pub fn load(mut self) -> std::io::Result<ROM> {
        // Uses a loop instead of a for loop over an iterator due to issues with nesting iterator calls.
        loop {
            match self.rom_file_iter.next() {
                Some(byte) => {
                    let temp = ROM::decode_instr(&mut self, byte?);
                    self.instruction_list.push(temp);
                }
                None => break,
            }
        }

        Ok(self)
    }
}
