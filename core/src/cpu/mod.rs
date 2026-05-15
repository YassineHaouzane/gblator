use crate::utils::{ByteOps, merge_bytes};

#[derive(Copy, Clone)]
pub enum Regs {
    A,
    B,
    C,
    D,
    E,
    F,
    H,
    L,
}

#[derive(Copy, Clone)]
pub enum Regs16 {
    AF,
    BC,
    DE,
    HL,
    SP,
}

pub enum Flags {
    Z,
    N,
    H,
    C,
}

struct Registers {
    pc: u16,
    sp: u16,
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: u8,
    h: u8,
    l: u8,
}

pub struct Cpu {
    registers: Registers,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            registers: Registers {
                pc: 0x0000,
                sp: 0x0000,
                a: 0x00,
                b: 0x00,
                c: 0x00,
                d: 0x00,
                e: 0x00,
                f: 0x00,
                h: 0x00,
                l: 0x00,
            },
        }
    }

    pub fn get_r8(&self, r: Regs) -> u8 {
        match r {
            Regs::A => self.registers.a,
            Regs::B => self.registers.b,
            Regs::C => self.registers.c,
            Regs::D => self.registers.d,
            Regs::E => self.registers.e,
            Regs::F => self.registers.f,
            Regs::H => self.registers.h,
            Regs::L => self.registers.l,
        }
    }

    pub fn set_r8(&mut self, r: Regs, val: u8) {
        match r {
            Regs::A => self.registers.a = val,
            Regs::B => self.registers.b = val,
            Regs::C => self.registers.c = val,
            Regs::D => self.registers.d = val,
            Regs::E => self.registers.e = val,
            // Note: The bottom four bits of F shall always be 0
            Regs::F => self.registers.f = val & 0xF0,
            Regs::H => self.registers.h = val,
            Regs::L => self.registers.l = val,
        }
    }

    pub fn get_r16(&self, r: Regs16) -> u16 {
        match r {
            Regs16::AF => merge_bytes(self.registers.a, self.registers.b),
            Regs16::BC => merge_bytes(self.registers.b, self.registers.c),
            Regs16::DE => merge_bytes(self.registers.d, self.registers.e),
            Regs16::HL => merge_bytes(self.registers.h, self.registers.l),
            Regs16::SP => self.registers.sp,
        }
    }

    pub fn set_r16(&mut self, r: Regs16, val: u16) {
        let high_val = val.high_byte();
        let low_val = val.low_byte();
        match r {
            Regs16::AF => {
                self.set_r8(Regs::A, high_val);
                self.set_r8(Regs::F, low_val);
            }
            Regs16::BC => {
                self.set_r8(Regs::B, high_val);
                self.set_r8(Regs::C, low_val);
            }
            Regs16::DE => {
                self.set_r8(Regs::D, high_val);
                self.set_r8(Regs::E, low_val);
            }
            Regs16::HL => {
                self.set_r8(Regs::H, high_val);
                self.set_r8(Regs::L, low_val);
            }
            Regs16::SP => self.registers.sp = val,
        }
    }

    pub fn get_flag(&self, f: Flags) -> bool {
        let register_f = self.registers.f;
        match f {
            Flags::Z => (register_f & 0b1000_0000) != 0,
            Flags::N => (register_f & 0b0100_0000) != 0,
            Flags::H => (register_f & 0b0010_0000) != 0,
            Flags::C => (register_f & 0b0001_0000) != 0,
        }
    }

    pub fn set_flag(&mut self, f: Flags, val: bool) {
        if val {
            match f {
                Flags::Z => self.registers.f |= 0b1000_0000,
                Flags::N => self.registers.f |= 0b0100_0000,
                Flags::H => self.registers.f |= 0b0010_0000,
                Flags::C => self.registers.f |= 0b0001_0000,
            }
        } else {
            match f {
                Flags::Z => self.registers.f &= 0b0111_0000,
                Flags::N => self.registers.f &= 0b1011_0000,
                Flags::H => self.registers.f &= 0b1101_0000,
                Flags::C => self.registers.f &= 0b1110_0000,
            }
        }
    }
}
