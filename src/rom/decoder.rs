use super::ROM;
use crate::cpu::instructions::exec::InstructionPair;
use crate::cpu::instructions::{AddressingMode, Instruction};

impl ROM {
    /// Should only be used while fetching an unsigned 8-bit value for an instruction.
    /// Will panic when it fails to fetch a new byte due to finishing the file or failing to read the file.
    /// This should generally never happen.
    #[inline]
    fn fetch_u8(&mut self) -> u8 {
        self.rom_file_iter.next().unwrap().unwrap()
    }

    /// Should only be used while fetching a signed 8-bit value for an instruction.
    /// Will panic when it fails to fetch a new byte due to finishing the file or failing to read the file.
    #[inline]
    fn fetch_i8(&mut self) -> i8 {
        self.fetch_u8() as i8
    }

    /// Fetches two bytes using fetch_u8 and combines it using [u16::from_le_bytes].
    /// Should only be used while fetching a 16-bit value for an instruction.
    /// Will panic when it fails to fetch a new byte due to finishing the file or failing to read the file.
    #[inline]
    fn fetch_u16(&mut self) -> u16 {
        u16::from_le_bytes([self.fetch_u8(), self.fetch_u8()])
    }

    /// Originally, here was a complicated decoding function...
    /// But after crossing 150+ lines of mind-boggling complexity,
    /// I've rage quit and decided on a simple match function of 256 values.
    /// There ARE patterns in the opcodes! But they're not worth my mental health.
    /// Additionally, we don't need to deal with panics!
    /// Also, we can still consolide some of the repeating instructions!
    pub fn decode_instr(&mut self, byte: u8) -> InstructionPair {
        match byte {
            0x00 => InstructionPair::new(Instruction::BRK, AddressingMode::Implicit),
            0x01 => InstructionPair::new(
                Instruction::ORA,
                AddressingMode::IndexedIndirect(self.fetch_u8()),
            ),
            0x02 => InstructionPair::new(Instruction::NOP, AddressingMode::Implicit), // STP
            0x03 => InstructionPair::new(
                Instruction::SLO,
                AddressingMode::IndexedIndirect(self.fetch_u8()),
            ),
            0x04 => {
                InstructionPair::new(Instruction::NOP, AddressingMode::ZeroPage(self.fetch_u8()))
            }
            0x05 => {
                InstructionPair::new(Instruction::ORA, AddressingMode::ZeroPage(self.fetch_u8()))
            }
            0x06 => {
                InstructionPair::new(Instruction::ASL, AddressingMode::ZeroPage(self.fetch_u8()))
            }
            0x07 => {
                InstructionPair::new(Instruction::SLO, AddressingMode::ZeroPage(self.fetch_u8()))
            }
            0x08 => InstructionPair::new(Instruction::PHP, AddressingMode::Implicit),
            0x09 => {
                InstructionPair::new(Instruction::ORA, AddressingMode::Immediate(self.fetch_u8()))
            }
            0x0A => InstructionPair::new(Instruction::ASL, AddressingMode::Implicit),
            0x0B => {
                InstructionPair::new(Instruction::ANC, AddressingMode::Immediate(self.fetch_u8()))
            }
            0x0C => {
                InstructionPair::new(Instruction::NOP, AddressingMode::Absolute(self.fetch_u16()))
            }
            0x0D => {
                InstructionPair::new(Instruction::ORA, AddressingMode::Absolute(self.fetch_u16()))
            }
            0x0E => {
                InstructionPair::new(Instruction::ASL, AddressingMode::Absolute(self.fetch_u16()))
            }
            0x0F => {
                InstructionPair::new(Instruction::SLO, AddressingMode::Absolute(self.fetch_u16()))
            }
            0x10 => {
                InstructionPair::new(Instruction::BPL, AddressingMode::Relative(self.fetch_i8()))
            }
            0x11 => InstructionPair::new(
                Instruction::ORA,
                AddressingMode::IndirectIndexed(self.fetch_u8()),
            ),
            0x12 => InstructionPair::new(Instruction::NOP, AddressingMode::Implicit), // STP
            0x13 => InstructionPair::new(
                Instruction::SLO,
                AddressingMode::IndirectIndexed(self.fetch_u8()),
            ),
            0x14 => InstructionPair::new(
                Instruction::NOP,
                AddressingMode::ZeroPageIndexedX(self.fetch_u8()),
            ),
            0x15 => InstructionPair::new(
                Instruction::ORA,
                AddressingMode::ZeroPageIndexedX(self.fetch_u8()),
            ),
            0x16 => InstructionPair::new(
                Instruction::ASL,
                AddressingMode::ZeroPageIndexedX(self.fetch_u8()),
            ),
            0x17 => InstructionPair::new(
                Instruction::SLO,
                AddressingMode::ZeroPageIndexedX(self.fetch_u8()),
            ),
            0x18 => InstructionPair::new(Instruction::CLC, AddressingMode::Implicit),
            0x19 => InstructionPair::new(
                Instruction::ORA,
                AddressingMode::AbsoluteIndexedY(self.fetch_u16()),
            ),
            0x1A => InstructionPair::new(Instruction::NOP, AddressingMode::Implicit),
            0x1B => InstructionPair::new(
                Instruction::SLO,
                AddressingMode::AbsoluteIndexedY(self.fetch_u16()),
            ),
            0x1C => InstructionPair::new(
                Instruction::NOP,
                AddressingMode::AbsoluteIndexedX(self.fetch_u16()),
            ),
            0x1D => InstructionPair::new(
                Instruction::ORA,
                AddressingMode::AbsoluteIndexedX(self.fetch_u16()),
            ),
            0x1E => InstructionPair::new(
                Instruction::ASL,
                AddressingMode::AbsoluteIndexedX(self.fetch_u16()),
            ),
            0x1F => InstructionPair::new(
                Instruction::SLO,
                AddressingMode::AbsoluteIndexedX(self.fetch_u16()),
            ),
            0x20 => {
                InstructionPair::new(Instruction::JSR, AddressingMode::Absolute(self.fetch_u16()))
            }
            0x21 => InstructionPair::new(
                Instruction::AND,
                AddressingMode::IndexedIndirect(self.fetch_u8()),
            ),
            0x22 => InstructionPair::new(Instruction::NOP, AddressingMode::Implicit), // STP
            0x23 => InstructionPair::new(
                Instruction::RLA,
                AddressingMode::IndexedIndirect(self.fetch_u8()),
            ),
            0x24 => {
                InstructionPair::new(Instruction::BIT, AddressingMode::ZeroPage(self.fetch_u8()))
            }
            0x25 => {
                InstructionPair::new(Instruction::AND, AddressingMode::ZeroPage(self.fetch_u8()))
            }
            0x26 => {
                InstructionPair::new(Instruction::ROL, AddressingMode::ZeroPage(self.fetch_u8()))
            }
            0x27 => {
                InstructionPair::new(Instruction::RLA, AddressingMode::ZeroPage(self.fetch_u8()))
            }
            0x28 => InstructionPair::new(Instruction::PLP, AddressingMode::Implicit),
            0x29 => {
                InstructionPair::new(Instruction::AND, AddressingMode::Immediate(self.fetch_u8()))
            }
            0x2A => InstructionPair::new(Instruction::ROL, AddressingMode::Implicit),
            0x2B => {
                InstructionPair::new(Instruction::ANC, AddressingMode::Immediate(self.fetch_u8()))
            }
            0x2C => {
                InstructionPair::new(Instruction::BIT, AddressingMode::Absolute(self.fetch_u16()))
            }
            0x2D => {
                InstructionPair::new(Instruction::AND, AddressingMode::Absolute(self.fetch_u16()))
            }
            0x2E => {
                InstructionPair::new(Instruction::ROL, AddressingMode::Absolute(self.fetch_u16()))
            }
            0x2F => {
                InstructionPair::new(Instruction::RLA, AddressingMode::Absolute(self.fetch_u16()))
            }
            0x30 => {
                InstructionPair::new(Instruction::BMI, AddressingMode::Relative(self.fetch_i8()))
            }
            0x31 => InstructionPair::new(
                Instruction::AND,
                AddressingMode::IndirectIndexed(self.fetch_u8()),
            ),
            0x32 => InstructionPair::new(Instruction::NOP, AddressingMode::Implicit), // STP
            0x33 => InstructionPair::new(
                Instruction::RLA,
                AddressingMode::IndirectIndexed(self.fetch_u8()),
            ),
            0x34 => InstructionPair::new(
                Instruction::NOP,
                AddressingMode::ZeroPageIndexedX(self.fetch_u8()),
            ),
            0x35 => InstructionPair::new(
                Instruction::AND,
                AddressingMode::ZeroPageIndexedX(self.fetch_u8()),
            ),
            0x36 => InstructionPair::new(
                Instruction::ROL,
                AddressingMode::ZeroPageIndexedX(self.fetch_u8()),
            ),
            0x37 => InstructionPair::new(
                Instruction::RLA,
                AddressingMode::ZeroPageIndexedX(self.fetch_u8()),
            ),
            0x38 => InstructionPair::new(Instruction::SEC, AddressingMode::Implicit),
            0x39 => InstructionPair::new(
                Instruction::AND,
                AddressingMode::AbsoluteIndexedY(self.fetch_u16()),
            ),
            0x3A => InstructionPair::new(Instruction::NOP, AddressingMode::Implicit),
            0x3B => InstructionPair::new(
                Instruction::RLA,
                AddressingMode::AbsoluteIndexedY(self.fetch_u16()),
            ),
            0x3C => InstructionPair::new(
                Instruction::NOP,
                AddressingMode::AbsoluteIndexedX(self.fetch_u16()),
            ),
            0x3D => InstructionPair::new(
                Instruction::AND,
                AddressingMode::AbsoluteIndexedX(self.fetch_u16()),
            ),
            0x3E => InstructionPair::new(
                Instruction::ROL,
                AddressingMode::AbsoluteIndexedX(self.fetch_u16()),
            ),
            0x3F => InstructionPair::new(
                Instruction::RLA,
                AddressingMode::AbsoluteIndexedX(self.fetch_u16()),
            ),
            0x40 => InstructionPair::new(Instruction::RTI, AddressingMode::Implicit),
            0x41 => InstructionPair::new(
                Instruction::EOR,
                AddressingMode::IndexedIndirect(self.fetch_u8()),
            ),
            0x42 => InstructionPair::new(Instruction::NOP, AddressingMode::Implicit), // STP
            0x43 => InstructionPair::new(
                Instruction::SRE,
                AddressingMode::IndexedIndirect(self.fetch_u8()),
            ),
            0x44 => {
                InstructionPair::new(Instruction::NOP, AddressingMode::ZeroPage(self.fetch_u8()))
            }
            0x45 => {
                InstructionPair::new(Instruction::EOR, AddressingMode::ZeroPage(self.fetch_u8()))
            }
            0x46 => {
                InstructionPair::new(Instruction::LSR, AddressingMode::ZeroPage(self.fetch_u8()))
            }
            0x47 => {
                InstructionPair::new(Instruction::SRE, AddressingMode::ZeroPage(self.fetch_u8()))
            }
            0x48 => InstructionPair::new(Instruction::PHA, AddressingMode::Implicit),
            0x49 => {
                InstructionPair::new(Instruction::EOR, AddressingMode::Immediate(self.fetch_u8()))
            }
            0x4A => InstructionPair::new(Instruction::LSR, AddressingMode::Implicit),
            0x4B => {
                InstructionPair::new(Instruction::ALR, AddressingMode::Immediate(self.fetch_u8()))
            }
            0x4C => {
                InstructionPair::new(Instruction::JMP, AddressingMode::Absolute(self.fetch_u16()))
            }
            0x4D => {
                InstructionPair::new(Instruction::EOR, AddressingMode::Absolute(self.fetch_u16()))
            }
            0x4E => {
                InstructionPair::new(Instruction::LSR, AddressingMode::Absolute(self.fetch_u16()))
            }
            0x4F => {
                InstructionPair::new(Instruction::SRE, AddressingMode::Absolute(self.fetch_u16()))
            }
            0x50 => {
                InstructionPair::new(Instruction::BVC, AddressingMode::Relative(self.fetch_i8()))
            }
            0x51 => InstructionPair::new(
                Instruction::EOR,
                AddressingMode::IndirectIndexed(self.fetch_u8()),
            ),
            0x52 => InstructionPair::new(Instruction::NOP, AddressingMode::Implicit), // STP
            0x53 => InstructionPair::new(
                Instruction::SRE,
                AddressingMode::IndirectIndexed(self.fetch_u8()),
            ),
            0x54 => InstructionPair::new(
                Instruction::NOP,
                AddressingMode::ZeroPageIndexedX(self.fetch_u8()),
            ),
            0x55 => InstructionPair::new(
                Instruction::EOR,
                AddressingMode::ZeroPageIndexedX(self.fetch_u8()),
            ),
            0x56 => InstructionPair::new(
                Instruction::LSR,
                AddressingMode::ZeroPageIndexedX(self.fetch_u8()),
            ),
            0x57 => InstructionPair::new(
                Instruction::SRE,
                AddressingMode::ZeroPageIndexedX(self.fetch_u8()),
            ),
            0x58 => InstructionPair::new(Instruction::CLI, AddressingMode::Implicit),
            0x59 => InstructionPair::new(
                Instruction::EOR,
                AddressingMode::AbsoluteIndexedY(self.fetch_u16()),
            ),
            0x5A => InstructionPair::new(Instruction::NOP, AddressingMode::Implicit),
            0x5B => InstructionPair::new(
                Instruction::SRE,
                AddressingMode::AbsoluteIndexedY(self.fetch_u16()),
            ),
            0x5C => InstructionPair::new(
                Instruction::NOP,
                AddressingMode::AbsoluteIndexedX(self.fetch_u16()),
            ),
            0x5D => InstructionPair::new(
                Instruction::EOR,
                AddressingMode::AbsoluteIndexedX(self.fetch_u16()),
            ),
            0x5E => InstructionPair::new(
                Instruction::LSR,
                AddressingMode::AbsoluteIndexedX(self.fetch_u16()),
            ),
            0x5F => InstructionPair::new(
                Instruction::SRE,
                AddressingMode::AbsoluteIndexedX(self.fetch_u16()),
            ),
            0x60 => InstructionPair::new(Instruction::RTS, AddressingMode::Implicit),
            0x61 => InstructionPair::new(
                Instruction::ADC,
                AddressingMode::IndexedIndirect(self.fetch_u8()),
            ),
            0x62 => InstructionPair::new(Instruction::NOP, AddressingMode::Implicit), // STP
            0x63 => InstructionPair::new(
                Instruction::RRA,
                AddressingMode::IndexedIndirect(self.fetch_u8()),
            ),
            0x64 => {
                InstructionPair::new(Instruction::NOP, AddressingMode::ZeroPage(self.fetch_u8()))
            }
            0x65 => {
                InstructionPair::new(Instruction::ADC, AddressingMode::ZeroPage(self.fetch_u8()))
            }
            0x66 => {
                InstructionPair::new(Instruction::ROR, AddressingMode::ZeroPage(self.fetch_u8()))
            }
            0x67 => {
                InstructionPair::new(Instruction::RRA, AddressingMode::ZeroPage(self.fetch_u8()))
            }
            0x68 => InstructionPair::new(Instruction::PLA, AddressingMode::Implicit),
            0x69 => {
                InstructionPair::new(Instruction::ADC, AddressingMode::Immediate(self.fetch_u8()))
            }
            0x6A => InstructionPair::new(Instruction::ROR, AddressingMode::Implicit),
            0x6B => {
                InstructionPair::new(Instruction::ARR, AddressingMode::Immediate(self.fetch_u8()))
            }
            0x6C => {
                InstructionPair::new(Instruction::JMP, AddressingMode::Indirect(self.fetch_u16()))
            }
            0x6D => {
                InstructionPair::new(Instruction::ADC, AddressingMode::Absolute(self.fetch_u16()))
            }
            0x6E => {
                InstructionPair::new(Instruction::ROR, AddressingMode::Absolute(self.fetch_u16()))
            }
            0x6F => {
                InstructionPair::new(Instruction::RRA, AddressingMode::Absolute(self.fetch_u16()))
            }
            0x70 => {
                InstructionPair::new(Instruction::BVS, AddressingMode::Relative(self.fetch_i8()))
            }
            0x71 => InstructionPair::new(
                Instruction::ADC,
                AddressingMode::IndirectIndexed(self.fetch_u8()),
            ),
            0x72 => InstructionPair::new(Instruction::NOP, AddressingMode::Implicit), // STP
            0x73 => InstructionPair::new(
                Instruction::RRA,
                AddressingMode::IndirectIndexed(self.fetch_u8()),
            ),
            0x74 => InstructionPair::new(
                Instruction::NOP,
                AddressingMode::ZeroPageIndexedX(self.fetch_u8()),
            ),
            0x75 => InstructionPair::new(
                Instruction::ADC,
                AddressingMode::ZeroPageIndexedX(self.fetch_u8()),
            ),
            0x76 => InstructionPair::new(
                Instruction::ROR,
                AddressingMode::ZeroPageIndexedX(self.fetch_u8()),
            ),
            0x77 => InstructionPair::new(
                Instruction::RRA,
                AddressingMode::ZeroPageIndexedX(self.fetch_u8()),
            ),
            0x78 => InstructionPair::new(Instruction::SEI, AddressingMode::Implicit),
            0x79 => InstructionPair::new(
                Instruction::ADC,
                AddressingMode::AbsoluteIndexedY(self.fetch_u16()),
            ),
            0x7A => InstructionPair::new(Instruction::NOP, AddressingMode::Implicit),
            0x7B => InstructionPair::new(
                Instruction::RRA,
                AddressingMode::AbsoluteIndexedY(self.fetch_u16()),
            ),
            0x7C => InstructionPair::new(
                Instruction::NOP,
                AddressingMode::AbsoluteIndexedX(self.fetch_u16()),
            ),
            0x7D => InstructionPair::new(
                Instruction::ADC,
                AddressingMode::AbsoluteIndexedX(self.fetch_u16()),
            ),
            0x7E => InstructionPair::new(
                Instruction::ROR,
                AddressingMode::AbsoluteIndexedX(self.fetch_u16()),
            ),
            0x7F => InstructionPair::new(
                Instruction::RRA,
                AddressingMode::AbsoluteIndexedX(self.fetch_u16()),
            ),
            0x80 => {
                InstructionPair::new(Instruction::NOP, AddressingMode::Immediate(self.fetch_u8()))
            }
            0x81 => InstructionPair::new(
                Instruction::STA,
                AddressingMode::IndexedIndirect(self.fetch_u8()),
            ),
            0x82 => {
                InstructionPair::new(Instruction::NOP, AddressingMode::Immediate(self.fetch_u8()))
            }
            0x83 => InstructionPair::new(
                Instruction::SAX,
                AddressingMode::IndexedIndirect(self.fetch_u8()),
            ),
            0x84 => {
                InstructionPair::new(Instruction::STY, AddressingMode::ZeroPage(self.fetch_u8()))
            }
            0x85 => {
                InstructionPair::new(Instruction::STA, AddressingMode::ZeroPage(self.fetch_u8()))
            }
            0x86 => {
                InstructionPair::new(Instruction::STX, AddressingMode::ZeroPage(self.fetch_u8()))
            }
            0x87 => {
                InstructionPair::new(Instruction::SAX, AddressingMode::ZeroPage(self.fetch_u8()))
            }
            0x88 => InstructionPair::new(Instruction::DEY, AddressingMode::Implicit),
            0x89 => {
                InstructionPair::new(Instruction::NOP, AddressingMode::Immediate(self.fetch_u8()))
            }
            0x8A => InstructionPair::new(Instruction::TXA, AddressingMode::Implicit),
            0x8B => {
                InstructionPair::new(Instruction::XAA, AddressingMode::Immediate(self.fetch_u8()))
            }
            0x8C => {
                InstructionPair::new(Instruction::STY, AddressingMode::Absolute(self.fetch_u16()))
            }
            0x8D => {
                InstructionPair::new(Instruction::STA, AddressingMode::Absolute(self.fetch_u16()))
            }
            0x8E => {
                InstructionPair::new(Instruction::STX, AddressingMode::Absolute(self.fetch_u16()))
            }
            0x8F => {
                InstructionPair::new(Instruction::SAX, AddressingMode::Absolute(self.fetch_u16()))
            }
            0x90 => {
                InstructionPair::new(Instruction::BCC, AddressingMode::Relative(self.fetch_i8()))
            }
            0x91 => InstructionPair::new(
                Instruction::STA,
                AddressingMode::IndirectIndexed(self.fetch_u8()),
            ),
            0x92 => InstructionPair::new(Instruction::NOP, AddressingMode::Implicit), // STP
            0x93 => InstructionPair::new(
                Instruction::AHX,
                AddressingMode::IndirectIndexed(self.fetch_u8()),
            ),
            0x94 => InstructionPair::new(
                Instruction::STY,
                AddressingMode::ZeroPageIndexedX(self.fetch_u8()),
            ),
            0x95 => InstructionPair::new(
                Instruction::STA,
                AddressingMode::ZeroPageIndexedX(self.fetch_u8()),
            ),
            0x96 => InstructionPair::new(
                Instruction::STX,
                AddressingMode::ZeroPageIndexedY(self.fetch_u8()),
            ),
            0x97 => InstructionPair::new(
                Instruction::SAX,
                AddressingMode::ZeroPageIndexedY(self.fetch_u8()),
            ),
            0x98 => InstructionPair::new(Instruction::TYA, AddressingMode::Implicit),
            0x99 => InstructionPair::new(
                Instruction::STA,
                AddressingMode::AbsoluteIndexedY(self.fetch_u16()),
            ),
            0x9A => InstructionPair::new(Instruction::TXS, AddressingMode::Implicit),
            0x9B => InstructionPair::new(
                Instruction::TAS,
                AddressingMode::AbsoluteIndexedY(self.fetch_u16()),
            ),
            0x9C => InstructionPair::new(
                Instruction::SHY,
                AddressingMode::AbsoluteIndexedX(self.fetch_u16()),
            ),
            0x9D => InstructionPair::new(
                Instruction::STA,
                AddressingMode::AbsoluteIndexedX(self.fetch_u16()),
            ),
            0x9E => InstructionPair::new(
                Instruction::SHX,
                AddressingMode::AbsoluteIndexedY(self.fetch_u16()),
            ),
            0x9F => InstructionPair::new(
                Instruction::AHX,
                AddressingMode::AbsoluteIndexedY(self.fetch_u16()),
            ),
            0xA0 => {
                InstructionPair::new(Instruction::LDY, AddressingMode::Immediate(self.fetch_u8()))
            }
            0xA1 => InstructionPair::new(
                Instruction::LDA,
                AddressingMode::IndexedIndirect(self.fetch_u8()),
            ),
            0xA2 => {
                InstructionPair::new(Instruction::LDX, AddressingMode::Immediate(self.fetch_u8()))
            }
            0xA3 => InstructionPair::new(
                Instruction::LAX,
                AddressingMode::IndexedIndirect(self.fetch_u8()),
            ),
            0xA4 => {
                InstructionPair::new(Instruction::LDY, AddressingMode::ZeroPage(self.fetch_u8()))
            }
            0xA5 => {
                InstructionPair::new(Instruction::LDA, AddressingMode::ZeroPage(self.fetch_u8()))
            }
            0xA6 => {
                InstructionPair::new(Instruction::LDX, AddressingMode::ZeroPage(self.fetch_u8()))
            }
            0xA7 => {
                InstructionPair::new(Instruction::LAX, AddressingMode::ZeroPage(self.fetch_u8()))
            }
            0xA8 => InstructionPair::new(Instruction::TAY, AddressingMode::Implicit),
            0xA9 => {
                InstructionPair::new(Instruction::LDA, AddressingMode::Immediate(self.fetch_u8()))
            }
            0xAA => InstructionPair::new(Instruction::TAX, AddressingMode::Implicit),
            0xAB => {
                InstructionPair::new(Instruction::LAX, AddressingMode::Immediate(self.fetch_u8()))
            }
            0xAC => {
                InstructionPair::new(Instruction::LDY, AddressingMode::Absolute(self.fetch_u16()))
            }
            0xAD => {
                InstructionPair::new(Instruction::LDA, AddressingMode::Absolute(self.fetch_u16()))
            }
            0xAE => {
                InstructionPair::new(Instruction::LDX, AddressingMode::Absolute(self.fetch_u16()))
            }
            0xAF => {
                InstructionPair::new(Instruction::LAX, AddressingMode::Absolute(self.fetch_u16()))
            }
            0xB0 => {
                InstructionPair::new(Instruction::BCS, AddressingMode::Relative(self.fetch_i8()))
            }
            0xB1 => InstructionPair::new(
                Instruction::LDA,
                AddressingMode::IndirectIndexed(self.fetch_u8()),
            ),
            0xB2 => InstructionPair::new(Instruction::NOP, AddressingMode::Implicit), // STP
            0xB3 => InstructionPair::new(
                Instruction::LAX,
                AddressingMode::IndirectIndexed(self.fetch_u8()),
            ),
            0xB4 => InstructionPair::new(
                Instruction::LDY,
                AddressingMode::ZeroPageIndexedX(self.fetch_u8()),
            ),
            0xB5 => InstructionPair::new(
                Instruction::LDA,
                AddressingMode::ZeroPageIndexedX(self.fetch_u8()),
            ),
            0xB6 => InstructionPair::new(
                Instruction::LDX,
                AddressingMode::ZeroPageIndexedY(self.fetch_u8()),
            ),
            0xB7 => InstructionPair::new(
                Instruction::LAX,
                AddressingMode::ZeroPageIndexedY(self.fetch_u8()),
            ),
            0xB8 => InstructionPair::new(Instruction::CLV, AddressingMode::Implicit),
            0xB9 => InstructionPair::new(
                Instruction::LDA,
                AddressingMode::AbsoluteIndexedY(self.fetch_u16()),
            ),
            0xBA => InstructionPair::new(Instruction::TSX, AddressingMode::Implicit),
            0xBB => InstructionPair::new(
                Instruction::LAS,
                AddressingMode::AbsoluteIndexedY(self.fetch_u16()),
            ),
            0xBC => InstructionPair::new(
                Instruction::LDY,
                AddressingMode::AbsoluteIndexedX(self.fetch_u16()),
            ),
            0xBD => InstructionPair::new(
                Instruction::LDA,
                AddressingMode::AbsoluteIndexedX(self.fetch_u16()),
            ),
            0xBE => InstructionPair::new(
                Instruction::LDX,
                AddressingMode::AbsoluteIndexedY(self.fetch_u16()),
            ),
            0xBF => InstructionPair::new(
                Instruction::LAX,
                AddressingMode::AbsoluteIndexedY(self.fetch_u16()),
            ),
            0xC0 => {
                InstructionPair::new(Instruction::CPY, AddressingMode::Immediate(self.fetch_u8()))
            }
            0xC1 => InstructionPair::new(
                Instruction::CMP,
                AddressingMode::IndexedIndirect(self.fetch_u8()),
            ),
            0xC2 => {
                InstructionPair::new(Instruction::NOP, AddressingMode::Immediate(self.fetch_u8()))
            }
            0xC3 => InstructionPair::new(
                Instruction::DCP,
                AddressingMode::IndexedIndirect(self.fetch_u8()),
            ),
            0xC4 => {
                InstructionPair::new(Instruction::CPY, AddressingMode::ZeroPage(self.fetch_u8()))
            }
            0xC5 => {
                InstructionPair::new(Instruction::CMP, AddressingMode::ZeroPage(self.fetch_u8()))
            }
            0xC6 => {
                InstructionPair::new(Instruction::DEC, AddressingMode::ZeroPage(self.fetch_u8()))
            }
            0xC7 => {
                InstructionPair::new(Instruction::DCP, AddressingMode::ZeroPage(self.fetch_u8()))
            }
            0xC8 => InstructionPair::new(Instruction::INY, AddressingMode::Implicit),
            0xC9 => {
                InstructionPair::new(Instruction::CMP, AddressingMode::Immediate(self.fetch_u8()))
            }
            0xCA => InstructionPair::new(Instruction::DEX, AddressingMode::Implicit),
            0xCB => {
                InstructionPair::new(Instruction::AXS, AddressingMode::Immediate(self.fetch_u8()))
            }
            0xCC => {
                InstructionPair::new(Instruction::CPY, AddressingMode::Absolute(self.fetch_u16()))
            }
            0xCD => {
                InstructionPair::new(Instruction::CMP, AddressingMode::Absolute(self.fetch_u16()))
            }
            0xCE => {
                InstructionPair::new(Instruction::DEC, AddressingMode::Absolute(self.fetch_u16()))
            }
            0xCF => {
                InstructionPair::new(Instruction::DCP, AddressingMode::Absolute(self.fetch_u16()))
            }
            0xD0 => {
                InstructionPair::new(Instruction::BNE, AddressingMode::Relative(self.fetch_i8()))
            }
            0xD1 => InstructionPair::new(
                Instruction::CMP,
                AddressingMode::IndirectIndexed(self.fetch_u8()),
            ),
            0xD2 => InstructionPair::new(Instruction::NOP, AddressingMode::Implicit), // STP
            0xD3 => InstructionPair::new(
                Instruction::DCP,
                AddressingMode::IndirectIndexed(self.fetch_u8()),
            ),
            0xD4 => InstructionPair::new(
                Instruction::NOP,
                AddressingMode::ZeroPageIndexedX(self.fetch_u8()),
            ),
            0xD5 => InstructionPair::new(
                Instruction::CMP,
                AddressingMode::ZeroPageIndexedX(self.fetch_u8()),
            ),
            0xD6 => InstructionPair::new(
                Instruction::DEC,
                AddressingMode::ZeroPageIndexedX(self.fetch_u8()),
            ),
            0xD7 => InstructionPair::new(
                Instruction::DCP,
                AddressingMode::ZeroPageIndexedX(self.fetch_u8()),
            ),
            0xD8 => InstructionPair::new(Instruction::CLD, AddressingMode::Implicit),
            0xD9 => InstructionPair::new(
                Instruction::CMP,
                AddressingMode::AbsoluteIndexedY(self.fetch_u16()),
            ),
            0xDA => InstructionPair::new(Instruction::NOP, AddressingMode::Implicit),
            0xDB => InstructionPair::new(
                Instruction::DCP,
                AddressingMode::AbsoluteIndexedY(self.fetch_u16()),
            ),
            0xDC => InstructionPair::new(
                Instruction::NOP,
                AddressingMode::AbsoluteIndexedX(self.fetch_u16()),
            ),
            0xDD => InstructionPair::new(
                Instruction::CMP,
                AddressingMode::AbsoluteIndexedX(self.fetch_u16()),
            ),
            0xDE => InstructionPair::new(
                Instruction::DEC,
                AddressingMode::AbsoluteIndexedX(self.fetch_u16()),
            ),
            0xDF => InstructionPair::new(
                Instruction::DCP,
                AddressingMode::AbsoluteIndexedX(self.fetch_u16()),
            ),
            0xE0 => {
                InstructionPair::new(Instruction::CPX, AddressingMode::Immediate(self.fetch_u8()))
            }
            0xE1 => InstructionPair::new(
                Instruction::SBC,
                AddressingMode::IndexedIndirect(self.fetch_u8()),
            ),
            0xE2 => {
                InstructionPair::new(Instruction::NOP, AddressingMode::Immediate(self.fetch_u8()))
            }
            0xE3 => InstructionPair::new(
                Instruction::ISC,
                AddressingMode::IndexedIndirect(self.fetch_u8()),
            ),
            0xE4 => {
                InstructionPair::new(Instruction::CPX, AddressingMode::ZeroPage(self.fetch_u8()))
            }
            0xE5 => {
                InstructionPair::new(Instruction::SBC, AddressingMode::ZeroPage(self.fetch_u8()))
            }
            0xE6 => {
                InstructionPair::new(Instruction::INC, AddressingMode::ZeroPage(self.fetch_u8()))
            }
            0xE7 => {
                InstructionPair::new(Instruction::ISC, AddressingMode::ZeroPage(self.fetch_u8()))
            }
            0xE8 => InstructionPair::new(Instruction::INX, AddressingMode::Implicit),
            0xE9 => {
                InstructionPair::new(Instruction::SBC, AddressingMode::Immediate(self.fetch_u8()))
            }
            0xEA => InstructionPair::new(Instruction::NOP, AddressingMode::Implicit),
            0xEB => {
                InstructionPair::new(Instruction::SBC, AddressingMode::Immediate(self.fetch_u8()))
            }
            0xEC => {
                InstructionPair::new(Instruction::CPX, AddressingMode::Absolute(self.fetch_u16()))
            }
            0xED => {
                InstructionPair::new(Instruction::SBC, AddressingMode::Absolute(self.fetch_u16()))
            }
            0xEE => {
                InstructionPair::new(Instruction::INC, AddressingMode::Absolute(self.fetch_u16()))
            }
            0xEF => {
                InstructionPair::new(Instruction::ISC, AddressingMode::Absolute(self.fetch_u16()))
            }
            0xF0 => {
                InstructionPair::new(Instruction::BEQ, AddressingMode::Relative(self.fetch_i8()))
            }
            0xF1 => InstructionPair::new(
                Instruction::SBC,
                AddressingMode::IndirectIndexed(self.fetch_u8()),
            ),
            0xF2 => InstructionPair::new(Instruction::NOP, AddressingMode::Implicit), // STP
            0xF3 => InstructionPair::new(
                Instruction::ISC,
                AddressingMode::IndirectIndexed(self.fetch_u8()),
            ),
            0xF4 => InstructionPair::new(
                Instruction::NOP,
                AddressingMode::ZeroPageIndexedX(self.fetch_u8()),
            ),
            0xF5 => InstructionPair::new(
                Instruction::SBC,
                AddressingMode::ZeroPageIndexedX(self.fetch_u8()),
            ),
            0xF6 => InstructionPair::new(
                Instruction::INC,
                AddressingMode::ZeroPageIndexedX(self.fetch_u8()),
            ),
            0xF7 => InstructionPair::new(
                Instruction::ISC,
                AddressingMode::ZeroPageIndexedX(self.fetch_u8()),
            ),
            0xF8 => InstructionPair::new(Instruction::SED, AddressingMode::Implicit),
            0xF9 => InstructionPair::new(
                Instruction::SBC,
                AddressingMode::AbsoluteIndexedY(self.fetch_u16()),
            ),
            0xFA => InstructionPair::new(Instruction::NOP, AddressingMode::Implicit),
            0xFB => InstructionPair::new(
                Instruction::ISC,
                AddressingMode::AbsoluteIndexedY(self.fetch_u16()),
            ),
            0xFC => InstructionPair::new(
                Instruction::NOP,
                AddressingMode::AbsoluteIndexedX(self.fetch_u16()),
            ),
            0xFD => InstructionPair::new(
                Instruction::SBC,
                AddressingMode::AbsoluteIndexedX(self.fetch_u16()),
            ),
            0xFE => InstructionPair::new(
                Instruction::INC,
                AddressingMode::AbsoluteIndexedX(self.fetch_u16()),
            ),
            0xFF => InstructionPair::new(
                Instruction::ISC,
                AddressingMode::AbsoluteIndexedX(self.fetch_u16()),
            ),
        }
    }
}
