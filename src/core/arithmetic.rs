
pub fn sum_f64(a: f64, b: f64) -> f64 {
    a + b
}

pub fn sum_i32(a: i32, b: i32) -> i32 {
    a + b
}

pub fn subtract_f64(a: f64, b: f64) -> f64 {
    a - b
}

pub fn subtract_i32(a: i32, b: i32) -> i32 {
    a - b
}

pub fn multiply_f64(a: f64, b: f64) -> f64 {
    a * b
}

pub fn multiply_i32(a: i32, b: i32) -> i32 {
    a * b
}

pub fn divide_f64(a: f64, b: f64) -> f64 {
    a / b
}

pub fn divide_i32(a: i32, b: i32) -> i32 {
    a / b
}

pub fn modulo_f64(a: f64, b: f64) -> f64 {
    let rem = a % b;
    if rem < 0.0 {
        rem + b
    } else {
        rem
    }
}

pub fn modulo_i32(a: i32, b: i32) -> i32 {
    let rem = a % b;
    if rem < 0 {
        rem + b
    } else {
        rem
    }
}

pub fn abs_f64(a: f64) -> f64 {
    if a < 0.0 {
        -a
    } else {
        a
    }
}

pub fn abs_i32(a: i32) -> i32 {
    abs_f64(a as f64) as i32
}

pub fn power_f64(a: f64, b: f64) -> f64 {
    let mut base = a;
    let mut exp = abs_f64(b);
    let mut res = 1.0;

    if b == 0.0 {
        return 1.0;
    }

    while exp > 0.0 {
        if modulo_f64(exp, 2.0) >= 1.0 {
            res = res * base;
        }

        exp = exp / 2.0;
        base = base * base;
    }

    if b < 0.0 {
        1.0 / res
    } else {
        res
    }
}

pub fn power_i32(a: i32, b: i32) -> i32 {
    let mut base = a;
    let mut exp = abs_i32(b);
    let mut res = 1;

    if b == 0 {
        return 1;
    }

    while exp > 1 {
        if modulo_i32(exp, 2) > 0 {
            res = res * base;
            exp = exp - 1;
        }

        exp = exp / 2;
        base = base * base;
    }

    res = res * base;

    if b < 0 {
        1 / res
    } else {
        res
    }
}