fn main() {
    {
        // Methods

        #[derive(Debug)]
        struct Point {
            x: f64,
            y: f64,
        }

        impl Point {
            // Static methods
            fn origin() -> Point {
                Point {x: 0.0, y: 0.0}
            }

            fn new(x: f64, y: f64) -> Point {
                Point {x: x, y: y}
            }
        }

        let o = Point::origin();
        println!("{:?}", o);

        let p = Point::new(1.0, 2.0);
        println!("{:?}", p);

        #[derive(Debug)]
        struct Rectangle {
            p: Point,
            q: Point,
        }
        impl Rectangle {
            fn area(&self) -> f64 {
                let Point {x: p, y: q} = self.p;
                let Point {x: r, y: s} = self.q;

                ((p - r) * (q - s)).abs()
            }

            fn expand(&self, rate: f64) -> Rectangle {
                Rectangle {
                    p: Point {x: self.p.x * rate, y: self.p.y * rate},
                    q: Point {x: self.q.x * rate, y: self.q.y * rate},
                }
            }

            fn new(x: f64, y: f64, z: f64, w: f64) -> Rectangle {
                Rectangle {
                    p: Point::new(x, y),
                    q: Point::new(z, w),
                }
            }
        }

        let rect = Rectangle::new(1.0, 2.0, 3.0, 4.0);
        println!("{:?}: area is {}, expanded: {:?}", rect, rect.area(), rect.expand(3.0));
    }

    {
        let inc = |i| i + 1;
        let inc_annotated = |i: i32| -> i32 { i + 1 };
        println!("{}, {}", inc(1), inc_annotated(1));

        // Captured
        let inc_inc = |i| inc(inc(i));
        println!("{}", inc_inc(1));

        // No argument
        let answer = || 42;
        println!("{}", answer());
    }
}
