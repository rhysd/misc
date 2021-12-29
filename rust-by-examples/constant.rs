// Note: 'static seems lifetime
static LANGUAGE: &'static str = "Rust";

const THRESHOLD: i32 = 42;

fn is_big(n: i32) -> bool {
    n > THRESHOLD
}

fn main() {
    println!("This is {}", LANGUAGE);
    let n = 52;
    println!("{} is {}", n, if is_big(n) { "big" } else { "small" });
}
