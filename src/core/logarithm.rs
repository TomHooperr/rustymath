pub fn ln(x: f64) -> f64 {
    // log(x) = log(2^k * m)
    // log(x) = log(2^k) + log(m)
    //        = (k * log(2)) + log(m)

    let exp = extract_exponent(x);
    let man = extract_mantissa(x);

    // k * ln(2) + ln(m)
    (exp as f64 * ln2_taylor_approx_alt(10)) + ln_pade_approx(man - 1.0)
}

pub fn log(base: f64, x: f64) -> f64 {
    ln(x) / ln(base)
}

pub fn log10(x: f64) -> f64 {
    log(10.0, x)
}

pub fn ln_pade_approx(x: f64) -> f64 {
    let numerator = x * (1.0 + 0.5 * x);
    let denominator = 1.0 + x * (1.0 + x / 6.0);

    numerator / denominator
}

pub fn ln2_taylor_approx(depth: i32) -> f64 {
    // ln(1+x) = x - x^2/2 + x^3/3 - ...
    // ln(2)   = 1 - 1/2 + 1/3 - ... for x = 1

    let mut ln2 = 1.0;
    let mut sign = -1.0;

    for i in 2..depth {
        ln2 += 1.0 / (i as f64 * sign);
        sign *= -1.0 // flip sign
    }

    ln2
}

pub fn ln2_taylor_approx_alt(depth: i32) -> f64 {
    // Using ln(2) = ln((1+x)/(1-x)) with x = 1/3
    // This gives: ln(2) = 2(x + x^3/3 + x^5/5 + x^7/7 + ...)
    // Much faster convergence since we're using x = 1/3 instead of x = 1

    let x = 1.0 / 3.0;
    let x_squared = x * x;
    let mut x_power = x;
    let mut ln2 = 0.0;

    for i in 0..depth {
        let denominator = (2 * i + 1) as f64;
        ln2 += x_power / denominator;
        x_power *= x_squared;
    }

    ln2 * 2.0
}

// pub fn exp(x: f64) {}

// k where x = 2^k * m
pub fn extract_exponent(x: f64) -> i32 {
    // 64 - 1 (sign) - 11 (exp) = 52
    // 1. Shift k bits to the right
    // 2. Mask to remove the sign bit
    let biased_exp = ((x.to_bits() >> 52) & 0x7FF) as i32;

    // return unbiased k (-1023 is the min so it is offset to place 0 as the min. We want to undo this)
    biased_exp - 1023
}

// m where x = 2^k * m
pub fn extract_mantissa(x: f64) -> f64 {
    // 1. Keep only mantissa bits
    // 2. Set k bits to 1023 (0 unbiased) to get m for 2^0 (2^0 * m = m)
    let m_bits = (x.to_bits() & 0x000FFFFFFFFFFFFF) | 0x3FF0000000000000;

    f64::from_bits(m_bits)
}
