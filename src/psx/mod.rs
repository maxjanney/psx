pub mod cpu;

pub struct Psx {
    pub cpu: cpu::Cpu,
    scratchpad: ScratchPad,
}

// Supported bus widths
pub enum BusWidth {
    Byte = 1,
    Word = 2,
    DoubleWord = 4,
}

// Trait for generic load/stores
pub trait Addressable {
    fn width() -> BusWidth;

    fn from_u32(val: u32) -> Self;

    fn as_u32(&self) -> u32;
}

impl Addressable for u8 {
    fn width() -> BusWidth {
        BusWidth::Byte
    }

    fn from_u32(val: u32) -> Self {
        val as u8
    }

    fn as_u32(&self) -> u32 {
        *self as u32
    }
}

impl Addressable for u16 {
    fn width() -> BusWidth {
        BusWidth::Word
    }

    fn from_u32(val: u32) -> Self {
        val as u16
    }

    fn as_u32(&self) -> u32 {
        *self as u32
    }
}

impl Addressable for u32 {
    fn width() -> BusWidth {
        BusWidth::DoubleWord
    }

    fn from_u32(val: u32) -> Self {
        val
    }

    fn as_u32(&self) -> u32 {
        *self
    }
}

// Scratchpad is 1 KB
const SCRATCHPAD_SIZE: usize = 1024;

struct ScratchPad {
    dat: Box<[u8; SCRATCHPAD_SIZE]>,
}

impl ScratchPad {
    pub fn new() -> Self {
        Self {
            dat: Box::new([0u8; SCRATCHPAD_SIZE]),
        }
    }

    // pub fn load<W: Addressable>(&self, offset: u32) -> T {}

    // pub fn store<W: Addressable>(&mut self, offset: u32, val: T) {}
}
