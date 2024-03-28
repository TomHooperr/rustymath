
pub mod core;

fn main() {
    println!("Hello, world!");

    let a = 4.0;
    let b = -3.0;

    println!("{}^{} = {}", a, b, core::arithmetic::power(a, b));
}
