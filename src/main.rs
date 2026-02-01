pub mod core;

fn main() {
    println!("Hello, world!");

    let x: f64 = 10.0;

    println!(
        "x = {} -> k = {}, m = {}",
        x,
        core::logarithm::extract_exponent(x),
        core::logarithm::extract_mantissa(x)
    );
    println!("ln2 std = {}", f64::ln(2.0));
    println!("ln2 taylor = {}", core::logarithm::ln2_taylor_approx(10));
    println!(
        "ln2 taylor alt = {}",
        core::logarithm::ln2_taylor_approx_alt(10)
    );
    println!("ln2 pade = {}", core::logarithm::ln_pade_approx(1.0));
    println!("ln(2)= {}", core::logarithm::ln(2.0));
    println!("log10(45) = {}", core::logarithm::log10(45.0));
}
