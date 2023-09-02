use crate::{
    cpu::{StatusFlags, CPU},
    utils::bits::get_bit,
};

impl CPU {
    fn set_z_flag(&mut self, value: u8) {
        if value == 0 {
            self.registers.status_register.zero = true;
        }
    }

    fn set_n_flag(&mut self, value: u8) {
        if get_bit(value, 7) {
            self.registers.status_register.negative = true;
        }
    }

    fn branch(&mut self, value: i8) {
        let old_bytes: [u8; 2] = self.registers.program_counter.to_le_bytes();

        let new_pc = self
            .registers
            .program_counter
            .wrapping_add_signed(value as i16);

        let new_bytes: [u8; 2] = new_pc.to_le_bytes();

        if old_bytes[0] == new_bytes[0] {
            self.incr_cycles(2);
        }

        self.registers.program_counter = new_pc;

        self.incr_cycles(1);
    }

    pub fn lda(&mut self, value: u8, cycles: usize) {
        self.registers.accumulator = value;

        self.set_z_flag(self.registers.accumulator);

        self.set_n_flag(self.registers.accumulator);

        self.incr_cycles(cycles)
    }

    pub fn ldx(&mut self, value: u8, cycles: usize) {
        self.registers.index_x = value;

        self.set_z_flag(self.registers.index_x);

        self.set_n_flag(self.registers.index_x);

        self.incr_cycles(cycles);
    }

    pub fn ldy(&mut self, value: u8, cycles: usize) {
        self.registers.index_y = value;

        self.set_z_flag(self.registers.index_y);

        self.set_n_flag(self.registers.index_y);

        self.incr_cycles(cycles);
    }

    pub fn sta(&mut self, addr: u16, cycles: usize) {
        self.memory.write(addr, self.registers.accumulator);

        self.incr_cycles(cycles);
    }

    pub fn stx(&mut self, addr: u16, cycles: usize) {
        self.memory.write(addr, self.registers.index_x);

        self.incr_cycles(cycles);
    }

    pub fn sty(&mut self, addr: u16, cycles: usize) {
        self.memory.write(addr, self.registers.index_y);

        self.incr_cycles(cycles);
    }

    pub fn tax(&mut self) {
        self.registers.index_x = self.registers.accumulator;

        self.set_z_flag(self.registers.index_x);

        self.set_n_flag(self.registers.index_x);

        self.incr_cycles(2);
    }

    pub fn tay(&mut self) {
        self.registers.index_y = self.registers.accumulator;

        self.set_z_flag(self.registers.index_y);

        self.set_n_flag(self.registers.index_y);

        self.incr_cycles(2);
    }

    pub fn txa(&mut self) {
        self.registers.accumulator = self.registers.index_x;

        self.set_z_flag(self.registers.accumulator);

        self.set_n_flag(self.registers.accumulator);

        self.incr_cycles(2);
    }

    pub fn tya(&mut self) {
        self.registers.accumulator = self.registers.index_y;

        self.set_z_flag(self.registers.accumulator);

        self.set_n_flag(self.registers.accumulator);

        self.incr_cycles(2);
    }

    pub fn tsx(&mut self) {
        self.registers.index_x = self.registers.stack_pointer;

        self.set_z_flag(self.registers.index_x);

        self.set_n_flag(self.registers.index_x);

        self.incr_cycles(2);
    }

    pub fn txs(&mut self) {
        self.registers.stack_pointer = self.registers.index_x;

        self.incr_cycles(2);
    }

    pub fn pha(&mut self) {
        self.memory.write(
            0x0100 + self.registers.stack_pointer as u16,
            self.registers.accumulator,
        );

        self.registers.stack_pointer -= 1;

        self.incr_cycles(3);
    }

    pub fn php(&mut self) {
        assert!((0x0100 + self.registers.stack_pointer as u16) > 0x00FF);
        assert!((0x0100 + self.registers.stack_pointer as u16) < 0x0200);

        self.memory.write(
            0x0100 + self.registers.stack_pointer as u16,
            self.registers.status_register.into(),
        );

        self.registers.stack_pointer -= 1;

        self.incr_cycles(3);
    }

    pub fn pla(&mut self) {
        assert!((0x0100 + self.registers.stack_pointer as u16) > 0x00FF);
        assert!((0x0100 + self.registers.stack_pointer as u16) < 0x0200);

        self.registers.accumulator = self
            .memory
            .fetch(0x0100 + self.registers.stack_pointer as u16);

        self.registers.stack_pointer += 1;

        self.set_z_flag(self.registers.accumulator);

        self.set_n_flag(self.registers.accumulator);

        self.incr_cycles(4);
    }

    pub fn plp(&mut self) {
        assert!((0x0100 + self.registers.stack_pointer as u16) > 0x00FF);
        assert!((0x0100 + self.registers.stack_pointer as u16) < 0x0200);

        let temp = self
            .memory
            .fetch(0x0100 + self.registers.stack_pointer as u16);

        self.registers.status_register = StatusFlags {
            carry: get_bit(temp, 0),
            zero: get_bit(temp, 1),
            interrupt_disable: get_bit(temp, 2),
            decimal: get_bit(temp, 3),
            b_flag_4: get_bit(temp, 4),
            b_flag_5: get_bit(temp, 5),
            overflow: get_bit(temp, 6),
            negative: get_bit(temp, 7),
        };

        self.registers.stack_pointer += 1;

        self.incr_cycles(4);
    }

    pub fn and(&mut self, value: u8, cycles: usize) {
        self.registers.accumulator &= value;

        self.set_z_flag(self.registers.accumulator);

        self.set_n_flag(self.registers.accumulator);

        self.incr_cycles(cycles);
    }

    pub fn eor(&mut self, value: u8, cycles: usize) {
        self.registers.accumulator ^= value;

        self.set_z_flag(self.registers.accumulator);

        self.set_n_flag(self.registers.accumulator);

        self.incr_cycles(cycles);
    }

    pub fn ora(&mut self, value: u8, cycles: usize) {
        self.registers.accumulator |= value;

        self.set_z_flag(self.registers.accumulator);

        self.set_n_flag(self.registers.accumulator);

        self.incr_cycles(cycles);
    }

    pub fn bit(&mut self, value: u8, cycles: usize) {
        self.set_z_flag(self.registers.accumulator & value);

        if get_bit(value, 6) {
            self.registers.status_register.overflow = true;
        }

        self.set_n_flag(value);

        self.incr_cycles(cycles);
    }

    pub fn adc(&mut self, value: u8, cycles: usize) {
        let old_bit_7 = get_bit(self.registers.accumulator, 7);

        let (temp_val, temp_overflow) = self.registers.accumulator.overflowing_add(value);
        let (temp_val, temp_overflow_2) =
            temp_val.overflowing_add(self.registers.status_register.carry as u8);

        self.registers.accumulator = temp_val;

        if temp_overflow || temp_overflow_2 {
            self.registers.status_register.carry = true;
        };

        if old_bit_7 != get_bit(self.registers.accumulator, 7) {
            self.registers.status_register.overflow = true;
        };

        self.set_z_flag(self.registers.accumulator);

        self.set_n_flag(self.registers.accumulator);

        self.incr_cycles(cycles);
    }

    pub fn sbc(&mut self, value: u8, cycles: usize) {
        let old_bit_7 = get_bit(self.registers.accumulator, 7);

        let (temp_val, temp_overflow) = self.registers.accumulator.overflowing_sub(value);
        let (temp_val, temp_overflow_2) =
            temp_val.overflowing_sub(!self.registers.status_register.carry as u8); // NOT of the carry bit

        self.registers.accumulator = temp_val;

        if temp_overflow || temp_overflow_2 {
            self.registers.status_register.carry = false;
        };

        if old_bit_7 != get_bit(self.registers.accumulator, 7) {
            self.registers.status_register.overflow = true;
        };

        self.set_z_flag(self.registers.accumulator);

        self.set_n_flag(self.registers.accumulator);

        self.incr_cycles(cycles);
    }

    pub fn cmp(&mut self, value: u8, cycles: usize) {
        if self.registers.accumulator >= value {
            self.registers.status_register.carry = true;
        }

        if self.registers.accumulator == value {
            self.registers.status_register.zero = true;
        }

        self.set_n_flag(self.registers.accumulator - value);

        self.incr_cycles(cycles);
    }

    pub fn cpx(&mut self, value: u8, cycles: usize) {
        if self.registers.index_x >= value {
            self.registers.status_register.carry = true;
        }

        if self.registers.index_x == value {
            self.registers.status_register.zero = true;
        }

        self.set_n_flag(self.registers.index_x - value);

        self.incr_cycles(cycles);
    }

    pub fn cpy(&mut self, value: u8, cycles: usize) {
        if self.registers.index_y >= value {
            self.registers.status_register.carry = true;
        }

        if self.registers.index_y == value {
            self.registers.status_register.zero = true;
        }

        self.set_n_flag(self.registers.index_y - value);

        self.incr_cycles(cycles);
    }

    pub fn inc(&mut self, value: u16, cycles: usize) {
        let result = self.memory.fetch(value) + 1;

        self.memory.write(value, result);

        self.set_z_flag(result);

        self.set_n_flag(result);

        self.incr_cycles(cycles);
    }

    pub fn inx(&mut self) {
        self.registers.index_x += 1;

        self.set_z_flag(self.registers.index_x);

        self.set_n_flag(self.registers.index_x);

        self.incr_cycles(2);
    }

    pub fn iny(&mut self) {
        self.registers.index_y += 1;

        self.set_z_flag(self.registers.index_y);

        self.set_n_flag(self.registers.index_y);

        self.incr_cycles(2);
    }

    pub fn dec(&mut self, value: u16, cycles: usize) {
        let result = self.memory.fetch(value) - 1;

        self.memory.write(value, result);

        self.set_z_flag(result);

        self.set_n_flag(result);

        self.incr_cycles(cycles);
    }

    pub fn dex(&mut self) {
        self.registers.index_x -= 1;

        self.set_z_flag(self.registers.index_x);

        self.set_n_flag(self.registers.index_x);

        self.incr_cycles(2);
    }

    pub fn dey(&mut self) {
        self.registers.index_y -= 1;

        self.set_z_flag(self.registers.index_y);

        self.set_n_flag(self.registers.index_y);

        self.incr_cycles(2);
    }

    pub fn asl_acc(&mut self) {
        self.registers.status_register.carry = get_bit(self.registers.accumulator, 7);

        self.registers.accumulator = self.registers.accumulator << 1;

        self.set_z_flag(self.registers.accumulator);

        self.set_n_flag(self.registers.accumulator);

        self.incr_cycles(2);
    }

    pub fn asl(&mut self, value: u16, cycles: usize) {
        self.registers.status_register.carry = get_bit(self.memory.fetch(value), 7);

        let result = self.memory.fetch(value) << 1;

        self.memory.write(value, result);

        self.set_z_flag(result);

        self.set_n_flag(result);

        self.incr_cycles(cycles);
    }

    pub fn lsr_acc(&mut self) {
        self.registers.status_register.carry = get_bit(self.registers.accumulator, 0);

        self.registers.accumulator = self.registers.accumulator >> 1;

        self.set_z_flag(self.registers.accumulator);

        self.set_n_flag(self.registers.accumulator);

        self.incr_cycles(2);
    }

    pub fn lsr(&mut self, value: u16, cycles: usize) {
        self.registers.status_register.carry = get_bit(self.memory.fetch(value), 0);

        let result = self.memory.fetch(value) >> 1;

        self.memory.write(value, result);

        self.set_z_flag(result);

        self.set_n_flag(result);

        self.incr_cycles(cycles);
    }

    pub fn rol_acc(&mut self) {
        let old_carry = self.registers.status_register.carry;
        self.registers.status_register.carry = get_bit(self.registers.accumulator, 7);

        self.registers.accumulator = (self.registers.accumulator << 1) + (old_carry as u8);

        self.set_z_flag(self.registers.accumulator);

        self.set_n_flag(self.registers.accumulator);

        self.incr_cycles(2);
    }

    pub fn rol(&mut self, value: u16, cycles: usize) {
        let old_carry = self.registers.status_register.carry;
        self.registers.status_register.carry = get_bit(self.memory.fetch(value), 7);

        let result = (self.memory.fetch(value) << 1) + (old_carry as u8);

        self.memory.write(value, result);

        self.set_z_flag(result);

        self.set_n_flag(result);

        self.incr_cycles(cycles);
    }

    pub fn ror_acc(&mut self) {
        let old_carry = self.registers.status_register.carry;
        self.registers.status_register.carry = get_bit(self.registers.accumulator, 0);

        self.registers.accumulator =
            (self.registers.accumulator >> 1) + (old_carry as u8) * 0b1000_0000;

        self.set_z_flag(self.registers.accumulator);

        self.set_n_flag(self.registers.accumulator);

        self.incr_cycles(2);
    }

    pub fn ror(&mut self, value: u16, cycles: usize) {
        let old_carry = self.registers.status_register.carry;
        self.registers.status_register.carry = get_bit(self.memory.fetch(value), 0);

        let result = (self.memory.fetch(value) >> 1) + (old_carry as u8) * 0b1000_0000;

        self.memory.write(value, result);

        self.set_z_flag(result);

        self.set_n_flag(result);

        self.incr_cycles(cycles);
    }

    pub fn jmp(&mut self, value: u16, cycles: usize) {
        self.registers.program_counter = value;

        self.incr_cycles(cycles);
    }

    pub fn jsr(&mut self, value: u16) {
        let bytes: [u8; 2] = (self.registers.program_counter - 1).to_le_bytes();

        self.stack_push(bytes[0]);

        self.stack_push(bytes[1]);

        self.registers.program_counter = value;

        self.incr_cycles(6);
    }

    pub fn rts(&mut self) {
        let (byte_2, byte_1) = (self.stack_pull(), self.stack_pull());

        self.registers.program_counter = u16::from_le_bytes([byte_1, byte_2]) + 1;

        self.incr_cycles(6);
    }

    pub fn bcc(&mut self, value: i8) {
        if !self.registers.status_register.carry {
            self.branch(value);
        }

        self.incr_cycles(2);
    }

    pub fn bcs(&mut self, value: i8) {
        if self.registers.status_register.carry {
            self.branch(value);
        }

        self.incr_cycles(2);
    }

    pub fn beq(&mut self, value: i8) {
        if self.registers.status_register.zero {
            self.branch(value);
        }

        self.incr_cycles(2);
    }

    pub fn bmi(&mut self, value: i8) {
        if self.registers.status_register.negative {
            self.branch(value);
        }

        self.incr_cycles(2);
    }

    pub fn bne(&mut self, value: i8) {
        if !self.registers.status_register.zero {
            self.branch(value);
        }

        self.incr_cycles(2);
    }

    pub fn bpl(&mut self, value: i8) {
        if !self.registers.status_register.negative {
            self.branch(value);
        }

        self.incr_cycles(2);
    }

    pub fn bvc(&mut self, value: i8) {
        if !self.registers.status_register.overflow {
            self.branch(value);
        }

        self.incr_cycles(2);
    }

    pub fn bvs(&mut self, value: i8) {
        if self.registers.status_register.overflow {
            self.branch(value);
        }

        self.incr_cycles(2);
    }

    pub fn clc(&mut self) {
        self.registers.status_register.carry = false;

        self.incr_cycles(2);
    }

    pub fn cld(&mut self) {
        self.registers.status_register.decimal = false;

        self.incr_cycles(2);
    }

    pub fn cli(&mut self) {
        self.registers.status_register.interrupt_disable = false;

        self.incr_cycles(2);
    }

    pub fn clv(&mut self) {
        self.registers.status_register.overflow = false;

        self.incr_cycles(2);
    }

    pub fn sec(&mut self) {
        self.registers.status_register.carry = true;

        self.incr_cycles(2);
    }

    pub fn sed(&mut self) {
        self.registers.status_register.decimal = true;

        self.incr_cycles(2);
    }

    pub fn sei(&mut self) {
        self.registers.status_register.interrupt_disable = true;

        self.incr_cycles(2);
    }

    pub fn brk(&mut self) {
        self.registers.status_register.b_flag_4 = true;
        self.registers.status_register.b_flag_5 = true;
        self.registers.status_register.interrupt_disable = true;
    }


}
