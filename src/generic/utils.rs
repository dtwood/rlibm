use std::mem;

pub trait Transmute {
    type U32TUPLE;

    fn all(self) -> u64;
    fn high_u32(self) -> u32;
    fn low_u32(self) -> u32;
    fn from_u64(u64) -> f64;
    fn from_u32s(Self::U32TUPLE) -> f64;

    fn exponent(self) -> u16;
    fn fraction(self) -> u64;
}

impl Transmute for f64 {
    type U32TUPLE=(u32, u32);

    fn all(self) -> u64 {
        unsafe { mem::transmute(self) }
    }

    fn high_u32(self) -> u32 {
        (self.all() >> 32) as u32
    }

    fn low_u32(self) -> u32 {
        (self.all() & 0xFF) as u32
    }

    fn from_u64(x: u64) -> Self {
        unsafe { mem::transmute(x) }
    }

    fn from_u32s(xs: Self::U32TUPLE) -> Self {
        let (hi, lo) = xs;
        Self::from_u64((hi as u64) << 32 + (lo as u64))
    }

    fn exponent(self) -> u16 {
        (self.all() >> 52 & 0b11111111_111) as u16
    }

    fn fraction(self) -> u64 {
        self.all() & 0b11111111_11111111_11111111_11111111_11111111_11111111_1111
    }
}
