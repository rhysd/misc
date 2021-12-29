use std::fmt;

// Line comment

/*
 * Block comment
 */

fn main() {
    /*
     * Print
     */
    {
        println!("Hello, world!");
        println!("{} days", 31);
        println!("{0} is not {1}, {1} is not {0}", 42, 0);
        let days = 31;
        println!("DEBUG: {:?}", "foo bar");
        println!("{days} days", days=days);
        println!("{average:>width$}\n{total:>width$}", average=10, total=100, width=5);
    }

    /*
     * Debug
     */
    {
        #[allow(dead_code)]
        struct Structure(i32);

        #[derive(Debug)]
        struct DebugPrintable(i32, u32);

        #[derive(Debug)]
        struct Deep(DebugPrintable);

        println!("DEBUG: {:?}", DebugPrintable(32, 10u32));
        println!("DEBUG: {:?}", Deep(DebugPrintable(-1, 1)));
    }

    /*
     * Display
     */
    {
        struct S(i32);
        impl fmt::Display for S {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{} is a first member of S", self.0)
            }
        }
        println!("{}", S(42));

        #[derive(Debug)]
        struct MinMax(i64, i64);

        impl fmt::Display for MinMax {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
                write!(f, "({}, {})", self.0, self.1)
            }
        }

        #[derive(Debug)]
        struct Point2d {
            x: f64,
            y: f64,
        }

        impl fmt::Display for Point2d {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "(x, y) = ({}, {})", self.x, self.y)
            }
        }

        let mm = MinMax(0, 14);
        println!("Display: {}", mm);
        println!("Debug: {:?}", mm);

        let p = Point2d{x: 3.3, y: 7.2};
        println!("Display: {}", p);
        println!("Debug: {:?}", p);

        struct List(Vec<i32>);

        impl fmt::Display for List {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                let List(ref vec) = *self;
                try!(write!(f, "["));

                for (count, v) in vec.iter().enumerate() {
                    if count != 0 {
                        try!(write!(f, ", "));
                    }
                    try!(write!(f, "{}", v));
                }

                write!(f, "]")
            }
        }

        let l = List(vec![1, 2, 3]);
        println!("{}", l);
    }
}
