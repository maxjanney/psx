use super::Psx;

use std::fmt;

// Initial value for the pc
const RESET_PC: u32 = 0xbfc00000;

pub struct Cpu {
    // General purpose registers
    pub regs: [u32; 32],
    // Program counter
    pub pc: u32,
    // Next pc for the branch delay slot
    pub next_pc: u32,
    // Load delay slot
    pub delayed_load: Option<(usize, u32)>,
    // HI multiply/divide result
    pub hi: u32,
    // LO multiply/divide result
    pub lo: u32,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            regs: [0u32; 32],
            pc: RESET_PC,
            next_pc: RESET_PC.wrapping_add(4),
            delayed_load: None,
            hi: 0,
            lo: 0,
        }
    }

    // Set the given register
    fn set_reg(&mut self, reg: usize, val: u32) {
        self.regs[reg] = val;
        // R0 is always zero
        self.regs[0] = 0;
    }

    // Perform a delayed load, if any
    fn delayed_load(&mut self) {
        if let Some((reg, val)) = self.delayed_load {
            self.set_reg(reg, val);
            self.delayed_load = None;
        }
    }
}

impl fmt::Display for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "pc: 0x{:08x}", self.pc)?;
        for (i, r) in self.regs.iter().enumerate() {
            writeln!(f, "{}: 0x{:08x}", REG_NAMES[i], r)?;
        }
        Ok(())
    }
}

pub fn step(psx: &mut Psx) {}

// TODO: Fetch an instruction from memory
fn fetch_instruction(psx: &mut Psx) -> Instruction {
    Instruction::new(0)
}

#[derive(Copy, Clone)]
pub struct Instruction(u32);

impl Instruction {
    pub fn new(i: u32) -> Self {
        Self(i)
    }

    pub const fn op(self) -> u32 {
        (self.0 >> 26) & 0x3f
    }

    pub const fn funct(self) -> u32 {
        self.0 & 0x1f
    }

    pub const fn rs(self) -> usize {
        ((self.0 >> 21) & 0x1f) as usize
    }

    pub const fn rt(self) -> usize {
        ((self.0 >> 16) & 0x1f) as usize
    }

    pub const fn rd(self) -> usize {
        ((self.0 >> 11) & 0x1f) as usize
    }

    pub const fn shmat(self) -> u32 {
        (self.0 >> 6) & 0x1f
    }

    pub const fn simm(self) -> u32 {
        ((self.0 & 0xffff) as i16) as u32
    }

    pub const fn imm(self) -> u32 {
        self.0 & 0xfff
    }

    pub const fn jimm(self) -> u32 {
        (self.0 & 0x03ffffff) << 2
    }
}

const REG_NAMES: [&'static str; 32] = [
    "r0", "at", "v0", "v1", "a0", "a1", "a2", "a3", "t0", "t1", "t2", "t3", "t4", "t5", "t6", "t7",
    "s0", "s1", "s2", "s3", "s4", "s5", "s7", "s7", "t8", "t9", "k0", "k1", "gp", "sp", "fp", "ra",
];
