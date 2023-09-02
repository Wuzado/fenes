use super::*;
use crate::cpu::CPU;

#[derive(Debug)]
pub struct InstructionPair {
    instruction: Instruction,
    addr_mode: AddressingMode,
}

type FetchVal = u8;
type Addr = u16;
type Cycles = usize;

/// CAUTION: Use cycles only when the base cycle value is equal to 4. (All that require checking page boundaries.)
fn absolute_indexed(value: u16, index: u8) -> (Addr, Cycles) {
    let old_hi_nibble = value >> 8;
    let addr = value + (index as u16);
    let mut cycles: usize = 4;

    if (addr >> 8) != old_hi_nibble {
        // Page crossed.
        cycles += 1;
    }

    (addr, cycles)
}

#[inline]
fn indexedindirect(cpu: &CPU, value: u8) -> FetchVal {
    cpu.memory.fetch(
        cpu.memory.fetch((value + cpu.registers.index_x) as u16) as u16
            + (cpu.memory.fetch((value + cpu.registers.index_x + 1) as u16) << 8) as u16,
    )
}

#[inline]
/// CAUTION: Do not use cycles with STA (Store Accumulator), it's always 6 cycles.
fn indirectindexed(cpu: &CPU, value: u8) -> (FetchVal, Cycles) {
    let addr = cpu.memory.fetch(value as u16) as u16
        + (cpu.memory.fetch((value + 1) as u16) << 8) as u16
        + cpu.registers.index_y as u16;

    let mut cycles = 5;

    if (addr << 8) != 0 {
        cycles += 1;
    }

    (cpu.memory.fetch(addr), cycles)
}

impl InstructionPair {
    pub fn new(instruction: Instruction, addr_mode: AddressingMode) -> InstructionPair {
        InstructionPair {
            instruction,
            addr_mode,
        }
    }

    pub fn exec(&self, cpu: &mut CPU) {
        let (instruction, addr_mode) = (&self.instruction, &self.addr_mode);

        match (instruction, addr_mode) {
            (Instruction::LDA, AddressingMode::Immediate(value)) => {
                cpu.incr_pc(2);
                cpu.lda(*value, 2);
            }
            (Instruction::LDA, AddressingMode::ZeroPage(value)) => {
                cpu.incr_pc(2);
                cpu.lda(cpu.memory.fetch(*value as u16), 3);
            }
            (Instruction::LDA, AddressingMode::ZeroPageIndexedX(value)) => {
                cpu.incr_pc(2);
                cpu.lda(cpu.memory.fetch((*value + cpu.registers.index_x) as u16), 4);
            }
            (Instruction::LDA, AddressingMode::Absolute(value)) => {
                cpu.incr_pc(3);
                cpu.lda(cpu.memory.fetch(*value), 4);
            }
            (Instruction::LDA, AddressingMode::AbsoluteIndexedX(value)) => {
                let (addr, cycles) = absolute_indexed(*value, cpu.registers.index_x);

                cpu.incr_pc(3);
                cpu.lda(cpu.memory.fetch(addr), cycles);
            }
            (Instruction::LDA, AddressingMode::AbsoluteIndexedY(value)) => {
                let (addr, cycles) = absolute_indexed(*value, cpu.registers.index_y);

                cpu.incr_pc(3);
                cpu.lda(cpu.memory.fetch(addr), cycles);
            }
            (Instruction::LDA, AddressingMode::IndexedIndirect(value)) => {
                cpu.incr_pc(2);
                cpu.lda(indexedindirect(cpu, *value), 6);
            }
            (Instruction::LDA, AddressingMode::IndirectIndexed(value)) => {
                let (val, cycles) = indirectindexed(cpu, *value);

                cpu.incr_pc(2);
                cpu.lda(val, cycles);
            }
            (Instruction::LDX, AddressingMode::Immediate(value)) => {
                cpu.incr_pc(2);
                cpu.ldx(*value, 2);
            }
            (Instruction::LDX, AddressingMode::ZeroPage(value)) => {
                cpu.incr_pc(2);
                cpu.ldx(cpu.memory.fetch(*value as u16), 3);
            }
            (Instruction::LDX, AddressingMode::ZeroPageIndexedY(value)) => {
                cpu.incr_pc(2);
                cpu.ldx(cpu.memory.fetch((*value + cpu.registers.index_y) as u16), 4);
            }
            (Instruction::LDX, AddressingMode::Absolute(value)) => {
                cpu.incr_pc(3);
                cpu.ldx(cpu.memory.fetch(*value), 4);
            }
            (Instruction::LDX, AddressingMode::AbsoluteIndexedY(value)) => {
                let (addr, cycles) = absolute_indexed(*value, cpu.registers.index_y);

                cpu.incr_pc(3);
                cpu.ldx(cpu.memory.fetch(addr), cycles);
            }
            (Instruction::LDY, AddressingMode::Immediate(value)) => {
                cpu.incr_pc(2);
                cpu.ldy(*value, 2);
            }
            (Instruction::LDY, AddressingMode::ZeroPage(value)) => {
                cpu.incr_pc(2);
                cpu.ldy(cpu.memory.fetch(*value as u16), 3);
            }
            (Instruction::LDY, AddressingMode::ZeroPageIndexedX(value)) => {
                cpu.incr_pc(2);
                cpu.ldy(cpu.memory.fetch((*value + cpu.registers.index_x) as u16), 4);
            }
            (Instruction::LDY, AddressingMode::Absolute(value)) => {
                cpu.incr_pc(3);
                cpu.ldy(cpu.memory.fetch(*value), 4);
            }
            (Instruction::LDY, AddressingMode::AbsoluteIndexedX(value)) => {
                let (addr, cycles) = absolute_indexed(*value, cpu.registers.index_x);

                cpu.incr_pc(3);
                cpu.ldy(cpu.memory.fetch(addr), cycles);
            }
            (Instruction::STA, AddressingMode::ZeroPage(value)) => {
                cpu.incr_pc(2);
                cpu.sta(*value as u16, 3);
            }
            (Instruction::STA, AddressingMode::ZeroPageIndexedX(value)) => {
                cpu.incr_pc(2);
                cpu.sta(
                    cpu.memory.fetch((*value + cpu.registers.index_x) as u16) as u16,
                    4,
                );
            }
            (Instruction::STA, AddressingMode::Absolute(value)) => {
                cpu.incr_pc(3);
                cpu.sta(*value, 4);
            }
            (Instruction::STA, AddressingMode::AbsoluteIndexedX(value)) => {
                let (addr, _) = absolute_indexed(*value, cpu.registers.index_x);

                cpu.incr_pc(3);
                cpu.sta(addr, 5);
            }
            (Instruction::STA, AddressingMode::AbsoluteIndexedY(value)) => {
                let (addr, _) = absolute_indexed(*value, cpu.registers.index_x);

                cpu.incr_pc(3);
                cpu.sta(addr, 5);
            }
            (Instruction::STA, AddressingMode::IndexedIndirect(value)) => {
                cpu.incr_pc(2);
                cpu.sta(indexedindirect(cpu, *value) as u16, 6);
            }
            (Instruction::STA, AddressingMode::IndirectIndexed(value)) => {
                let (addr, _) = indirectindexed(cpu, *value);

                cpu.incr_pc(2);
                cpu.sta(addr as u16, 6);
            }
            (Instruction::STX, AddressingMode::ZeroPage(value)) => {
                cpu.incr_pc(2);
                cpu.stx(*value as u16, 3);
            }
            (Instruction::STX, AddressingMode::ZeroPageIndexedY(value)) => {
                cpu.incr_pc(2);
                cpu.stx(
                    cpu.memory.fetch((*value + cpu.registers.index_y) as u16) as u16,
                    4,
                );
            }
            (Instruction::STX, AddressingMode::Absolute(value)) => {
                cpu.incr_pc(3);
                cpu.stx(*value, 4);
            }
            (Instruction::STY, AddressingMode::ZeroPage(value)) => {
                cpu.incr_pc(2);
                cpu.sty(*value as u16, 3);
            }
            (Instruction::STY, AddressingMode::ZeroPageIndexedX(value)) => {
                cpu.incr_pc(2);
                cpu.sty(
                    cpu.memory.fetch((*value + cpu.registers.index_x) as u16) as u16,
                    4,
                );
            }
            (Instruction::STY, AddressingMode::Absolute(value)) => {
                cpu.incr_pc(3);
                cpu.sty(*value, 4);
            }
            (Instruction::TAX, AddressingMode::Implicit) => {
                cpu.incr_pc(1);
                cpu.tax();
            }
            (Instruction::TAY, AddressingMode::Implicit) => {
                cpu.incr_pc(1);
                cpu.tay();
            }
            (Instruction::TXA, AddressingMode::Implicit) => {
                cpu.incr_pc(1);
                cpu.txa();
            }
            (Instruction::TYA, AddressingMode::Implicit) => {
                cpu.incr_pc(1);
                cpu.tya();
            }
            (Instruction::TSX, AddressingMode::Implicit) => {
                cpu.incr_pc(1);
                cpu.tsx();
            }
            (Instruction::TXS, AddressingMode::Implicit) => {
                cpu.incr_pc(1);
                cpu.txs();
            }
            (Instruction::PHA, AddressingMode::Implicit) => {
                cpu.incr_pc(1);
                cpu.pha();
            }
            (Instruction::PHP, AddressingMode::Implicit) => {
                cpu.incr_pc(1);
                cpu.php();
            }
            (Instruction::PLA, AddressingMode::Implicit) => {
                cpu.incr_pc(1);
                cpu.pla();
            }
            (Instruction::PLP, AddressingMode::Implicit) => {
                cpu.incr_pc(1);
                cpu.plp();
            }
            (Instruction::AND, AddressingMode::Immediate(value)) => {
                cpu.incr_pc(2);
                cpu.and(*value, 2);
            }
            (Instruction::AND, AddressingMode::ZeroPage(value)) => {
                cpu.incr_pc(2);
                cpu.and(cpu.memory.fetch(*value as u16), 3);
            }
            (Instruction::AND, AddressingMode::ZeroPageIndexedX(value)) => {
                cpu.incr_pc(2);
                cpu.and(cpu.memory.fetch((*value + cpu.registers.index_x) as u16), 4);
            }
            (Instruction::AND, AddressingMode::Absolute(value)) => {
                cpu.incr_pc(3);
                cpu.and(cpu.memory.fetch(*value), 4);
            }
            (Instruction::AND, AddressingMode::AbsoluteIndexedX(value)) => {
                let (addr, cycles) = absolute_indexed(*value, cpu.registers.index_x);

                cpu.incr_pc(3);
                cpu.and(cpu.memory.fetch(addr), cycles);
            }
            (Instruction::AND, AddressingMode::AbsoluteIndexedY(value)) => {
                let (addr, cycles) = absolute_indexed(*value, cpu.registers.index_y);

                cpu.incr_pc(3);
                cpu.and(cpu.memory.fetch(addr), cycles);
            }
            (Instruction::AND, AddressingMode::IndexedIndirect(value)) => {
                cpu.incr_pc(2);
                cpu.and(indexedindirect(cpu, *value), 6);
            }
            (Instruction::AND, AddressingMode::IndirectIndexed(value)) => {
                let (val, cycles) = indirectindexed(cpu, *value);

                cpu.incr_pc(2);
                cpu.and(val, cycles);
            }
            (Instruction::EOR, AddressingMode::Immediate(value)) => {
                cpu.incr_pc(2);
                cpu.eor(*value, 2);
            }
            (Instruction::EOR, AddressingMode::ZeroPage(value)) => {
                cpu.incr_pc(2);
                cpu.eor(cpu.memory.fetch(*value as u16), 3);
            }
            (Instruction::EOR, AddressingMode::ZeroPageIndexedX(value)) => {
                cpu.incr_pc(2);
                cpu.eor(cpu.memory.fetch((*value + cpu.registers.index_x) as u16), 4);
            }
            (Instruction::EOR, AddressingMode::Absolute(value)) => {
                cpu.incr_pc(3);
                cpu.eor(cpu.memory.fetch(*value), 4);
            }
            (Instruction::EOR, AddressingMode::AbsoluteIndexedX(value)) => {
                let (addr, cycles) = absolute_indexed(*value, cpu.registers.index_x);

                cpu.incr_pc(3);
                cpu.eor(cpu.memory.fetch(addr), cycles);
            }
            (Instruction::EOR, AddressingMode::AbsoluteIndexedY(value)) => {
                let (addr, cycles) = absolute_indexed(*value, cpu.registers.index_y);

                cpu.incr_pc(3);
                cpu.eor(cpu.memory.fetch(addr), cycles);
            }
            (Instruction::EOR, AddressingMode::IndexedIndirect(value)) => {
                cpu.incr_pc(2);
                cpu.eor(indexedindirect(cpu, *value), 6);
            }
            (Instruction::EOR, AddressingMode::IndirectIndexed(value)) => {
                let (val, cycles) = indirectindexed(cpu, *value);

                cpu.incr_pc(2);
                cpu.eor(val, cycles);
            }
            (Instruction::ORA, AddressingMode::Immediate(value)) => {
                cpu.incr_pc(2);
                cpu.ora(*value, 2);
            }
            (Instruction::ORA, AddressingMode::ZeroPage(value)) => {
                cpu.incr_pc(2);
                cpu.ora(cpu.memory.fetch(*value as u16), 3);
            }
            (Instruction::ORA, AddressingMode::ZeroPageIndexedX(value)) => {
                cpu.incr_pc(2);
                cpu.ora(cpu.memory.fetch((*value + cpu.registers.index_x) as u16), 4);
            }
            (Instruction::ORA, AddressingMode::Absolute(value)) => {
                cpu.incr_pc(3);
                cpu.ora(cpu.memory.fetch(*value), 4);
            }
            (Instruction::ORA, AddressingMode::AbsoluteIndexedX(value)) => {
                let (addr, cycles) = absolute_indexed(*value, cpu.registers.index_x);

                cpu.incr_pc(3);
                cpu.ora(cpu.memory.fetch(addr), cycles);
            }
            (Instruction::ORA, AddressingMode::AbsoluteIndexedY(value)) => {
                let (addr, cycles) = absolute_indexed(*value, cpu.registers.index_y);

                cpu.incr_pc(3);
                cpu.ora(cpu.memory.fetch(addr), cycles);
            }
            (Instruction::ORA, AddressingMode::IndexedIndirect(value)) => {
                cpu.incr_pc(2);
                cpu.ora(indexedindirect(cpu, *value), 6);
            }
            (Instruction::ORA, AddressingMode::IndirectIndexed(value)) => {
                let (val, cycles) = indirectindexed(cpu, *value);

                cpu.incr_pc(2);
                cpu.ora(val, cycles);
            }
            (Instruction::BIT, AddressingMode::ZeroPage(value)) => {
                cpu.incr_pc(2);
                cpu.bit(cpu.memory.fetch(*value as u16), 3);
            }
            (Instruction::BIT, AddressingMode::Absolute(value)) => {
                cpu.incr_pc(3);
                cpu.bit(cpu.memory.fetch(*value), 4);
            }
            (Instruction::ADC, AddressingMode::Immediate(value)) => {
                cpu.incr_pc(2);
                cpu.adc(*value, 2);
            }
            (Instruction::ADC, AddressingMode::ZeroPage(value)) => {
                cpu.incr_pc(2);
                cpu.adc(cpu.memory.fetch(*value as u16), 3);
            }
            (Instruction::ADC, AddressingMode::ZeroPageIndexedX(value)) => {
                cpu.incr_pc(2);
                cpu.adc(cpu.memory.fetch((*value + cpu.registers.index_x) as u16), 4);
            }
            (Instruction::ADC, AddressingMode::Absolute(value)) => {
                cpu.incr_pc(3);
                cpu.adc(cpu.memory.fetch(*value), 4);
            }
            (Instruction::ADC, AddressingMode::AbsoluteIndexedX(value)) => {
                let (addr, cycles) = absolute_indexed(*value, cpu.registers.index_x);

                cpu.incr_pc(3);
                cpu.adc(cpu.memory.fetch(addr), cycles);
            }
            (Instruction::ADC, AddressingMode::AbsoluteIndexedY(value)) => {
                let (addr, cycles) = absolute_indexed(*value, cpu.registers.index_y);

                cpu.incr_pc(3);
                cpu.adc(cpu.memory.fetch(addr), cycles);
            }
            (Instruction::ADC, AddressingMode::IndexedIndirect(value)) => {
                cpu.incr_pc(2);
                cpu.adc(indexedindirect(cpu, *value), 6);
            }
            (Instruction::ADC, AddressingMode::IndirectIndexed(value)) => {
                let (val, cycles) = indirectindexed(cpu, *value);

                cpu.incr_pc(2);
                cpu.adc(val, cycles);
            }
            (Instruction::SBC, AddressingMode::Immediate(value)) => {
                cpu.incr_pc(2);
                cpu.sbc(*value, 2);
            }
            (Instruction::SBC, AddressingMode::ZeroPage(value)) => {
                cpu.incr_pc(2);
                cpu.sbc(cpu.memory.fetch(*value as u16), 3);
            }
            (Instruction::SBC, AddressingMode::ZeroPageIndexedX(value)) => {
                cpu.incr_pc(2);
                cpu.sbc(cpu.memory.fetch((*value + cpu.registers.index_x) as u16), 4);
            }
            (Instruction::SBC, AddressingMode::Absolute(value)) => {
                cpu.incr_pc(3);
                cpu.sbc(cpu.memory.fetch(*value), 4);
            }
            (Instruction::SBC, AddressingMode::AbsoluteIndexedX(value)) => {
                let (addr, cycles) = absolute_indexed(*value, cpu.registers.index_x);

                cpu.incr_pc(3);
                cpu.sbc(cpu.memory.fetch(addr), cycles);
            }
            (Instruction::SBC, AddressingMode::AbsoluteIndexedY(value)) => {
                let (addr, cycles) = absolute_indexed(*value, cpu.registers.index_y);

                cpu.incr_pc(3);
                cpu.sbc(cpu.memory.fetch(addr), cycles);
            }
            (Instruction::SBC, AddressingMode::IndexedIndirect(value)) => {
                cpu.incr_pc(2);
                cpu.sbc(indexedindirect(cpu, *value), 6);
            }
            (Instruction::SBC, AddressingMode::IndirectIndexed(value)) => {
                let (val, cycles) = indirectindexed(cpu, *value);

                cpu.incr_pc(2);
                cpu.sbc(val, cycles);
            }
            (Instruction::CMP, AddressingMode::Immediate(value)) => {
                cpu.incr_pc(2);
                cpu.cmp(*value, 2);
            }
            (Instruction::CMP, AddressingMode::ZeroPage(value)) => {
                cpu.incr_pc(2);
                cpu.cmp(cpu.memory.fetch(*value as u16), 3);
            }
            (Instruction::CMP, AddressingMode::ZeroPageIndexedX(value)) => {
                cpu.incr_pc(2);
                cpu.cmp(cpu.memory.fetch((*value + cpu.registers.index_x) as u16), 4);
            }
            (Instruction::CMP, AddressingMode::Absolute(value)) => {
                cpu.incr_pc(3);
                cpu.cmp(cpu.memory.fetch(*value), 4);
            }
            (Instruction::CMP, AddressingMode::AbsoluteIndexedX(value)) => {
                let (addr, cycles) = absolute_indexed(*value, cpu.registers.index_x);

                cpu.incr_pc(3);
                cpu.cmp(cpu.memory.fetch(addr), cycles);
            }
            (Instruction::CMP, AddressingMode::AbsoluteIndexedY(value)) => {
                let (addr, cycles) = absolute_indexed(*value, cpu.registers.index_y);

                cpu.incr_pc(3);
                cpu.cmp(cpu.memory.fetch(addr), cycles);
            }
            (Instruction::CMP, AddressingMode::IndexedIndirect(value)) => {
                cpu.incr_pc(2);
                cpu.cmp(indexedindirect(cpu, *value), 6);
            }
            (Instruction::CMP, AddressingMode::IndirectIndexed(value)) => {
                let (val, cycles) = indirectindexed(cpu, *value);

                cpu.incr_pc(2);
                cpu.cmp(val, cycles);
            }
            (Instruction::CPX, AddressingMode::Immediate(value)) => {
                cpu.incr_pc(2);
                cpu.cpx(*value, 2);
            }
            (Instruction::CPX, AddressingMode::ZeroPage(value)) => {
                cpu.incr_pc(2);
                cpu.cpx(cpu.memory.fetch(*value as u16), 3);
            }
            (Instruction::CPX, AddressingMode::Absolute(value)) => {
                cpu.incr_pc(3);
                cpu.cpx(cpu.memory.fetch(*value), 4);
            }
            (Instruction::CPY, AddressingMode::Immediate(value)) => {
                cpu.incr_pc(2);
                cpu.cpy(*value, 2);
            }
            (Instruction::CPY, AddressingMode::ZeroPage(value)) => {
                cpu.incr_pc(2);
                cpu.cpy(cpu.memory.fetch(*value as u16), 3);
            }
            (Instruction::CPY, AddressingMode::Absolute(value)) => {
                cpu.incr_pc(3);
                cpu.cpy(cpu.memory.fetch(*value), 4);
            }
            (Instruction::INC, AddressingMode::ZeroPage(value)) => {
                cpu.incr_pc(2);
                cpu.inc(*value as u16, 5);
            }
            (Instruction::INC, AddressingMode::ZeroPageIndexedX(value)) => {
                cpu.incr_pc(2);
                cpu.inc((*value + cpu.registers.index_x) as u16, 6);
            }
            (Instruction::INC, AddressingMode::Absolute(value)) => {
                cpu.incr_pc(3);
                cpu.inc(*value, 6);
            }
            (Instruction::INC, AddressingMode::AbsoluteIndexedX(value)) => {
                cpu.incr_pc(3);
                cpu.inc(value + (cpu.registers.index_x as u16), 7);
            }
            (Instruction::INX, AddressingMode::Implicit) => {
                cpu.incr_pc(1);
                cpu.inx();
            }
            (Instruction::INY, AddressingMode::Implicit) => {
                cpu.incr_pc(1);
                cpu.iny();
            }
            (Instruction::DEC, AddressingMode::ZeroPage(value)) => {
                cpu.incr_pc(2);
                cpu.dec(*value as u16, 5);
            }
            (Instruction::DEC, AddressingMode::ZeroPageIndexedX(value)) => {
                cpu.incr_pc(2);
                cpu.dec((*value + cpu.registers.index_x) as u16, 6);
            }
            (Instruction::DEC, AddressingMode::Absolute(value)) => {
                cpu.incr_pc(3);
                cpu.dec(*value, 6);
            }
            (Instruction::DEC, AddressingMode::AbsoluteIndexedX(value)) => {
                cpu.incr_pc(3);
                cpu.dec(value + (cpu.registers.index_x as u16), 7);
            }
            (Instruction::DEX, AddressingMode::Implicit) => {
                cpu.incr_pc(1);
                cpu.dex();
            }
            (Instruction::DEY, AddressingMode::Implicit) => {
                cpu.incr_pc(1);
                cpu.dey();
            }
            (Instruction::ASL, AddressingMode::Implicit) => {
                cpu.incr_pc(1);
                cpu.asl_acc();
            }
            (Instruction::ASL, AddressingMode::ZeroPage(value)) => {
                cpu.incr_pc(2);
                cpu.asl(*value as u16, 5);
            }
            (Instruction::ASL, AddressingMode::ZeroPageIndexedX(value)) => {
                cpu.incr_pc(2);
                cpu.asl((*value + cpu.registers.index_x) as u16, 6);
            }
            (Instruction::ASL, AddressingMode::Absolute(value)) => {
                cpu.incr_pc(3);
                cpu.asl(*value, 6);
            }
            (Instruction::ASL, AddressingMode::AbsoluteIndexedX(value)) => {
                cpu.incr_pc(3);
                cpu.asl(value + (cpu.registers.index_x as u16), 7);
            }
            (Instruction::LSR, AddressingMode::Implicit) => {
                cpu.incr_pc(1);
                cpu.lsr_acc();
            }
            (Instruction::LSR, AddressingMode::ZeroPage(value)) => {
                cpu.incr_pc(2);
                cpu.lsr(*value as u16, 5);
            }
            (Instruction::LSR, AddressingMode::ZeroPageIndexedX(value)) => {
                cpu.incr_pc(2);
                cpu.lsr((*value + cpu.registers.index_x) as u16, 6);
            }
            (Instruction::LSR, AddressingMode::Absolute(value)) => {
                cpu.incr_pc(3);
                cpu.lsr(*value, 6);
            }
            (Instruction::LSR, AddressingMode::AbsoluteIndexedX(value)) => {
                cpu.incr_pc(3);
                cpu.lsr(value + (cpu.registers.index_x as u16), 7);
            }
            (Instruction::ROL, AddressingMode::Implicit) => {
                cpu.incr_pc(1);
                cpu.rol_acc();
            }
            (Instruction::ROL, AddressingMode::ZeroPage(value)) => {
                cpu.incr_pc(2);
                cpu.rol(*value as u16, 5);
            }
            (Instruction::ROL, AddressingMode::ZeroPageIndexedX(value)) => {
                cpu.incr_pc(2);
                cpu.rol((*value + cpu.registers.index_x) as u16, 6);
            }
            (Instruction::ROL, AddressingMode::Absolute(value)) => {
                cpu.incr_pc(3);
                cpu.rol(*value, 6);
            }
            (Instruction::ROL, AddressingMode::AbsoluteIndexedX(value)) => {
                cpu.incr_pc(3);
                cpu.rol(value + (cpu.registers.index_x as u16), 7);
            }
            (Instruction::ROR, AddressingMode::Implicit) => {
                cpu.incr_pc(1);
                cpu.ror_acc();
            }
            (Instruction::ROR, AddressingMode::ZeroPage(value)) => {
                cpu.incr_pc(2);
                cpu.ror(*value as u16, 5);
            }
            (Instruction::ROR, AddressingMode::ZeroPageIndexedX(value)) => {
                cpu.incr_pc(2);
                cpu.ror((*value + cpu.registers.index_x) as u16, 6);
            }
            (Instruction::ROR, AddressingMode::Absolute(value)) => {
                cpu.incr_pc(3);
                cpu.ror(*value, 6);
            }
            (Instruction::ROR, AddressingMode::AbsoluteIndexedX(value)) => {
                cpu.incr_pc(3);
                cpu.ror(value + (cpu.registers.index_x as u16), 7);
            }
            (Instruction::JMP, AddressingMode::Absolute(value)) => {
                cpu.incr_pc(3);
                cpu.jmp(*value, 3);
            }
            (Instruction::JMP, AddressingMode::Indirect(value)) => {
                let bytes: [u8; 2] = value.to_le_bytes();

                let second_addr = if bytes[1] == 0xFF {
                    u16::from_le_bytes([bytes[0], 0x00])
                } else {
                    *value + 1
                };

                let addr =
                    u16::from_le_bytes([cpu.memory.fetch(*value), cpu.memory.fetch(second_addr)]);

                cpu.incr_pc(3);
                cpu.jmp(addr, 5);
            }
            (Instruction::JSR, AddressingMode::Absolute(value)) => {
                cpu.incr_pc(3);
                cpu.jsr(*value);
            }
            (Instruction::RTS, AddressingMode::Implicit) => {
                cpu.incr_pc(1); // Technically speaking, incrementing the PC here doesn't make a difference, but for consistency's sake...
                cpu.rts();
            }
            (Instruction::BCC, AddressingMode::Relative(value)) => {
                cpu.incr_pc(2);
                cpu.bcc(*value);
            }
            (Instruction::BCS, AddressingMode::Relative(value)) => {
                cpu.incr_pc(2);
                cpu.bcs(*value);
            }
            (Instruction::BEQ, AddressingMode::Relative(value)) => {
                cpu.incr_pc(2);
                cpu.beq(*value);
            },
            (Instruction::BMI, AddressingMode::Relative(value)) => {
                cpu.incr_pc(2);
                cpu.bmi(*value);
            },
            (Instruction::BNE, AddressingMode::Relative(value)) => {
                cpu.incr_pc(2);
                cpu.bne(*value);
            },
            (Instruction::BPL, AddressingMode::Relative(value)) => {
                cpu.incr_pc(2);
                cpu.bpl(*value);
            },
            (Instruction::BVC, AddressingMode::Relative(value)) => {
                cpu.incr_pc(2);
                cpu.bvc(*value);
            },
            (Instruction::BVS, AddressingMode::Relative(value)) => {
                cpu.incr_pc(2);
                cpu.bvs(*value);
            },
            (Instruction::CLC, AddressingMode::Implicit) => {
                cpu.incr_pc(1);
                cpu.clc();
            },
            (Instruction::CLD, AddressingMode::Implicit) => {
                cpu.incr_pc(1);
                cpu.cld();
            },
            (Instruction::CLI, AddressingMode::Implicit) => {
                cpu.incr_pc(1);
                cpu.cli();
            },
            (Instruction::CLV, AddressingMode::Implicit) => {
                cpu.incr_pc(1);
                cpu.clv();
            },
            (Instruction::SEC, AddressingMode::Implicit) => {
                cpu.incr_pc(1);
                cpu.sec();
            },
            (Instruction::SED, AddressingMode::Implicit) => {
                cpu.incr_pc(1);
                cpu.sed();
            },
            (Instruction::SEI, AddressingMode::Implicit) => {
                cpu.incr_pc(1);
                cpu.sei();
            },
            (Instruction::BRK, AddressingMode::Implicit) => todo!(),
            (Instruction::NOP, AddressingMode::Implicit) => {
                cpu.incr_pc(1);
                cpu.incr_cycles(2); // Not even bothering with writing a NOP impl.
            },
            (Instruction::RTI, AddressingMode::Implicit) => todo!(),
            _ => todo!("Hit an unimplemented illegal instruction: {:?}", &self),
            /*
            Instruction::SHY => todo!(),
            Instruction::ALR => todo!(),
            Instruction::ANC => todo!(),
            Instruction::ARR => todo!(),
            Instruction::AXS => todo!(),
            Instruction::LAX => todo!(),
            Instruction::SAX => todo!(),
            Instruction::DCP => todo!(),
            Instruction::ISC => todo!(),
            Instruction::RLA => todo!(),
            Instruction::RRA => todo!(),
            Instruction::SLO => todo!(),
            Instruction::SRE => todo!(),
            Instruction::SKB => todo!(),
            Instruction::IGN => todo!(),
            */
        }
    }
}
