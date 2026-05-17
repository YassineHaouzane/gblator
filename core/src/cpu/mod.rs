use crate::utils::{ByteOps, merge_bytes};

pub mod opcodes;

#[derive(Copy, Clone)]
pub enum Regs8 {
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

    pub fn get_r8(&self, r: Regs8) -> u8 {
        match r {
            Regs8::A => self.registers.a,
            Regs8::B => self.registers.b,
            Regs8::C => self.registers.c,
            Regs8::D => self.registers.d,
            Regs8::E => self.registers.e,
            Regs8::F => self.registers.f,
            Regs8::H => self.registers.h,
            Regs8::L => self.registers.l,
        }
    }

    pub fn set_r8(&mut self, r: Regs8, val: u8) {
        match r {
            Regs8::A => self.registers.a = val,
            Regs8::B => self.registers.b = val,
            Regs8::C => self.registers.c = val,
            Regs8::D => self.registers.d = val,
            Regs8::E => self.registers.e = val,
            // Note: The bottom four bits of F shall always be 0
            Regs8::F => self.registers.f = val & 0xF0,
            Regs8::H => self.registers.h = val,
            Regs8::L => self.registers.l = val,
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
                self.set_r8(Regs8::A, high_val);
                self.set_r8(Regs8::F, low_val);
            }
            Regs16::BC => {
                self.set_r8(Regs8::B, high_val);
                self.set_r8(Regs8::C, low_val);
            }
            Regs16::DE => {
                self.set_r8(Regs8::D, high_val);
                self.set_r8(Regs8::E, low_val);
            }
            Regs16::HL => {
                self.set_r8(Regs8::H, high_val);
                self.set_r8(Regs8::L, low_val);
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

    pub fn fetch(&mut self) -> u8 {
        let val = self.read_ram(self.registers.pc);
        self.registers.pc += 1;
        val
    }

    pub fn fetch_u16(&mut self) -> u16 {
        let low = self.fetch();
        let high = self.fetch();
        let val = merge_bytes(high, low);
        val
    }

    pub fn read_ram(&self, addr: u16) -> u8 {
        todo!();
    }

    pub fn write_ram(&mut self, addr: u16, val: u8) {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_flag_returns_true_when_flag_is_set() {
        let mut cpu = Cpu::new();

        cpu.registers.f = 0b1111_0000;

        assert!(cpu.get_flag(Flags::Z));
        assert!(cpu.get_flag(Flags::N));
        assert!(cpu.get_flag(Flags::H));
        assert!(cpu.get_flag(Flags::C));
    }

    #[test]
    fn get_flag_returns_false_when_flag_is_not_set() {
        let mut cpu = Cpu::new();

        cpu.registers.f = 0b0000_0000;

        assert!(!cpu.get_flag(Flags::Z));
        assert!(!cpu.get_flag(Flags::N));
        assert!(!cpu.get_flag(Flags::H));
        assert!(!cpu.get_flag(Flags::C));
    }

    #[test]
    fn set_flag_sets_each_flag_correctly() {
        let mut cpu = Cpu::new();

        cpu.set_flag(Flags::Z, true);
        assert_eq!(cpu.registers.f, 0b1000_0000);

        cpu.registers.f = 0;
        cpu.set_flag(Flags::N, true);
        assert_eq!(cpu.registers.f, 0b0100_0000);

        cpu.registers.f = 0;
        cpu.set_flag(Flags::H, true);
        assert_eq!(cpu.registers.f, 0b0010_0000);

        cpu.registers.f = 0;
        cpu.set_flag(Flags::C, true);
        assert_eq!(cpu.registers.f, 0b0001_0000);
    }

    #[test]
    fn set_flag_clears_each_flag_correctly() {
        let mut cpu = Cpu::new();

        cpu.registers.f = 0b1111_0000;
        cpu.set_flag(Flags::Z, false);
        assert_eq!(cpu.registers.f, 0b0111_0000);

        cpu.registers.f = 0b1111_0000;
        cpu.set_flag(Flags::N, false);
        assert_eq!(cpu.registers.f, 0b1011_0000);

        cpu.registers.f = 0b1111_0000;
        cpu.set_flag(Flags::H, false);
        assert_eq!(cpu.registers.f, 0b1101_0000);

        cpu.registers.f = 0b1111_0000;
        cpu.set_flag(Flags::C, false);
        assert_eq!(cpu.registers.f, 0b1110_0000);
    }

    #[test]
    fn set_flag_does_not_modify_other_flags_when_setting() {
        let mut cpu = Cpu::new();

        cpu.registers.f = 0b0100_0000;
        cpu.set_flag(Flags::Z, true);

        assert_eq!(cpu.registers.f, 0b1100_0000);
    }

    #[test]
    fn set_flag_does_not_modify_other_flags_when_clearing() {
        let mut cpu = Cpu::new();

        cpu.registers.f = 0b1111_0000;
        cpu.set_flag(Flags::H, false);

        assert!(cpu.get_flag(Flags::Z));
        assert!(cpu.get_flag(Flags::N));
        assert!(!cpu.get_flag(Flags::H));
        assert!(cpu.get_flag(Flags::C));
    }
}
