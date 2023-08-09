use std::collections::HashMap;

pub enum Value {
    Null,
    Boolean(bool),
    String(String),
    Int(i32),
    BigInt(i64),
    Float(f32),
    BigFloat(f64),
    Array(Vec<Value>),
    Object(HashMap<String, Value>),
    Ref(usize),
    SweepPointer(usize),
}

impl Value {
    pub fn get_human_type(&self) -> String {
        match self {
            Value::Null => String::from("null"),
            Value::Boolean(_) => String::from("bool"),
            Value::String(_) => String::from("str"),
            Value::Int(_) => String::from("num"),
            Value::BigInt(_) => String::from("num"),
            Value::Float(_) => String::from("num"),
            Value::BigFloat(_) => String::from("num"),
            Value::Array(_) => String::from("array"),
            Value::Object(_) => String::from("object"),
            Value::Ref(address) => format!("*{}", address),
            Value::SweepPointer(pointer) => format!("sweep *{}", pointer),
        }
    }
}
