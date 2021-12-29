fn main() {
    // Note: Prefix with '_' to ignore unused warning.
    let _immut_val = 42;

    let mut mut_val = 10;
    mut_val += 92;

    {
        // Shadowing
        let mut_val = 'a';
        println!("{}", mut_val);
    }

    println!("{}", mut_val);

    let foo = 42;
    println!("{}", foo);
    let foo = "aaa";
    println!("{}", foo);

    // Declare
    let a_binding;

    // Error!
    // println!("{}", a_binding);

    {
        let x = 2;
        a_binding = x * x;
    }

    println!("{}", a_binding);
}
