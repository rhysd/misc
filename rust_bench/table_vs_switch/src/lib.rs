use std::cmp::max;

#[derive(Clone)]
pub struct Cpu {
    pub a: u8,
    pub b: u8,
    pub pc: usize,
    pub mem: [u8; 0xffff],
}

impl Cpu {
    pub fn new(insns: &[u8]) -> Self {
        let mut mem = [0; 0xffff];
        mem[0..insns.len()].copy_from_slice(insns);
        Self {
            a: 0,
            b: 0,
            pc: 0,
            mem,
        }
    }

    fn load_pc(&mut self) -> u8 {
        let b = self.mem[self.pc];
        self.pc += 1;
        if self.pc > 255 {
            self.pc = 0;
        }
        b
    }

    fn load_addr(&mut self) -> usize {
        let hi = self.load_pc() as usize;
        let lo = self.load_pc() as usize;
        (hi << 8) | lo
    }

    fn load_mem(&mut self) -> u8 {
        let addr = self.load_addr();
        self.mem[addr]
    }

    fn jump(&mut self, addr: usize) {
        self.pc = max(addr, 255);
    }

    fn store(&mut self, addr: usize, val: u8) {
        self.mem[addr] = val;
    }

    pub fn step_switch(&mut self) {
        let op = self.load_pc();
        eval_switch(op, self);
    }

    pub fn step_table(&mut self) {
        let op = self.load_pc();
        eval_table(op, self);
    }

    pub fn step_switch_hi_lo(&mut self) {
        let op = self.load_pc();
        eval_switch_hi_lo(op, self);
    }
}

fn nop(_: &mut Cpu) {}

fn load_a_mem(cpu: &mut Cpu) {
    cpu.a = cpu.load_mem()
}

fn load_b_mem(cpu: &mut Cpu) {
    cpu.b = cpu.load_mem()
}

fn load_a_b(cpu: &mut Cpu) {
    cpu.a = cpu.b
}

fn load_b_a(cpu: &mut Cpu) {
    cpu.b = cpu.a
}

fn load_a_const(cpu: &mut Cpu) {
    cpu.a = cpu.load_pc()
}

fn load_b_const(cpu: &mut Cpu) {
    cpu.b = cpu.load_pc()
}

fn store_a(cpu: &mut Cpu) {
    let addr = cpu.load_addr();
    cpu.store(addr, cpu.a);
}

fn store_b(cpu: &mut Cpu) {
    let addr = cpu.load_addr();
    cpu.store(addr, cpu.b);
}

fn store_a_i(cpu: &mut Cpu) {
    let hi = cpu.load_mem() as usize;
    let lo = cpu.load_mem() as usize;
    cpu.store((hi << 8) | lo, cpu.a);
}

fn store_b_i(cpu: &mut Cpu) {
    let hi = cpu.load_mem() as usize;
    let lo = cpu.load_mem() as usize;
    cpu.store((hi << 8) | lo, cpu.b);
}

fn add(cpu: &mut Cpu) {
    cpu.a = cpu.a.wrapping_add(cpu.b);
}

fn sub(cpu: &mut Cpu) {
    cpu.a = cpu.a.wrapping_sub(cpu.b);
}

fn mul(cpu: &mut Cpu) {
    cpu.a = cpu.a.wrapping_mul(cpu.b);
}

fn div(cpu: &mut Cpu) {
    if cpu.b != 0 {
        cpu.a = cpu.a.wrapping_div(cpu.b);
    }
}

fn rem(cpu: &mut Cpu) {
    if cpu.b != 0 {
        cpu.a = cpu.a.wrapping_rem(cpu.b);
    }
}

fn shl(cpu: &mut Cpu) {
    cpu.a = cpu.a.wrapping_shl(cpu.b as u32);
}

fn shr(cpu: &mut Cpu) {
    cpu.a = cpu.a.wrapping_shr(cpu.b as u32);
}

fn sadd(cpu: &mut Cpu) {
    cpu.a = (cpu.a as i8).wrapping_add(cpu.b as i8) as u8;
}

fn ssub(cpu: &mut Cpu) {
    cpu.a = (cpu.a as i8).wrapping_sub(cpu.b as i8) as u8;
}

fn smul(cpu: &mut Cpu) {
    cpu.a = (cpu.a as i8).wrapping_mul(cpu.b as i8) as u8;
}

fn sdiv(cpu: &mut Cpu) {
    if cpu.b != 0 {
        cpu.a = (cpu.a as i8).wrapping_div(cpu.b as i8) as u8;
    }
}

fn srem(cpu: &mut Cpu) {
    if cpu.b != 0 {
        cpu.a = (cpu.a as i8).wrapping_rem(cpu.b as i8) as u8;
    }
}

fn sshr(cpu: &mut Cpu) {
    cpu.a = (cpu.a as i8).wrapping_shr(cpu.b as u32) as u8;
}

fn inc(cpu: &mut Cpu) {
    cpu.a = cpu.a.wrapping_add(1);
}

fn dec(cpu: &mut Cpu) {
    cpu.a = cpu.a.wrapping_sub(1);
}

fn jpc(cpu: &mut Cpu) {
    let addr = cpu.load_addr();
    cpu.jump(addr);
}

fn jpr(cpu: &mut Cpu) {
    let hi = cpu.load_mem() as usize;
    let lo = cpu.load_mem() as usize;
    cpu.jump((hi << 8) | lo);
}

const INSN_TABLE: [fn(&mut Cpu); 256] = [
    nop,
    load_b_mem,
    nop,
    store_b,
    store_a_i,
    store_b,
    sdiv,
    sdiv,
    load_b_a,
    ssub,
    load_b_a,
    sub,
    dec,
    shr,
    smul,
    ssub,
    load_a_const,
    load_b_mem,
    mul,
    store_b,
    dec,
    jpr,
    load_b_a,
    shl,
    mul,
    load_a_mem,
    div,
    load_a_b,
    jpr,
    div,
    nop,
    store_b_i,
    sub,
    jpr,
    sshr,
    srem,
    sadd,
    ssub,
    srem,
    store_b_i,
    sdiv,
    mul,
    store_a_i,
    inc,
    store_a,
    div,
    load_b_mem,
    store_b_i,
    load_b_mem,
    shl,
    store_a,
    rem,
    load_b_mem,
    load_a_const,
    mul,
    smul,
    store_b_i,
    sdiv,
    smul,
    load_a_mem,
    store_b_i,
    load_a_const,
    add,
    load_b_const,
    store_a,
    jpc,
    jpc,
    store_b_i,
    shl,
    smul,
    sdiv,
    dec,
    add,
    sdiv,
    shr,
    store_a_i,
    smul,
    load_b_const,
    shr,
    load_a_b,
    load_a_const,
    load_a_mem,
    load_a_b,
    dec,
    store_a_i,
    mul,
    sadd,
    jpc,
    shr,
    sshr,
    jpr,
    nop,
    add,
    rem,
    store_a,
    dec,
    add,
    load_b_a,
    store_a_i,
    mul,
    div,
    inc,
    jpr,
    inc,
    store_b,
    load_b_const,
    sshr,
    ssub,
    sshr,
    smul,
    load_b_a,
    load_b_mem,
    shl,
    inc,
    load_b_const,
    ssub,
    srem,
    srem,
    sadd,
    nop,
    load_b_const,
    sdiv,
    load_a_b,
    load_b_const,
    sub,
    sadd,
    srem,
    load_a_mem,
    load_b_mem,
    inc,
    sadd,
    jpc,
    store_b_i,
    load_a_const,
    dec,
    load_a_b,
    div,
    store_b_i,
    rem,
    sadd,
    load_b_mem,
    jpr,
    add,
    inc,
    load_a_mem,
    rem,
    load_a_mem,
    mul,
    store_a,
    load_a_const,
    srem,
    load_a_const,
    store_a_i,
    load_b_const,
    sshr,
    sdiv,
    add,
    jpc,
    sadd,
    load_b_a,
    srem,
    nop,
    add,
    load_b_const,
    sadd,
    load_a_mem,
    ssub,
    load_b_mem,
    load_a_b,
    store_b_i,
    sdiv,
    load_a_const,
    shl,
    dec,
    add,
    load_b_mem,
    store_a_i,
    sub,
    jpc,
    load_a_const,
    rem,
    store_a,
    ssub,
    add,
    nop,
    rem,
    load_a_b,
    jpc,
    store_a,
    shr,
    store_b,
    jpc,
    shl,
    smul,
    load_b_a,
    nop,
    shr,
    sshr,
    jpr,
    shr,
    nop,
    inc,
    shl,
    smul,
    dec,
    sub,
    store_b,
    sadd,
    ssub,
    ssub,
    sub,
    load_b_a,
    sshr,
    dec,
    store_a,
    srem,
    shr,
    rem,
    load_a_b,
    jpr,
    rem,
    shr,
    div,
    store_b,
    store_a_i,
    smul,
    store_a,
    load_a_mem,
    shl,
    sshr,
    inc,
    mul,
    div,
    div,
    sshr,
    load_b_const,
    store_b,
    rem,
    load_a_b,
    sub,
    store_a_i,
    sub,
    load_a_b,
    sub,
    load_a_mem,
    store_b,
    load_a_mem,
    nop,
    srem,
    jpc,
    inc,
    shl,
    mul,
    div,
    jpr,
    load_b_a,
];

fn eval_table(op: u8, cpu: &mut Cpu) {
    INSN_TABLE[op as usize](cpu);
}

fn eval_switch(op: u8, cpu: &mut Cpu) {
    match op {
        0x00 => nop(cpu),
        0x01 => load_b_mem(cpu),
        0x02 => nop(cpu),
        0x03 => store_b(cpu),
        0x04 => store_a_i(cpu),
        0x05 => store_b(cpu),
        0x06 => sdiv(cpu),
        0x07 => sdiv(cpu),
        0x08 => load_b_a(cpu),
        0x09 => ssub(cpu),
        0x0a => load_b_a(cpu),
        0x0b => sub(cpu),
        0x0c => dec(cpu),
        0x0d => shr(cpu),
        0x0e => smul(cpu),
        0x0f => ssub(cpu),
        0x10 => load_a_const(cpu),
        0x11 => load_b_mem(cpu),
        0x12 => mul(cpu),
        0x13 => store_b(cpu),
        0x14 => dec(cpu),
        0x15 => jpr(cpu),
        0x16 => load_b_a(cpu),
        0x17 => shl(cpu),
        0x18 => mul(cpu),
        0x19 => load_a_mem(cpu),
        0x1a => div(cpu),
        0x1b => load_a_b(cpu),
        0x1c => jpr(cpu),
        0x1d => div(cpu),
        0x1e => nop(cpu),
        0x1f => store_b_i(cpu),
        0x20 => sub(cpu),
        0x21 => jpr(cpu),
        0x22 => sshr(cpu),
        0x23 => srem(cpu),
        0x24 => sadd(cpu),
        0x25 => ssub(cpu),
        0x26 => srem(cpu),
        0x27 => store_b_i(cpu),
        0x28 => sdiv(cpu),
        0x29 => mul(cpu),
        0x2a => store_a_i(cpu),
        0x2b => inc(cpu),
        0x2c => store_a(cpu),
        0x2d => div(cpu),
        0x2e => load_b_mem(cpu),
        0x2f => store_b_i(cpu),
        0x30 => load_b_mem(cpu),
        0x31 => shl(cpu),
        0x32 => store_a(cpu),
        0x33 => rem(cpu),
        0x34 => load_b_mem(cpu),
        0x35 => load_a_const(cpu),
        0x36 => mul(cpu),
        0x37 => smul(cpu),
        0x38 => store_b_i(cpu),
        0x39 => sdiv(cpu),
        0x3a => smul(cpu),
        0x3b => load_a_mem(cpu),
        0x3c => store_b_i(cpu),
        0x3d => load_a_const(cpu),
        0x3e => add(cpu),
        0x3f => load_b_const(cpu),
        0x40 => store_a(cpu),
        0x41 => jpc(cpu),
        0x42 => jpc(cpu),
        0x43 => store_b_i(cpu),
        0x44 => shl(cpu),
        0x45 => smul(cpu),
        0x46 => sdiv(cpu),
        0x47 => dec(cpu),
        0x48 => add(cpu),
        0x49 => sdiv(cpu),
        0x4a => shr(cpu),
        0x4b => store_a_i(cpu),
        0x4c => smul(cpu),
        0x4d => load_b_const(cpu),
        0x4e => shr(cpu),
        0x4f => load_a_b(cpu),
        0x50 => load_a_const(cpu),
        0x51 => load_a_mem(cpu),
        0x52 => load_a_b(cpu),
        0x53 => dec(cpu),
        0x54 => store_a_i(cpu),
        0x55 => mul(cpu),
        0x56 => sadd(cpu),
        0x57 => jpc(cpu),
        0x58 => shr(cpu),
        0x59 => sshr(cpu),
        0x5a => jpr(cpu),
        0x5b => nop(cpu),
        0x5c => add(cpu),
        0x5d => rem(cpu),
        0x5e => store_a(cpu),
        0x5f => dec(cpu),
        0x60 => add(cpu),
        0x61 => load_b_a(cpu),
        0x62 => store_a_i(cpu),
        0x63 => mul(cpu),
        0x64 => div(cpu),
        0x65 => inc(cpu),
        0x66 => jpr(cpu),
        0x67 => inc(cpu),
        0x68 => store_b(cpu),
        0x69 => load_b_const(cpu),
        0x6a => sshr(cpu),
        0x6b => ssub(cpu),
        0x6c => sshr(cpu),
        0x6d => smul(cpu),
        0x6e => load_b_a(cpu),
        0x6f => load_b_mem(cpu),
        0x70 => shl(cpu),
        0x71 => inc(cpu),
        0x72 => load_b_const(cpu),
        0x73 => ssub(cpu),
        0x74 => srem(cpu),
        0x75 => srem(cpu),
        0x76 => sadd(cpu),
        0x77 => nop(cpu),
        0x78 => load_b_const(cpu),
        0x79 => sdiv(cpu),
        0x7a => load_a_b(cpu),
        0x7b => load_b_const(cpu),
        0x7c => sub(cpu),
        0x7d => sadd(cpu),
        0x7e => srem(cpu),
        0x7f => load_a_mem(cpu),
        0x80 => load_b_mem(cpu),
        0x81 => inc(cpu),
        0x82 => sadd(cpu),
        0x83 => jpc(cpu),
        0x84 => store_b_i(cpu),
        0x85 => load_a_const(cpu),
        0x86 => dec(cpu),
        0x87 => load_a_b(cpu),
        0x88 => div(cpu),
        0x89 => store_b_i(cpu),
        0x8a => rem(cpu),
        0x8b => sadd(cpu),
        0x8c => load_b_mem(cpu),
        0x8d => jpr(cpu),
        0x8e => add(cpu),
        0x8f => inc(cpu),
        0x90 => load_a_mem(cpu),
        0x91 => rem(cpu),
        0x92 => load_a_mem(cpu),
        0x93 => mul(cpu),
        0x94 => store_a(cpu),
        0x95 => load_a_const(cpu),
        0x96 => srem(cpu),
        0x97 => load_a_const(cpu),
        0x98 => store_a_i(cpu),
        0x99 => load_b_const(cpu),
        0x9a => sshr(cpu),
        0x9b => sdiv(cpu),
        0x9c => add(cpu),
        0x9d => jpc(cpu),
        0x9e => sadd(cpu),
        0x9f => load_b_a(cpu),
        0xa0 => srem(cpu),
        0xa1 => nop(cpu),
        0xa2 => add(cpu),
        0xa3 => load_b_const(cpu),
        0xa4 => sadd(cpu),
        0xa5 => load_a_mem(cpu),
        0xa6 => ssub(cpu),
        0xa7 => load_b_mem(cpu),
        0xa8 => load_a_b(cpu),
        0xa9 => store_b_i(cpu),
        0xaa => sdiv(cpu),
        0xab => load_a_const(cpu),
        0xac => shl(cpu),
        0xad => dec(cpu),
        0xae => add(cpu),
        0xaf => load_b_mem(cpu),
        0xb0 => store_a_i(cpu),
        0xb1 => sub(cpu),
        0xb2 => jpc(cpu),
        0xb3 => load_a_const(cpu),
        0xb4 => rem(cpu),
        0xb5 => store_a(cpu),
        0xb6 => ssub(cpu),
        0xb7 => add(cpu),
        0xb8 => nop(cpu),
        0xb9 => rem(cpu),
        0xba => load_a_b(cpu),
        0xbb => jpc(cpu),
        0xbc => store_a(cpu),
        0xbd => shr(cpu),
        0xbe => store_b(cpu),
        0xbf => jpc(cpu),
        0xc0 => shl(cpu),
        0xc1 => smul(cpu),
        0xc2 => load_b_a(cpu),
        0xc3 => nop(cpu),
        0xc4 => shr(cpu),
        0xc5 => sshr(cpu),
        0xc6 => jpr(cpu),
        0xc7 => shr(cpu),
        0xc8 => nop(cpu),
        0xc9 => inc(cpu),
        0xca => shl(cpu),
        0xcb => smul(cpu),
        0xcc => dec(cpu),
        0xcd => sub(cpu),
        0xce => store_b(cpu),
        0xcf => sadd(cpu),
        0xd0 => ssub(cpu),
        0xd1 => ssub(cpu),
        0xd2 => sub(cpu),
        0xd3 => load_b_a(cpu),
        0xd4 => sshr(cpu),
        0xd5 => dec(cpu),
        0xd6 => store_a(cpu),
        0xd7 => srem(cpu),
        0xd8 => shr(cpu),
        0xd9 => rem(cpu),
        0xda => load_a_b(cpu),
        0xdb => jpr(cpu),
        0xdc => rem(cpu),
        0xdd => shr(cpu),
        0xde => div(cpu),
        0xdf => store_b(cpu),
        0xe0 => store_a_i(cpu),
        0xe1 => smul(cpu),
        0xe2 => store_a(cpu),
        0xe3 => load_a_mem(cpu),
        0xe4 => shl(cpu),
        0xe5 => sshr(cpu),
        0xe6 => inc(cpu),
        0xe7 => mul(cpu),
        0xe8 => div(cpu),
        0xe9 => div(cpu),
        0xea => sshr(cpu),
        0xeb => load_b_const(cpu),
        0xec => store_b(cpu),
        0xed => rem(cpu),
        0xee => load_a_b(cpu),
        0xef => sub(cpu),
        0xf0 => store_a_i(cpu),
        0xf1 => sub(cpu),
        0xf2 => load_a_b(cpu),
        0xf3 => sub(cpu),
        0xf4 => load_a_mem(cpu),
        0xf5 => store_b(cpu),
        0xf6 => load_a_mem(cpu),
        0xf7 => nop(cpu),
        0xf8 => srem(cpu),
        0xf9 => jpc(cpu),
        0xfa => inc(cpu),
        0xfb => shl(cpu),
        0xfc => mul(cpu),
        0xfd => div(cpu),
        0xfe => jpr(cpu),
        0xff => load_b_a(cpu),
    }
}

fn eval_switch_hi_lo(op: u8, cpu: &mut Cpu) {
    match op >> 4 {
        0x0 => match op & 0xf {
            0x0 => nop(cpu),
            0x1 => load_b_mem(cpu),
            0x2 => nop(cpu),
            0x3 => store_b(cpu),
            0x4 => store_a_i(cpu),
            0x5 => store_b(cpu),
            0x6 => sdiv(cpu),
            0x7 => sdiv(cpu),
            0x8 => load_b_a(cpu),
            0x9 => ssub(cpu),
            0xa => load_b_a(cpu),
            0xb => sub(cpu),
            0xc => dec(cpu),
            0xd => shr(cpu),
            0xe => smul(cpu),
            0xf => ssub(cpu),
            _ => unreachable!(),
        },
        0x1 => match op & 0xf {
            0x0 => load_a_const(cpu),
            0x1 => load_b_mem(cpu),
            0x2 => mul(cpu),
            0x3 => store_b(cpu),
            0x4 => dec(cpu),
            0x5 => jpr(cpu),
            0x6 => load_b_a(cpu),
            0x7 => shl(cpu),
            0x8 => mul(cpu),
            0x9 => load_a_mem(cpu),
            0xa => div(cpu),
            0xb => load_a_b(cpu),
            0xc => jpr(cpu),
            0xd => div(cpu),
            0xe => nop(cpu),
            0xf => store_b_i(cpu),
            _ => unreachable!(),
        },
        0x2 => match op & 0xf {
            0x0 => sub(cpu),
            0x1 => jpr(cpu),
            0x2 => sshr(cpu),
            0x3 => srem(cpu),
            0x4 => sadd(cpu),
            0x5 => ssub(cpu),
            0x6 => srem(cpu),
            0x7 => store_b_i(cpu),
            0x8 => sdiv(cpu),
            0x9 => mul(cpu),
            0xa => store_a_i(cpu),
            0xb => inc(cpu),
            0xc => store_a(cpu),
            0xd => div(cpu),
            0xe => load_b_mem(cpu),
            0xf => store_b_i(cpu),
            _ => unreachable!(),
        },
        0x3 => match op & 0xf {
            0x0 => load_b_mem(cpu),
            0x1 => shl(cpu),
            0x2 => store_a(cpu),
            0x3 => rem(cpu),
            0x4 => load_b_mem(cpu),
            0x5 => load_a_const(cpu),
            0x6 => mul(cpu),
            0x7 => smul(cpu),
            0x8 => store_b_i(cpu),
            0x9 => sdiv(cpu),
            0xa => smul(cpu),
            0xb => load_a_mem(cpu),
            0xc => store_b_i(cpu),
            0xd => load_a_const(cpu),
            0xe => add(cpu),
            0xf => load_b_const(cpu),
            _ => unreachable!(),
        },
        0x4 => match op & 0xf {
            0x0 => store_a(cpu),
            0x1 => jpc(cpu),
            0x2 => jpc(cpu),
            0x3 => store_b_i(cpu),
            0x4 => shl(cpu),
            0x5 => smul(cpu),
            0x6 => sdiv(cpu),
            0x7 => dec(cpu),
            0x8 => add(cpu),
            0x9 => sdiv(cpu),
            0xa => shr(cpu),
            0xb => store_a_i(cpu),
            0xc => smul(cpu),
            0xd => load_b_const(cpu),
            0xe => shr(cpu),
            0xf => load_a_b(cpu),
            _ => unreachable!(),
        },
        0x5 => match op & 0xf {
            0x0 => load_a_const(cpu),
            0x1 => load_a_mem(cpu),
            0x2 => load_a_b(cpu),
            0x3 => dec(cpu),
            0x4 => store_a_i(cpu),
            0x5 => mul(cpu),
            0x6 => sadd(cpu),
            0x7 => jpc(cpu),
            0x8 => shr(cpu),
            0x9 => sshr(cpu),
            0xa => jpr(cpu),
            0xb => nop(cpu),
            0xc => add(cpu),
            0xd => rem(cpu),
            0xe => store_a(cpu),
            0xf => dec(cpu),
            _ => unreachable!(),
        },
        0x6 => match op & 0xf {
            0x0 => add(cpu),
            0x1 => load_b_a(cpu),
            0x2 => store_a_i(cpu),
            0x3 => mul(cpu),
            0x4 => div(cpu),
            0x5 => inc(cpu),
            0x6 => jpr(cpu),
            0x7 => inc(cpu),
            0x8 => store_b(cpu),
            0x9 => load_b_const(cpu),
            0xa => sshr(cpu),
            0xb => ssub(cpu),
            0xc => sshr(cpu),
            0xd => smul(cpu),
            0xe => load_b_a(cpu),
            0xf => load_b_mem(cpu),
            _ => unreachable!(),
        },
        0x7 => match op & 0xf {
            0x0 => shl(cpu),
            0x1 => inc(cpu),
            0x2 => load_b_const(cpu),
            0x3 => ssub(cpu),
            0x4 => srem(cpu),
            0x5 => srem(cpu),
            0x6 => sadd(cpu),
            0x7 => nop(cpu),
            0x8 => load_b_const(cpu),
            0x9 => sdiv(cpu),
            0xa => load_a_b(cpu),
            0xb => load_b_const(cpu),
            0xc => sub(cpu),
            0xd => sadd(cpu),
            0xe => srem(cpu),
            0xf => load_a_mem(cpu),
            _ => unreachable!(),
        },
        0x8 => match op & 0xf {
            0x0 => load_b_mem(cpu),
            0x1 => inc(cpu),
            0x2 => sadd(cpu),
            0x3 => jpc(cpu),
            0x4 => store_b_i(cpu),
            0x5 => load_a_const(cpu),
            0x6 => dec(cpu),
            0x7 => load_a_b(cpu),
            0x8 => div(cpu),
            0x9 => store_b_i(cpu),
            0xa => rem(cpu),
            0xb => sadd(cpu),
            0xc => load_b_mem(cpu),
            0xd => jpr(cpu),
            0xe => add(cpu),
            0xf => inc(cpu),
            _ => unreachable!(),
        },
        0x9 => match op & 0xf {
            0x0 => load_a_mem(cpu),
            0x1 => rem(cpu),
            0x2 => load_a_mem(cpu),
            0x3 => mul(cpu),
            0x4 => store_a(cpu),
            0x5 => load_a_const(cpu),
            0x6 => srem(cpu),
            0x7 => load_a_const(cpu),
            0x8 => store_a_i(cpu),
            0x9 => load_b_const(cpu),
            0xa => sshr(cpu),
            0xb => sdiv(cpu),
            0xc => add(cpu),
            0xd => jpc(cpu),
            0xe => sadd(cpu),
            0xf => load_b_a(cpu),
            _ => unreachable!(),
        },
        0xa => match op & 0xf {
            0x0 => srem(cpu),
            0x1 => nop(cpu),
            0x2 => add(cpu),
            0x3 => load_b_const(cpu),
            0x4 => sadd(cpu),
            0x5 => load_a_mem(cpu),
            0x6 => ssub(cpu),
            0x7 => load_b_mem(cpu),
            0x8 => load_a_b(cpu),
            0x9 => store_b_i(cpu),
            0xa => sdiv(cpu),
            0xb => load_a_const(cpu),
            0xc => shl(cpu),
            0xd => dec(cpu),
            0xe => add(cpu),
            0xf => load_b_mem(cpu),
            _ => unreachable!(),
        },
        0xb => match op & 0xf {
            0x0 => store_a_i(cpu),
            0x1 => sub(cpu),
            0x2 => jpc(cpu),
            0x3 => load_a_const(cpu),
            0x4 => rem(cpu),
            0x5 => store_a(cpu),
            0x6 => ssub(cpu),
            0x7 => add(cpu),
            0x8 => nop(cpu),
            0x9 => rem(cpu),
            0xa => load_a_b(cpu),
            0xb => jpc(cpu),
            0xc => store_a(cpu),
            0xd => shr(cpu),
            0xe => store_b(cpu),
            0xf => jpc(cpu),
            _ => unreachable!(),
        },
        0xc => match op & 0xf {
            0x0 => shl(cpu),
            0x1 => smul(cpu),
            0x2 => load_b_a(cpu),
            0x3 => nop(cpu),
            0x4 => shr(cpu),
            0x5 => sshr(cpu),
            0x6 => jpr(cpu),
            0x7 => shr(cpu),
            0x8 => nop(cpu),
            0x9 => inc(cpu),
            0xa => shl(cpu),
            0xb => smul(cpu),
            0xc => dec(cpu),
            0xd => sub(cpu),
            0xe => store_b(cpu),
            0xf => sadd(cpu),
            _ => unreachable!(),
        },
        0xd => match op & 0xf {
            0x0 => ssub(cpu),
            0x1 => ssub(cpu),
            0x2 => sub(cpu),
            0x3 => load_b_a(cpu),
            0x4 => sshr(cpu),
            0x5 => dec(cpu),
            0x6 => store_a(cpu),
            0x7 => srem(cpu),
            0x8 => shr(cpu),
            0x9 => rem(cpu),
            0xa => load_a_b(cpu),
            0xb => jpr(cpu),
            0xc => rem(cpu),
            0xd => shr(cpu),
            0xe => div(cpu),
            0xf => store_b(cpu),
            _ => unreachable!(),
        },
        0xe => match op & 0xf {
            0x0 => store_a_i(cpu),
            0x1 => smul(cpu),
            0x2 => store_a(cpu),
            0x3 => load_a_mem(cpu),
            0x4 => shl(cpu),
            0x5 => sshr(cpu),
            0x6 => inc(cpu),
            0x7 => mul(cpu),
            0x8 => div(cpu),
            0x9 => div(cpu),
            0xa => sshr(cpu),
            0xb => load_b_const(cpu),
            0xc => store_b(cpu),
            0xd => rem(cpu),
            0xe => load_a_b(cpu),
            0xf => sub(cpu),
            _ => unreachable!(),
        },
        0xf => match op & 0xf {
            0x0 => store_a_i(cpu),
            0x1 => sub(cpu),
            0x2 => load_a_b(cpu),
            0x3 => sub(cpu),
            0x4 => load_a_mem(cpu),
            0x5 => store_b(cpu),
            0x6 => load_a_mem(cpu),
            0x7 => nop(cpu),
            0x8 => srem(cpu),
            0x9 => jpc(cpu),
            0xa => inc(cpu),
            0xb => shl(cpu),
            0xc => mul(cpu),
            0xd => div(cpu),
            0xe => jpr(cpu),
            0xf => load_b_a(cpu),
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }
}
