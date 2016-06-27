// Suppress all overflow warning on casting
#![allow(overflowing_literals)]

fn main() {
    let decimal = 65.4321_f32;

    // Explicit conversion
    let integer = decimal as u8;
    let character = integer as char;

    println!("{} -> {} -> {}", decimal, integer, character);
    println!("{}, {}, {}", std::mem::size_of_val(&decimal), std::mem::size_of_val(&integer), std::mem::size_of_val(&character));

    // Inference
    let elem = 5u8;

    let mut vec = Vec::new();

    // Infer type of 'vec' from type of 'elem'
    vec.push(elem);

    println!("{:?}", vec);

    // Type aliases

    // Types must have CamelCase names
    type NanoSecond = u64;
    type Inch = u64;

    #[allow(non_camel_case_types)]
    type u64_t = u64;

    let nanosec: NanoSecond = 5 as u64;
    let inches: Inch = 2 as u64_t;

    // Aliases are *NOT* new types.
    println!("{}, {}, {}", nanosec, inches, nanosec + inches);
}
