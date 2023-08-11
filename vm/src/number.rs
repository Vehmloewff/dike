use crate::{memory::Memory, value::Value};

pub enum ReconciledNumbers {
    Int(i32, i32),
    Float(f32, f32),
    BigInt(i64, i64),
    BigFloat(f64, f64),
}

macro_rules! reconcile_panic {
    ($a:expr, $b:expr) => {
        panic!(
            "Left term ({}) cannot be added to right term ({})",
            $a.get_human_type(),
            $b.get_human_type()
        )
    };
}

impl ReconciledNumbers {
    pub fn reconcile(a: Value, b: Value, memory: &Memory) -> ReconciledNumbers {
        // A few notes here:
        //
        // - If a is an int and b is a float, we will upgrade both to big floats. This is because an
        //   int->float conversion may loose precision
        match a {
            Value::Int(a_int) => match b {
                Value::Int(b_int) => ReconciledNumbers::Int(a_int, b_int),
                Value::BigInt(b_big_int) => ReconciledNumbers::BigInt(a_int as i64, b_big_int),
                Value::Float(b_float) => ReconciledNumbers::BigFloat(a_int as f64, b_float as f64),
                Value::BigFloat(b_big_float) => {
                    ReconciledNumbers::BigFloat(a_int as f64, b_big_float)
                }
                Value::Ref(address) => match *memory.get(address) {
                    Value::Int(b_int) => ReconciledNumbers::Int(a_int, b_int),
                    Value::BigInt(b_big_int) => ReconciledNumbers::BigInt(a_int as i64, b_big_int),
                    Value::Float(b_float) => {
                        ReconciledNumbers::BigFloat(a_int as f64, b_float as f64)
                    }
                    Value::BigFloat(b_big_float) => {
                        ReconciledNumbers::BigFloat(a_int as f64, b_big_float)
                    }
                    _ => reconcile_panic!(a, *memory.get(address)),
                },
                _ => reconcile_panic!(a, b),
            },
            Value::BigInt(a_big_int) => match b {
                Value::Int(b_int) => ReconciledNumbers::BigInt(a_big_int, b_int as i64),
                Value::BigInt(b_big_int) => ReconciledNumbers::BigInt(a_big_int, b_big_int),
                Value::Float(b_float) => {
                    ReconciledNumbers::BigFloat(a_big_int as f64, b_float as f64)
                }
                Value::BigFloat(b_big_float) => {
                    ReconciledNumbers::BigFloat(a_big_int as f64, b_big_float)
                }
                Value::Ref(address) => match *memory.get(address) {
                    Value::Int(b_int) => ReconciledNumbers::BigInt(a_big_int, b_int as i64),
                    Value::BigInt(b_big_int) => ReconciledNumbers::BigInt(a_big_int, b_big_int),
                    Value::Float(b_float) => {
                        ReconciledNumbers::BigFloat(a_big_int as f64, b_float as f64)
                    }
                    Value::BigFloat(b_big_float) => {
                        ReconciledNumbers::BigFloat(a_big_int as f64, b_big_float)
                    }
                    _ => reconcile_panic!(a, *memory.get(address)),
                },
                _ => reconcile_panic!(a, b),
            },
            Value::Float(a_float) => match b {
                Value::Int(b_int) => ReconciledNumbers::BigFloat(a_float as f64, b_int as f64),
                Value::BigInt(b_big_int) => {
                    ReconciledNumbers::BigFloat(a_float as f64, b_big_int as f64)
                }
                Value::Float(b_float) => {
                    ReconciledNumbers::BigFloat(a_float as f64, b_float as f64)
                }
                Value::BigFloat(b_big_float) => {
                    ReconciledNumbers::BigFloat(a_float as f64, b_big_float)
                }
                Value::Ref(address) => match *memory.get(address) {
                    Value::Int(b_int) => ReconciledNumbers::BigFloat(a_float as f64, b_int as f64),
                    Value::BigInt(b_big_int) => {
                        ReconciledNumbers::BigFloat(a_float as f64, b_big_int as f64)
                    }
                    Value::Float(b_float) => {
                        ReconciledNumbers::BigFloat(a_float as f64, b_float as f64)
                    }
                    Value::BigFloat(b_big_float) => {
                        ReconciledNumbers::BigFloat(a_float as f64, b_big_float)
                    }
                    _ => reconcile_panic!(a, *memory.get(address)),
                },
                _ => reconcile_panic!(a, b),
            },
            Value::BigFloat(a_big_float) => match b {
                Value::Int(b_int) => ReconciledNumbers::BigFloat(a_big_float, b_int as f64),
                Value::BigInt(b_big_int) => {
                    ReconciledNumbers::BigFloat(a_big_float, b_big_int as f64)
                }
                Value::Float(b_float) => ReconciledNumbers::BigFloat(a_big_float, b_float as f64),
                Value::BigFloat(b_big_float) => {
                    ReconciledNumbers::BigFloat(a_big_float, b_big_float)
                }
                Value::Ref(address) => match *memory.get(address) {
                    Value::Int(b_int) => ReconciledNumbers::BigFloat(a_big_float, b_int as f64),
                    Value::BigInt(b_big_int) => {
                        ReconciledNumbers::BigFloat(a_big_float, b_big_int as f64)
                    }
                    Value::Float(b_float) => {
                        ReconciledNumbers::BigFloat(a_big_float, b_float as f64)
                    }
                    Value::BigFloat(b_big_float) => {
                        ReconciledNumbers::BigFloat(a_big_float, b_big_float)
                    }
                    _ => reconcile_panic!(a, *memory.get(address)),
                },
                _ => reconcile_panic!(a, b),
            },
            _ => reconcile_panic!(a, b),
        }
    }

    // fn get_int(value: Value, memory: &Memory) -> i32 {
    //     match value {
    //         Value::Int(int) => int,
    //         Value::Ref(address) => match *memory.get(address) {
    //             Value::Int(int) => int,
    //             _ => panic!("Cannot infer an integer from ({})", value.get_human_type()),
    //         },
    //         _ => panic!("Cannot infer an integer from ({})", value.get_human_type()),
    //     }
    // }
}
