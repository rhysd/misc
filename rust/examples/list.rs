use std::fmt;

enum List {
    Cons(u32, Box<List>),
    Nil,
}

use List::*;

impl List {
    fn new() -> List {
        Nil
    }

    // Note:
    // Here, self is not qualified as '&'.
    // It means 'self' is passed by value.  Don't worry, no copy is executed because
    // Rust *moves* a value by default!
    fn add(self, elem: u32) -> List {
        Cons(elem, Box::new(self))
    }

    fn len(&self) -> u32 {
        match *self {
            // 'self' is borrowed.  So we cannot take an ownership of 'tail' here.
            // To prevent moving 'tail', we can use 'ref' for passing by reference.
            Cons(_, ref tail) => tail.len() + 1,
            Nil => 0,
        }
    }

    fn sum(&self) -> u32 {
        match *self {
            Cons(head, ref tail) => head + tail.sum(),
            Nil => 0,
        }
    }
}

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Cons(head, ref tail) => {
                write!(f, "{}, ", head).expect("Element of list can't be written!");
                tail.fmt(f)
            },
            Nil => write!(f, "nil")
        }
    }
}

fn main() {
    let l = List::new();
    let l = l.add(10);
    let l = l.add(42);
    println!("{}\nlen: {}\nsum: {}", l, l.len(), l.sum());
}
