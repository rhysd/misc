
#![allow(dead_code)]

fn main() {
    let n = 5;

    if n < 0 {
        println!("{} is negative", n);
    } else if n > 0 {
        println!("{} is positive", n);
    } else {
        println!("{} is zero", n);
    }

    let big_n =
        if n < 10 && n > -10 {
            10 * n
        } else {
            n / 2
        };

    println!("{}", big_n);

    let mut count = 0;
    loop {
        count += 1;

        if count == 3 {
            println!("fizz");
            continue;
        }

        if count == 5 {
            break;
        }

        println!("{}", count);
    }

    'outer: loop {
        'inner: loop {
            break 'outer;
        }
    }

    while count > 0 {
        if count == 3 {
            println!("fizz");
        } else {
            println!("{}", count)
        }
        count -= 1;
    }

    for n in 1..5 {
        println!("hey, {}", n);
    }

    let age = 13;
    let msg = match age {
        1 => "One!",
        2 | 3 | 4 | 5 | 7 | 11 => "Prime number!",
        13...19 => "Teen!",
        _ => "Not so special",
    };
    println!("{}", msg);

    {
        enum Animal {
            Inu(u32),
            Neko,
        }
        fn show_animal(a: Animal) {
            match a {
                Animal::Neko => println!("cat!"),
                Animal::Inu(age) => println!("dog is {} years old!", age),
            }
        }
        show_animal(Animal::Neko);
        show_animal(Animal::Inu(3));

        let pair = (0, -2);
        match pair {
            (0, x) => println!("first is zero and second is {}", x),
            (x, 0) => println!("first is {} and second is x", x),
            _ => println!("neither is zero!"),
        }

        let reference = &4;
        match reference {
            // Dereference with pattern
            &val => println!("From reference: {}", val),
        }

        let not_reference = *reference;
        match not_reference {
            val => println!("Not a reference: {}", val),
        }

        let ref reference = 4;
        match reference {
            &val => println!("From reference: {}", val),
        }

        match not_reference {
            ref val => println!("As reference! {}", val),
        }

        struct Foo {x: (u32, u32), y: u32}

        let foo = Foo { x: (1, 2), y: 3 };
        let Foo{ x: (a, b), y } = foo;
        println!("{}, {}, {}", a, b, y);

        match foo {
            Foo{ x: (1, a), y: _ } => println!("{}!", a),
            Foo{ x: (a, _), .. /*ignore!*/ } => println!("{}!", a),
        }

        // Guard
        match pair {
            (0, x) if x == 1 => println!("(0, 1)!"),
            _ => println!("blah"),
        }

        fn age() -> u32 { 15 }

        match age() {
            n @ 1...13  => println!("Child: {}", n),
            n @ 14...19 => println!("Teen: {}", n),
            n           => println!("Adult: {}", n),
        }

        let o = Some(42);
        match o {
            Some(i) => println!("got {}", i),
            None => println!("Not found"),
        }
    }

    {
        let n = Some(7);
        if let Some(i) = n {
            println!("Got {}!", i);
        }

        let n: Option<i32> = None;
        if let Some(i) = n {
            println!("Got {}!", i);
        } else {
            println!("None!");
        }
    }

    {
        let mut o = Some(4);
        while let Some(i) = o {
            if i == 3 {
                println!("fizz");
            } else {
                println!("{}", i);
            }
            o = if i <= 0 { None } else { Some(i-1) }
        }
    }
}
