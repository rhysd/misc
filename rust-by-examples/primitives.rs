use std::mem;

fn analyze_slice(slice: &[i32]) {
    println!("first element: {}", slice[0]);
    println!("slice length: {}", slice.len());
}

fn main() {
    {
        let b: bool = true;
        let f: f64 = 3.14;
        let mut f2 = 3.14;
        println!("{}", f2);
        f2 = 7.28;
        let c: char = '🍣';

        let a = [1, 2, 3];
        println!("{:?}", a);
        println!("{}", a.len());

        let t = (false, 3.14);
        println!("{}, {}, {}, {:?}, {:?}, {:?}", b, f, f2, c, a, t);
    }

    {
        println!("{}", 1 - 2);
        println!("{} {}", true && false, !true);
        println!("{:04b} {:04b} {:04b}", 0b0011 & 0b0010, 0b1100 | 0b0011, 0b1010 ^ 0b1100);
        println!("{}", 1_000_000u32);
    }

    {
        let t = (1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, -4i64, 0.1f32, 0.2f64, 'a', true);
        println!("{} {} {:?}", t.0, t.9, t);

        let tt = (1u8, (2u16, 3u32, 4u64, -1i8), (-2i16, -3i32, -4i64), (0.1f32, 0.2f64, 'a', true));
        println!("{:?} {:?} {:?}", tt.1, (tt.2).1, tt);

        let x = 1;
        let y = 2;
        println!("{} {}", x, y);
        let (y, x) = (x, y);
        println!("{} {}", x, y);

        let t = (1, "hello", 4.5, true);
        let (a, b, c, d) = t;
        println!("{}, {}, {}, {}", a, b, c, d);
    }

    {
        // Array has fixed length
        let a: [i32; 5] = [1, 2, 3, 4, 5];
        println!("{:?}", a);

        let a500: [i32; 500] = [42; 500];

        println!("{}", a500.len());
        println!("{} {}", a[1], a[3]);

        println!("array occupies {} bytes", mem::size_of_val(&a500));
        analyze_slice(&a);
        analyze_slice(&a[1..3]);
        analyze_slice(&a[..3]);
        analyze_slice(&a[1..]);

        // Boundary error
        // println!("{}", a[10]);
    }
}
