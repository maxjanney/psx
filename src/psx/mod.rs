pub mod cpu;

pub struct Psx {
    pub cpu: cpu::Cpu,
    scratchpad: ScratchPad,
    // FFFE0130h Cache Control (R/W)
    cache_control: u32,
}

impl Psx {
    pub fn new() -> Self {
        Self {
            cpu: cpu::Cpu::new(),
            scratchpad: ScratchPad::new(),
            cache_control: 0,
        }
    }

    pub fn code_cache_enabled(&self) -> bool {
        self.cache_control & 0x800 != 0
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

    // Read a value from the scratchpad with the given width
    pub fn load<W: Addressable>(&self, offset: u32) -> W {
        let offset = (offset & 0x7fffff) as usize;
        let mut val = 0u32;
        for i in 0..W::WIDTH as usize {
            val |= (self.dat[offset + i] as u32) << (i * 8);
        }
        W::from_u32(val)
    }

    // Write a value to the scratchpad with the given width
    pub fn store<W: Addressable>(&mut self, offset: u32, val: W) {
        let offset = (offset & 0x7fffff) as usize;
        let val = val.as_u32();
        for i in 0..W::WIDTH as usize {
            self.dat[offset + i] = (val >> (i * 8)) as u8;
        }
    }
}

// Supported bus widths
pub enum BusWidth {
    Byte = 1,
    Word = 2,
    DoubleWord = 4,
}

// Trait for generic load/stores
pub trait Addressable {
    const WIDTH: BusWidth;

    fn from_u32(val: u32) -> Self;

    fn as_u32(&self) -> u32;
}

impl Addressable for u8 {
    const WIDTH: BusWidth = BusWidth::Byte;

    fn from_u32(val: u32) -> Self {
        val as u8
    }

    fn as_u32(&self) -> u32 {
        *self as u32
    }
}

impl Addressable for u16 {
    const WIDTH: BusWidth = BusWidth::Word;

    fn from_u32(val: u32) -> Self {
        val as u16
    }

    fn as_u32(&self) -> u32 {
        *self as u32
    }
}

impl Addressable for u32 {
    const WIDTH: BusWidth = BusWidth::DoubleWord;

    fn from_u32(val: u32) -> Self {
        val
    }

    fn as_u32(&self) -> u32 {
        *self
    }
}

pub mod map {
    const REGION_MASKS: [u32; 8] = [
        0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, // KUSEG: 2048MB
        0x7FFFFFFF, // KSEG0: 512MB
        0x1FFFFFFF, // KSEG1: 512MB
        0xFFFFFFFF, 0xFFFFFFFF, // KSEG1: 1024MB
    ];

    pub fn mask(addr: u32) -> u32 {
        addr & REGION_MASKS[(addr >> 29) as usize]
    }
}
