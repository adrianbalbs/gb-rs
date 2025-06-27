use crate::ram::Memory;

/// Notes:
///
/// We can combine registers to form a 16-bit register, e.g. h and l can be
/// combined to form HL
///
/// TODO: Figure out a nice way to read/write 16 bit registers, might want to
/// implement methods somewhere
struct Registers {
    a: u8,
    f: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
}

enum Flag {
    Z = 1 << 0,
    N = 1 << 1,
    H = 1 << 2,
    C = 1 << 3,
}

pub struct Cpu {
    registers: Registers,
    // TODO: Would it be better if this is stored as ref? Might need to
    // consider thinking about interior mutability
    memory: Memory,
    sp: u16,
    pc: u16,
}

impl Cpu {
    fn set_flag(&mut self, flag: Flag, val: bool) {
        if val {
            self.registers.f |= flag as u8;
        } else {
            self.registers.f &= !(flag as u8);
        }
    }

    fn is_flag_set(&self, flag: Flag) -> bool {
        self.registers.f & (flag as u8) != 0
    }
}
