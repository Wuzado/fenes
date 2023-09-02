// 8-bit CPU
// 16-bit address space
// 8-bit instructions
pub mod exec;
pub mod implementations;

// Note: instructions acting on an accumulator are treated as addressed implicitly.
#[derive(Debug)]
pub enum AddressingMode {
    /// abbr: (empty) / A when acting on an accumulator
    Implicit,
    /// abbr: #v
    Immediate(u8),
    /// abbr: d
    ZeroPage(u8),
    /// abbr: a
    Absolute(u16),
    /// abbr: label
    Relative(i8),
    /// abbr: (a)
    Indirect(u16),
    // Indexed:
    /// abbr: d,x
    ZeroPageIndexedX(u8),
    /// abbr: d,y
    ZeroPageIndexedY(u8),
    /// abbr: a,x
    AbsoluteIndexedX(u16),
    /// abbr: a,y
    AbsoluteIndexedY(u16),
    /// abbr: (d, x) / IDX
    IndexedIndirect(u8),
    /// abbr: (d),y / IDY
    IndirectIndexed(u8),
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug)]
pub enum Instruction {
    // Load/Store operations
    /// Load Accumulator
    LDA,
    /// Load X Register
    LDX,
    /// Load Y Register
    LDY,
    /// Store Accumulator
    STA,
    /// Store X Register
    STX,
    /// Store Y Register
    STY,
    // Register transfers
    /// Transfer accumulator to X
    TAX,
    /// Transfer accumulator to Y
    TAY,
    /// Transfer X to accumulator
    TXA,
    /// Transfer Y to accumulator
    TYA,
    // Stack operations
    /// Transfer stack pointer to X
    TSX,
    /// Transfer X to stack pointer
    TXS,
    /// Push accumulator on stack
    PHA,
    /// Push processor status on stack
    PHP,
    /// Pull accumulator from stack
    PLA,
    /// Pull processor status from stack
    PLP,
    // Logical
    /// Logical AND
    AND,
    /// Exclusive OR / XOR
    EOR,
    /// Logical Inclusive OR (or just OR)
    ORA,
    /// Bit Test
    BIT,
    // Arithmetic
    /// Add with Carry
    ADC,
    /// Subtract with Carry
    SBC,
    /// Compare accumulator
    CMP,
    /// Compare X register
    CPX,
    /// Compare Y register
    CPY,
    // Increments & decrements
    /// Increment a memory location
    INC,
    /// Increment the X register
    INX,
    /// Increment the Y register
    INY,
    /// Decrement a memory location
    DEC,
    /// Decrement the X register
    DEX,
    /// Decrement the Y register
    DEY,
    // Shifts
    /// Arithmetic Shift Left
    ASL,
    /// Logical Shift Right
    LSR,
    /// Rotate Left
    ROL,
    /// Rotate Right
    ROR,
    // Jumps & calls
    /// Jump to another location
    JMP,
    /// Jump to a subroutine
    JSR,
    /// Return from subroutine
    RTS,
    // Branches
    /// Branch if carry flag clear
    BCC,
    /// Branch if carry flag set
    BCS,
    /// Branch if zero flag set
    BEQ,
    /// Branch if negative flag set
    BMI,
    /// Branch if zero flag clear
    BNE,
    /// Branch if negative flag clear
    BPL,
    /// Branch if overflow flag clear
    BVC,
    /// Branch if overflow flag set
    BVS,
    // Status flag changes
    /// Clear carry flag
    CLC,
    /// Clear decimal mode flag
    CLD,
    /// Clear interrupt disable flag
    CLI,
    /// Clear overflow flag
    CLV,
    /// Set carry flag
    SEC,
    /// Set decimal mode flag
    SED,
    /// Set interrupt disable flag
    SEI,
    // System functions
    /// Force an interrupt
    BRK,
    /// No operation
    NOP,
    /// Return from interrupt
    RTI,
    // Unofficial opcodes
    // Refer to: https://www.nesdev.org/wiki/Programming_with_unofficial_opcodes
    SHY,
    ALR,
    ANC,
    ARR,
    AXS,
    LAX,
    SAX,
    DCP,
    ISC,
    RLA,
    RRA,
    SLO,
    SRE,
    SHX,
    XAA,
    AHX,
    TAS,
    LAS,
}
