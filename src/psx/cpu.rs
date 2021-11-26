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

// Add unsigned word
fn addu(cpu: &mut Cpu, i: Instruction) {
    let s = cpu.regs[i.rs()];
    let t = cpu.regs[i.rt()];
    cpu.delayed_load();
    cpu.set_reg(i.rd(), s.wrapping_add(t));
}

// Subtract unsigned word
fn subu(cpu: &mut Cpu, i: Instruction) {
    let s = cpu.regs[i.rs()];
    let t = cpu.regs[i.rt()];
    cpu.delayed_load();
    cpu.set_reg(i.rd(), s.wrapping_sub(t));
}

// Add immediate unsigned word
fn addiu(cpu: &mut Cpu, i: Instruction) {
    let s = cpu.regs[i.rs()];
    let imm = i.simm();
    cpu.delayed_load();
    cpu.set_reg(i.rd(), s.wrapping_add(imm));
}

// Set on less than
fn slt(cpu: &mut Cpu, i: Instruction) {
    let s = cpu.regs[i.rs()] as i32;
    let t = cpu.regs[i.rt()] as i32;
    cpu.delayed_load();
    cpu.set_reg(i.rd(), (s < t) as u32)
}

// Set on less than unsigned
fn sltu(cpu: &mut Cpu, i: Instruction) {
    let s = cpu.regs[i.rs()];
    let t = cpu.regs[i.rt()];
    cpu.delayed_load();
    cpu.set_reg(i.rd(), (s < t) as u32);
}

// Set on less than immediate
fn slti(cpu: &mut Cpu, i: Instruction) {
    let s = cpu.regs[i.rs()] as i32;
    let imm = i.simm() as i32;
    cpu.delayed_load();
    cpu.set_reg(i.rt(), (s < imm) as u32);
}

// Set on less than immediate unsigned
fn sltiu(cpu: &mut Cpu, i: Instruction) {
    let s = cpu.regs[i.rs()];
    let imm = i.simm();
    cpu.delayed_load();
    cpu.set_reg(i.rt(), (s < imm) as u32);
}

// Bitwise logical AND
fn and(cpu: &mut Cpu, i: Instruction) {
    let s = cpu.regs[i.rs()];
    let t = cpu.regs[i.rt()];
    cpu.delayed_load();
    cpu.set_reg(i.rd(), s & t);
}

// Bitwise logical OR
fn or(cpu: &mut Cpu, i: Instruction) {
    let s = cpu.regs[i.rs()];
    let t = cpu.regs[i.rt()];
    cpu.delayed_load();
    cpu.set_reg(i.rd(), s | t);
}

// Bitwise logical exclusive OR
fn xor(cpu: &mut Cpu, i: Instruction) {
    let s = cpu.regs[i.rs()];
    let t = cpu.regs[i.rt()];
    cpu.delayed_load();
    cpu.set_reg(i.rd(), s ^ t);
}

// Bitwise logical NOT OR
fn nor(cpu: &mut Cpu, i: Instruction) {
    let s = cpu.regs[i.rs()];
    let t = cpu.regs[i.rt()];
    cpu.delayed_load();
    cpu.set_reg(i.rd(), !(s | t));
}

// Bitwise logical OR with a constant
fn ori(cpu: &mut Cpu, i: Instruction) {
    let s = cpu.regs[i.rs()];
    let imm = i.imm();
    cpu.delayed_load();
    cpu.set_reg(i.rt(), s | imm);
}

// Bitwise logical exclusive OR with a constant
fn xori(cpu: &mut Cpu, i: Instruction) {
    let s = cpu.regs[i.rs()];
    let imm = i.imm();
    cpu.delayed_load();
    cpu.set_reg(i.rt(), s ^ imm);
}

#[derive(Copy, Clone)]
pub struct Instruction(u32);

impl Instruction {
    pub fn new(i: u32) -> Self {
        Self(i)
    }

    // Primary opcode field
    pub const fn op(self) -> usize {
        ((self.0 >> 26) & 0x3f) as usize
    }

    // Secondary opcode field
    pub const fn op2(self) -> usize {
        (self.0 & 0x1f) as usize
    }

    // Source register or base
    pub const fn rs(self) -> usize {
        ((self.0 >> 21) & 0x1f) as usize
    }

    // Source register
    pub const fn rt(self) -> usize {
        ((self.0 >> 16) & 0x1f) as usize
    }

    // Source register
    pub const fn rd(self) -> usize {
        ((self.0 >> 11) & 0x1f) as usize
    }

    // Shift immediate values
    pub const fn sa(self) -> u32 {
        (self.0 >> 6) & 0x1f
    }

    // 16-bit signed immediate value
    pub const fn simm(self) -> u32 {
        ((self.0 & 0xffff) as i16) as u32
    }

    // 16-bit zero-extended to the left
    pub const fn imm(self) -> u32 {
        self.0 & 0xfff
    }

    // Immediate value for jump instructions
    pub const fn jimm(self) -> u32 {
        (self.0 & 0x03ffffff) << 2
    }
}

const REG_NAMES: [&'static str; 32] = [
    "r0", "at", "v0", "v1", "a0", "a1", "a2", "a3", "t0", "t1", "t2", "t3", "t4", "t5", "t6", "t7",
    "s0", "s1", "s2", "s3", "s4", "s5", "s7", "s7", "t8", "t9", "k0", "k1", "gp", "sp", "fp", "ra",
];
