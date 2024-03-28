
pub fn sum(a: f64, b: f64) -> f64 {
    a + b
}

pub fn subtract(a: f64, b: f64) -> f64 {
    a - b
}

pub fn multiply(a: f64, b: f64) -> f64 {
    a * b
}

pub fn divide(a: f64, b: f64) -> f64 {
    a / b
}

pub fn modulo(a: f64, b: f64) -> f64 {
    let rem = a % b;
    if rem < 0.0 {
        sum(rem, b)
    } else {
        rem
    }
}

pub fn abs(a: f64) -> f64 {
    if a < 0.0 {
        -a
    } else {
        a
    }
}

pub fn power(a: f64, b: f64) -> f64 {
    let mut base = a;
    let mut exp = abs(b);
    let mut res = 1.0;

    while exp > 0.0 {
        if modulo(exp, 2.0) >= 1.0 {
            res = multiply(res, base);
        }

        base = multiply(base , base);
        exp = divide(exp, 2.0);
    }

    if b < 0.0 {
        divide(1.0, res)
    } else {
        res
    }
}