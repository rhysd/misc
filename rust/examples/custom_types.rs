#[derive(Debug)]
enum Job {
    Engineer,
    Student(i32),  // Tuple like
    Programmer {   // Struct like
        lang: String,
        freelance: bool,
    },
}

fn main() {
    /*
     * Structs
     */
    {
        #[derive(Debug)]
        struct Nil;

        #[derive(Debug)]
        struct NumPair(i32, i32);

        #[derive(Debug)]
        struct Point {
            x: f64,
            y: f64,
        }

        #[derive(Debug)]
        struct Rectangle {
            p1: Point,
            p2: Point,
        }

        let _nil = Nil;
        println!("{:?}", _nil);

        let nums = NumPair(10, 42);
        println!("{:?}", nums);

        let p1 = Point{x: -1.0, y: -1.1};
        println!("DEBUG: p1: {:?}", p1);

        let Point{x: x1, y: y1} = p1;
        println!("{}, {}", x1, y1);

        let Point{x, y} = p1;
        println!("{}, {}", x, y);

        let rect = Rectangle {p1: p1, p2: Point{x: 3.14, y: 42.0}};
        println!("DEBUG: rect: {:?}", rect);
    }

    /*
     * Enums
     */
    {
        struct Person<'a> (&'a str, Job);

        let alice = Person("Alice", Job::Engineer);
        let bob = Person("Bob", Job::Student(16));
        let ken = Person("Ken", Job::Programmer{lang: "Java".to_string(), freelance: false});
        let kumi = Person("Kumi", Job::Programmer{lang: "C++".to_string(), freelance: true});

        fn show_person(p: Person) {
            // Note:
            // Job must be declared in global scope
            use Job::*;
            match p.1 {
                Engineer => println!("{} is an engineer.", p.0),
                Student(age) => println!("{} is a student.  Age is {}", p.0, age),
                Programmer{lang, freelance} =>
                    println!(
                        "{} is a programmer.  Primary language is {}.  {}",
                        p.0,
                        lang,
                        if freelance {"Freelance"} else {"Employed"}
                    ),
            }
        }

        show_person(alice);
        show_person(bob);
        show_person(ken);
        show_person(kumi);

        /*
         * use partially
         */
        {
            use Job::{Engineer, Student};

            let alice = Person("Alice", Engineer);
            let bob = Person("Bob", Student(16));
            println!("{:?} and {:?}", alice.1, bob.1);
        }

        /*
         * C-like
         */
        {
            enum Num {
                Zero,
                Two = 2,
                Three,
            }

            enum Color {
                Red = 0xff0000,
                Green = 0x00ff00,
                Blue = 0x0000ff,
            }

            println!(
                "zero is {}, two is {}, three is {}",
                Num::Zero as i32,
                Num::Two as i32,
                Num::Three as i32,
            );

            println!("White is #{:x}", Color::Red as i32 | Color::Green as i32 | Color::Blue as i32);
        }
    }
}
