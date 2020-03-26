#![feature(test)]

extern crate test;

#[cfg(test)]
mod tests {
    use std::io;
    use std::io::BufWriter;
    use std::io::Write;
    use std::iter;
    use test::Bencher;

    fn gen_inputs() -> impl Iterator<Item = u8> {
        iter::repeat(b'a').take(1024 * 256)
    }

    #[bench]
    fn raw_output(b: &mut Bencher) {
        b.iter(move || {
            let mut stderr = io::stderr();
            let mut buf = [0u8];
            for b in gen_inputs() {
                buf[0] = b;
                stderr.write(&buf).unwrap();
            }
        });
    }

    #[bench]
    fn lock_output(b: &mut Bencher) {
        b.iter(move || {
            let mut stderr = io::stderr();
            let mut stderr = stderr.lock();
            let mut buf = [0u8];
            for b in gen_inputs() {
                buf[0] = b;
                stderr.write(&buf).unwrap();
            }
        });
    }

    #[bench]
    fn buf_output(b: &mut Bencher) {
        b.iter(move || {
            let mut stderr = BufWriter::new(io::stderr());
            let mut buf = [0u8];
            for b in gen_inputs() {
                buf[0] = b;
                stderr.write(&buf).unwrap();
            }
            stderr.flush().unwrap();
        });
    }

    #[bench]
    fn buf_and_lock_output(b: &mut Bencher) {
        b.iter(move || {
            let mut stderr = io::stderr();
            let mut stderr = BufWriter::new(stderr);
            let mut buf = [0u8];
            for b in gen_inputs() {
                buf[0] = b;
                stderr.write(&buf).unwrap();
            }
            stderr.flush().unwrap();
        });
    }
}
