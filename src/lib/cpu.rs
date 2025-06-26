use crate::ram::Memory;

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

enum Flags {
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
