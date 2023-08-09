use crate::value::Value;

pub fn add(left: Value, right: Value) -> Value {
    match left {
        Value::Int(left_int) => match right {
            Value::Int(right_int) => Value::Int(left_int + right_int),
            _ => panic!(
                "Left term ({}) cannot be added to right term ({})",
                left.get_human_type(),
                right.get_human_type()
            ),
        },
        _ => panic!(
            "Left term ({}) cannot be added to right term ({})",
            left.get_human_type(),
            right.get_human_type()
        ),
    }
}
