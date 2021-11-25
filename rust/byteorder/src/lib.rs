use byteorder::{ByteOrder, LittleEndian};

#[derive(Default)]
pub struct Regs1 {
    af: u16,
    bc: u16,
    de: u16,
    hl: u16,
}

impl Regs1 {
    #[inline]
    pub fn a(&self) -> u8 {
        (self.af >> 8) as u8
    }

    #[inline]
    pub fn b(&self) -> u8 {
        (self.bc >> 8) as u8
    }

    #[inline]
    pub fn c(&self) -> u8 {
        self.bc as u8
    }

    #[inline]
    pub fn d(&self) -> u8 {
        (self.de >> 8) as u8
    }

    #[inline]
    pub fn e(&self) -> u8 {
        self.de as u8
    }

    #[inline]
    pub fn h(&self) -> u8 {
        (self.hl >> 8) as u8
    }

    #[inline]
    pub fn l(&self) -> u8 {
        self.hl as u8
    }

    #[inline]
    pub fn bc(&self) -> u16 {
        self.bc
    }

    #[inline]
    pub fn de(&self) -> u16 {
        self.de
    }

    #[inline]
    pub fn hl(&self) -> u16 {
        self.hl
    }

    #[inline]
    pub fn set_a(&mut self, x: u8) {
        self.af = ((x as u16) << 8) | (self.af & 0x00ff);
    }

    #[inline]
    pub fn set_b(&mut self, x: u8) {
        self.bc = ((x as u16) << 8) | (self.bc & 0x00ff);
    }

    #[inline]
    pub fn set_c(&mut self, x: u8) {
        self.bc = (x as u16) | (self.bc & 0xff00)
    }

    #[inline]
    pub fn set_d(&mut self, x: u8) {
        self.de = ((x as u16) << 8) | (self.de & 0x00ff);
    }

    #[inline]
    pub fn set_e(&mut self, x: u8) {
        self.de = (x as u16) | (self.de & 0xff00)
    }

    #[inline]
    pub fn set_h(&mut self, x: u8) {
        self.hl = ((x as u16) << 8) | (self.hl & 0x00ff);
    }

    #[inline]
    pub fn set_l(&mut self, x: u8) {
        self.hl = (x as u16) | (self.hl & 0xff00)
    }

    #[inline]
    pub fn set_bc(&mut self, x: u16) {
        self.bc = x;
    }

    #[inline]
    pub fn set_de(&mut self, x: u16) {
        self.de = x;
    }

    #[inline]
    pub fn set_hl(&mut self, x: u16) {
        self.hl = x;
    }
}

#[derive(Default)]
pub struct Regs2 {
    buf: [u8; 8],
}

impl Regs2 {
    #[inline]
    pub fn a(&self) -> u8 {
        self.buf[0]
    }

    #[inline]
    pub fn b(&self) -> u8 {
        self.buf[2]
    }

    #[inline]
    pub fn c(&self) -> u8 {
        self.buf[3]
    }

    #[inline]
    pub fn d(&self) -> u8 {
        self.buf[4]
    }

    #[inline]
    pub fn e(&self) -> u8 {
        self.buf[5]
    }

    #[inline]
    pub fn h(&self) -> u8 {
        self.buf[6]
    }

    #[inline]
    pub fn l(&self) -> u8 {
        self.buf[7]
    }

    #[inline]
    pub fn bc(&self) -> u16 {
        LittleEndian::read_u16(&self.buf[2..4])
    }

    #[inline]
    pub fn de(&self) -> u16 {
        LittleEndian::read_u16(&self.buf[4..6])
    }

    #[inline]
    pub fn hl(&self) -> u16 {
        LittleEndian::read_u16(&self.buf[6..8])
    }

    #[inline]
    pub fn set_a(&mut self, x: u8) {
        self.buf[0] = x;
    }

    #[inline]
    pub fn set_b(&mut self, x: u8) {
        self.buf[2] = x;
    }

    #[inline]
    pub fn set_c(&mut self, x: u8) {
        self.buf[3] = x;
    }

    #[inline]
    pub fn set_d(&mut self, x: u8) {
        self.buf[4] = x;
    }

    #[inline]
    pub fn set_e(&mut self, x: u8) {
        self.buf[5] = x;
    }

    #[inline]
    pub fn set_h(&mut self, x: u8) {
        self.buf[6] = x;
    }

    #[inline]
    pub fn set_l(&mut self, x: u8) {
        self.buf[7] = x;
    }

    #[inline]
    pub fn set_bc(&mut self, x: u16) {
        LittleEndian::write_u16(&mut self.buf[2..4], x);
    }

    #[inline]
    pub fn set_de(&mut self, x: u16) {
        LittleEndian::write_u16(&mut self.buf[4..6], x);
    }

    #[inline]
    pub fn set_hl(&mut self, x: u16) {
        LittleEndian::write_u16(&mut self.buf[6..8], x);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut r1 = Regs1::default();
        let mut r2 = Regs2::default();

        r1.set_b(0x12);
        assert_eq!(r1.b(), 0x12);
        r2.set_b(0x12);
        assert_eq!(r2.b(), 0x12);
        r1.set_c(0x12);
        assert_eq!(r1.c(), 0x12);
        r2.set_c(0x12);
        assert_eq!(r2.c(), 0x12);
        r1.set_d(0x12);
        assert_eq!(r1.d(), 0x12);
        r2.set_d(0x12);
        assert_eq!(r2.d(), 0x12);
        r1.set_e(0x12);
        assert_eq!(r1.e(), 0x12);
        r2.set_e(0x12);
        assert_eq!(r2.e(), 0x12);
        r1.set_h(0x12);
        assert_eq!(r1.h(), 0x12);
        r2.set_h(0x12);
        assert_eq!(r2.h(), 0x12);
        r1.set_l(0x12);
        assert_eq!(r1.l(), 0x12);
        r2.set_l(0x12);
        assert_eq!(r2.l(), 0x12);
        r1.set_a(0x12);
        assert_eq!(r1.a(), 0x12);
        r2.set_a(0x12);
        assert_eq!(r2.a(), 0x12);

        assert_eq!(r1.bc(), 0x1212);
        assert_eq!(r2.bc(), 0x1212);
        assert_eq!(r1.de(), 0x1212);
        assert_eq!(r2.de(), 0x1212);
        assert_eq!(r1.hl(), 0x1212);
        assert_eq!(r2.hl(), 0x1212);

        r1.set_bc(0xabcd);
        r2.set_bc(0xabcd);
        r1.set_de(0xabcd);
        r2.set_de(0xabcd);
        r1.set_hl(0xabcd);
        r2.set_hl(0xabcd);
        assert_eq!(r1.bc(), 0xabcd);
        assert_eq!(r2.bc(), 0xabcd);
        assert_eq!(r1.de(), 0xabcd);
        assert_eq!(r2.de(), 0xabcd);
        assert_eq!(r1.hl(), 0xabcd);
        assert_eq!(r2.hl(), 0xabcd);
    }
}
