use crate::memory::Memory;

pub mod instructions;

/// Processor status decoded:
///
/// NVss DIZC
///
/// Negative, Overflow, (B flag #2, B flag #1), Decimal, Interrupt Disable, Zero, Carry
#[derive(Clone, Copy)]
pub struct StatusFlags {
    pub carry: bool,
    pub zero: bool,
    pub interrupt_disable: bool,
    pub decimal: bool,
    pub b_flag_4: bool,
    pub b_flag_5: bool,
    pub overflow: bool,
    pub negative: bool,
}

impl From<StatusFlags> for u8 {
    fn from(value: StatusFlags) -> Self {
        (value.carry as u8)
            + (value.zero as u8) * 0b10
            + (value.interrupt_disable as u8) * 0b100
            + (value.decimal as u8) * 0b1000
            + (value.b_flag_4 as u8) * 0b1_0000
            + (value.b_flag_5 as u8) * 0b10_0000
            + (value.overflow as u8) * 0b100_0000
            + (value.negative as u8) * 0b1000_0000
    }
}

pub struct CPURegisters {
    pub accumulator: u8,
    pub index_x: u8,
    pub index_y: u8,
    pub program_counter: u16,
    pub stack_pointer: u8,
    pub status_register: StatusFlags,
}

impl Default for CPURegisters {
    /// Initialize the state of CPU registers at start.
    fn default() -> Self {
        CPURegisters {
            accumulator: 0x00,
            index_x: 0x00,
            index_y: 0x00,
            program_counter: 0x0000,
            stack_pointer: 0xFD,
            status_register: StatusFlags {
                carry: false,
                zero: false,
                interrupt_disable: true,
                decimal: false,
                b_flag_4: true,
                b_flag_5: true,
                overflow: false,
                negative: false,
            },
        }
    }
}

#[allow(clippy::upper_case_acronyms)] // No, I don't care about the acronyms.
pub struct CPU {
    pub registers: CPURegisters,
    pub cycles: usize,
    pub memory: Memory,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            registers: CPURegisters::default(),
            cycles: 0,
            memory: Memory::new(),
        }
    }

    pub fn incr_cycles(&mut self, amount: usize) {
        self.cycles += amount;
    }

    pub fn incr_pc(&mut self, amount: u16) -> &mut CPU {
        self.registers.program_counter += amount;

        self
    }

    /// I am NOT juggling mutable borrows. F--k this.
    pub fn stack_push(&mut self, value: u8) {
        self.memory
            .write(0x0100 + self.registers.stack_pointer as u16, value);

        self.registers.stack_pointer -= 1;
    }

    pub fn stack_pull(&mut self) -> u8 {
        let temp = self
            .memory
            .fetch(0x0100 + self.registers.stack_pointer as u16);

        self.registers.stack_pointer += 1;

        temp
    }
}
