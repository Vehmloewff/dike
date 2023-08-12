use self::reconcile::ReconciledNumbers;

mod arithmetic;
mod reconcile;

pub enum Number {
    Int(i32),
    Float(f32),
    BigInt(i64),
    BigFloat(f64),
}

impl Number {
    pub fn from_usize(num: usize) -> Number {
        match num > i32::MAX as usize {
            false => Number::Int(num as i32),
            true => Number::BigInt(num as i64),
        }
    }

    pub fn add(self, number: Number) -> Number {
        match ReconciledNumbers::reconcile(self, number) {
            ReconciledNumbers::Int(a, b) => arithmetic::add_ints(a, b),
            ReconciledNumbers::BigInt(a, b) => arithmetic::add_big_ints(a, b),
            ReconciledNumbers::Float(a, b) => arithmetic::add_floats(a, b),
            ReconciledNumbers::BigFloat(a, b) => arithmetic::add_big_floats(a, b),
        }
    }

    pub fn subtract(self, number: Number) -> Number {
        match ReconciledNumbers::reconcile(self, number) {
            ReconciledNumbers::Int(a, b) => arithmetic::subtract_ints(a, b),
            ReconciledNumbers::BigInt(a, b) => arithmetic::subtract_big_ints(a, b),
            ReconciledNumbers::Float(a, b) => arithmetic::subtract_floats(a, b),
            ReconciledNumbers::BigFloat(a, b) => arithmetic::subtract_big_floats(a, b),
        }
    }

    pub fn multiply(self, number: Number) -> Number {
        match ReconciledNumbers::reconcile(self, number) {
            ReconciledNumbers::Int(a, b) => arithmetic::multiply_ints(a, b),
            ReconciledNumbers::BigInt(a, b) => arithmetic::multiply_big_ints(a, b),
            ReconciledNumbers::Float(a, b) => arithmetic::multiply_floats(a, b),
            ReconciledNumbers::BigFloat(a, b) => arithmetic::multiply_big_floats(a, b),
        }
    }

    pub fn divide(self, number: Number) -> Number {
        match ReconciledNumbers::reconcile(self, number) {
            ReconciledNumbers::Int(a, b) => arithmetic::divide_ints(a, b),
            ReconciledNumbers::BigInt(a, b) => arithmetic::divide_big_ints(a, b),
            ReconciledNumbers::Float(a, b) => arithmetic::divide_floats(a, b),
            ReconciledNumbers::BigFloat(a, b) => arithmetic::divide_big_floats(a, b),
        }
    }

    pub fn get_int(self) -> i32 {
        match self {
            Number::Int(int) => int,
            _ => panic!("Expected an integer"),
        }
    }

    pub fn get_float(self) -> f32 {
        match self {
            Number::Float(float) => float,
            _ => panic!("Expected a float"),
        }
    }

    pub fn get_big_int(self) -> i64 {
        match self {
            Number::BigInt(int) => int,
            _ => panic!("Expected a big int"),
        }
    }

    pub fn get_big_float(self) -> f64 {
        match self {
            Number::BigFloat(float) => float,
            _ => panic!("Expected a big float"),
        }
    }
}

impl Clone for Number {
    fn clone(&self) -> Self {
        match self {
            Number::Int(num) => Number::Int(num.clone()),
            Number::BigInt(num) => Number::BigInt(num.clone()),
            Number::Float(num) => Number::Float(num.clone()),
            Number::BigFloat(num) => Number::BigFloat(num.clone()),
        }
    }
}

impl Copy for Number {}
