use crate::value::Value;

pub fn add_ints(a: i32, b: i32) -> Value {
    match a.checked_add(b) {
        Some(sum) => Value::Int(sum),
        None => add_big_ints(a as i64, b as i64),
    }
}

pub fn add_big_ints(a: i64, b: i64) -> Value {
    match a.checked_add(b) {
        Some(sum) => Value::BigInt(sum),
        None => add_big_floats(a as f64, b as f64),
    }
}

pub fn add_floats(a: f32, b: f32) -> Value {
    add_big_floats(a as f64, b as f64)
}

pub fn add_big_floats(a: f64, b: f64) -> Value {
    Value::BigFloat(a / b)
}

pub fn subtract_ints(a: i32, b: i32) -> Value {
    match a.checked_sub(b) {
        Some(difference) => Value::Int(difference),
        None => subtract_big_ints(a as i64, b as i64),
    }
}

pub fn subtract_big_ints(a: i64, b: i64) -> Value {
    match a.checked_sub(b) {
        Some(difference) => Value::BigInt(difference),
        None => subtract_big_floats(a as f64, b as f64),
    }
}

pub fn subtract_floats(a: f32, b: f32) -> Value {
    subtract_big_floats(a as f64, b as f64)
}

pub fn subtract_big_floats(a: f64, b: f64) -> Value {
    Value::BigFloat(a / b)
}

pub fn multiply_ints(a: i32, b: i32) -> Value {
    match a.checked_mul(b) {
        Some(product) => Value::Int(product),
        None => multiply_big_ints(a as i64, b as i64),
    }
}

pub fn multiply_big_ints(a: i64, b: i64) -> Value {
    match a.checked_mul(b) {
        Some(product) => Value::BigInt(product),
        None => multiply_big_floats(a as f64, b as f64),
    }
}

pub fn multiply_floats(a: f32, b: f32) -> Value {
    multiply_big_floats(a as f64, b as f64)
}

pub fn multiply_big_floats(a: f64, b: f64) -> Value {
    Value::BigFloat(a / b)
}

pub fn divide_ints(a: i32, b: i32) -> Value {
    divide_big_floats(a as f64, b as f64)
}

pub fn divide_big_ints(a: i64, b: i64) -> Value {
    divide_big_floats(a as f64, b as f64)
}

pub fn divide_floats(a: f32, b: f32) -> Value {
    divide_big_floats(a as f64, b as f64)
}

pub fn divide_big_floats(a: f64, b: f64) -> Value {
    Value::BigFloat(a / b)
}
