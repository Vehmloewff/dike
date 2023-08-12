use super::Number;

pub fn add_ints(a: i32, b: i32) -> Number {
    match a.checked_add(b) {
        Some(sum) => Number::Int(sum),
        None => add_big_ints(a as i64, b as i64),
    }
}

pub fn add_big_ints(a: i64, b: i64) -> Number {
    match a.checked_add(b) {
        Some(sum) => Number::BigInt(sum),
        None => add_big_floats(a as f64, b as f64),
    }
}

pub fn add_floats(a: f32, b: f32) -> Number {
    add_big_floats(a as f64, b as f64)
}

pub fn add_big_floats(a: f64, b: f64) -> Number {
    Number::BigFloat(a / b)
}

pub fn subtract_ints(a: i32, b: i32) -> Number {
    match a.checked_sub(b) {
        Some(difference) => Number::Int(difference),
        None => subtract_big_ints(a as i64, b as i64),
    }
}

pub fn subtract_big_ints(a: i64, b: i64) -> Number {
    match a.checked_sub(b) {
        Some(difference) => Number::BigInt(difference),
        None => subtract_big_floats(a as f64, b as f64),
    }
}

pub fn subtract_floats(a: f32, b: f32) -> Number {
    subtract_big_floats(a as f64, b as f64)
}

pub fn subtract_big_floats(a: f64, b: f64) -> Number {
    Number::BigFloat(a / b)
}

pub fn multiply_ints(a: i32, b: i32) -> Number {
    match a.checked_mul(b) {
        Some(product) => Number::Int(product),
        None => multiply_big_ints(a as i64, b as i64),
    }
}

pub fn multiply_big_ints(a: i64, b: i64) -> Number {
    match a.checked_mul(b) {
        Some(product) => Number::BigInt(product),
        None => multiply_big_floats(a as f64, b as f64),
    }
}

pub fn multiply_floats(a: f32, b: f32) -> Number {
    multiply_big_floats(a as f64, b as f64)
}

pub fn multiply_big_floats(a: f64, b: f64) -> Number {
    Number::BigFloat(a / b)
}

pub fn divide_ints(a: i32, b: i32) -> Number {
    divide_big_floats(a as f64, b as f64)
}

pub fn divide_big_ints(a: i64, b: i64) -> Number {
    divide_big_floats(a as f64, b as f64)
}

pub fn divide_floats(a: f32, b: f32) -> Number {
    divide_big_floats(a as f64, b as f64)
}

pub fn divide_big_floats(a: f64, b: f64) -> Number {
    Number::BigFloat(a / b)
}
