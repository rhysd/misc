pub trait Addressable {
    fn read(&self, idx: usize) -> u8;
    fn write(&mut self, idx: usize, val: u8);
}

pub struct Memory1 {
    pub v: Vec<u8>,
}

impl Addressable for Memory1 {
    fn read(&self, idx: usize) -> u8 {
        self.v[idx]
    }
    fn write(&mut self, idx: usize, val: u8) {
        self.v[idx] = val;
    }
}

pub struct Memory2 {
    v: Vec<u8>,
    read_fn: fn(&[u8], idx: usize) -> u8,
    write_fn: fn(&mut [u8], idx: usize, val: u8),
}

fn read1(s: &[u8], idx: usize) -> u8 {
    s[idx]
}
fn write1(s: &mut [u8], idx: usize, val: u8) {
    s[idx] = val;
}

impl Memory2 {
    pub fn new(v: Vec<u8>) -> Self {
        Self {
            v,
            read_fn: read1,
            write_fn: write1,
        }
    }

    #[inline]
    pub fn read(&self, idx: usize) -> u8 {
        (self.read_fn)(&self.v, idx)
    }

    #[inline]
    pub fn write(&mut self, idx: usize, val: u8) {
        (self.write_fn)(&mut self.v, idx, val);
    }
}

pub trait Addressable2 {
    fn read(&self, mem: &[u8], idx: usize) -> u8;
    fn write(&self, mem: &mut [u8], idx: usize, val: u8);
}

pub struct Access;

impl Addressable2 for Access {
    fn read(&self, s: &[u8], idx: usize) -> u8 {
        s[idx]
    }
    fn write(&self, s: &mut [u8], idx: usize, val: u8) {
        s[idx] = val;
    }
}

pub struct Memory3 {
    pub v: Vec<u8>,
    pub a: Box<dyn Addressable2>,
}

impl Memory3 {
    pub fn read(&self, idx: usize) -> u8 {
        self.a.read(&self.v, idx)
    }
    pub fn write(&mut self, idx: usize, val: u8) {
        self.a.write(&mut self.v, idx, val);
    }
}

pub struct MemoryBase {
    pub v: Vec<u8>,
}

impl MemoryBase {
    pub fn read(&self, idx: usize) -> u8 {
        self.v[idx]
    }
    pub fn write(&mut self, idx: usize, val: u8) {
        self.v[idx] = val;
    }
}

pub struct MemoryInline {
    pub v: Vec<u8>,
}

impl MemoryInline {
    #[inline]
    pub fn read(&self, idx: usize) -> u8 {
        self.v[idx]
    }
    #[inline]
    pub fn write(&mut self, idx: usize, val: u8) {
        self.v[idx] = val;
    }
}
