pub trait Addressable {
    fn read(&self, idx: usize) -> u8;
    fn write(&mut self, idx: usize, val: u8);
}

pub struct MemoryDyn {
    pub v: Vec<u8>,
}

impl Addressable for MemoryDyn {
    #[inline]
    fn read(&self, idx: usize) -> u8 {
        self.v[idx]
    }
    #[inline]
    fn write(&mut self, idx: usize, val: u8) {
        self.v[idx] = val;
    }
}

pub fn new_addressable(v: Vec<u8>) -> Box<dyn Addressable> {
    Box::new(MemoryDyn { v })
}

pub struct MemoryFnPtr {
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

impl MemoryFnPtr {
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

pub trait AddressableNoState {
    fn read(&self, mem: &[u8], idx: usize) -> u8;
    fn write(&self, mem: &mut [u8], idx: usize, val: u8);
}

pub struct ReadWrite;

impl AddressableNoState for ReadWrite {
    #[inline]
    fn read(&self, s: &[u8], idx: usize) -> u8 {
        s[idx]
    }
    #[inline]
    fn write(&self, s: &mut [u8], idx: usize, val: u8) {
        s[idx] = val;
    }
}

pub struct MemoryDynNoState {
    pub v: Vec<u8>,
    pub a: Box<dyn AddressableNoState>,
}

impl MemoryDynNoState {
    pub fn new(v: Vec<u8>) -> Self {
        Self {
            v,
            a: Box::new(ReadWrite),
        }
    }
    #[inline]
    pub fn read(&self, idx: usize) -> u8 {
        self.a.read(&self.v, idx)
    }
    #[inline]
    pub fn write(&mut self, idx: usize, val: u8) {
        self.a.write(&mut self.v, idx, val);
    }
}

pub struct MemoryNoInline {
    pub v: Vec<u8>,
}

impl MemoryNoInline {
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
