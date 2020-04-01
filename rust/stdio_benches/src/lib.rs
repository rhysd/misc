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
        iter::repeat(b'a')
            .take(512)
            .chain(iter::once(b'\n'))
            .cycle()
            .take(512 * 8)
    }

    #[bench]
    fn raw_output(b: &mut Bencher) {
        b.iter(move || {
            let mut stdout = io::stdout();
            let mut buf = [0u8];
            for b in gen_inputs() {
                buf[0] = b;
                stdout.write(&buf).unwrap();
            }
        });
    }

    #[bench]
    fn lock_output(b: &mut Bencher) {
        b.iter(move || {
            let stdout = io::stdout();
            let mut stdout = stdout.lock();
            let mut buf = [0u8];
            for b in gen_inputs() {
                buf[0] = b;
                stdout.write(&buf).unwrap();
            }
        });
    }

    #[bench]
    fn buf_output(b: &mut Bencher) {
        b.iter(move || {
            let mut stdout = BufWriter::new(io::stdout());
            let mut buf = [0u8];
            for b in gen_inputs() {
                buf[0] = b;
                stdout.write(&buf).unwrap();
            }
            stdout.flush().unwrap();
        });
    }

    #[bench]
    fn buf_and_lock_output(b: &mut Bencher) {
        b.iter(move || {
            let stdout = io::stdout();
            let mut stdout = BufWriter::new(stdout);
            let mut buf = [0u8];
            for b in gen_inputs() {
                buf[0] = b;
                stdout.write(&buf).unwrap();
            }
            stdout.flush().unwrap();
        });
    }
}
