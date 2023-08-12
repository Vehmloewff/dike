use super::Number;

pub enum ReconciledNumbers {
    Int(i32, i32),
    BigInt(i64, i64),
    Float(f32, f32),
    BigFloat(f64, f64),
}

impl ReconciledNumbers {
    pub fn reconcile(a: Number, b: Number) -> ReconciledNumbers {
        // A few notes here:
        //
        // - If a is an int and b is a float, we will upgrade both to big floats. This is because an
        //   int->float conversion may loose precision
        match a {
            Number::Int(a_int) => match b {
                Number::Int(b_int) => ReconciledNumbers::Int(a_int, b_int),
                Number::BigInt(b_big_int) => ReconciledNumbers::BigInt(a_int as i64, b_big_int),
                Number::Float(b_float) => ReconciledNumbers::BigFloat(a_int as f64, b_float as f64),
                Number::BigFloat(b_big_float) => {
                    ReconciledNumbers::BigFloat(a_int as f64, b_big_float)
                }
            },
            Number::BigInt(a_big_int) => match b {
                Number::Int(b_int) => ReconciledNumbers::BigInt(a_big_int, b_int as i64),
                Number::BigInt(b_big_int) => ReconciledNumbers::BigInt(a_big_int, b_big_int),
                Number::Float(b_float) => {
                    ReconciledNumbers::BigFloat(a_big_int as f64, b_float as f64)
                }
                Number::BigFloat(b_big_float) => {
                    ReconciledNumbers::BigFloat(a_big_int as f64, b_big_float)
                }
            },
            Number::Float(a_float) => match b {
                Number::Int(b_int) => ReconciledNumbers::BigFloat(a_float as f64, b_int as f64),
                Number::BigInt(b_big_int) => {
                    ReconciledNumbers::BigFloat(a_float as f64, b_big_int as f64)
                }
                Number::Float(b_float) => ReconciledNumbers::Float(a_float, b_float),
                Number::BigFloat(b_big_float) => {
                    ReconciledNumbers::BigFloat(a_float as f64, b_big_float)
                }
            },
            Number::BigFloat(a_big_float) => match b {
                Number::Int(b_int) => ReconciledNumbers::BigFloat(a_big_float, b_int as f64),
                Number::BigInt(b_big_int) => {
                    ReconciledNumbers::BigFloat(a_big_float, b_big_int as f64)
                }
                Number::Float(b_float) => ReconciledNumbers::BigFloat(a_big_float, b_float as f64),
                Number::BigFloat(b_big_float) => {
                    ReconciledNumbers::BigFloat(a_big_float, b_big_float)
                }
            },
        }
    }
}
