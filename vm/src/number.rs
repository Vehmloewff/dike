use crate::value::Value;

pub enum ReconciledNumbers {
    Int(i32, i32),
    Float(f32, f32),
    BigInt(i64, i64),
    BigFloat(f64, f64),
}

impl ReconciledNumbers {
    pub fn reconcile(a: Value, b: Value) -> ReconciledNumbers {
        match a {
            Value::Int(a_int) => match b {
                Value::Int(b_int) => ReconciledNumbers::Int(a_int, b_int),
                _ => panic!(
                    "Cannot reconcile numbers ({}) and ({})",
                    a.get_human_type(),
                    b.get_human_type()
                ),
            },
            _ => panic!(
                "Left term ({}) cannot be added to right term ({})",
                a.get_human_type(),
                b.get_human_type()
            ),
        }
    }
}
