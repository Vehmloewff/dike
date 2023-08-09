pub mod channel;
pub mod instruction;
pub mod instruction_graph;
pub mod instruction_sweep;
pub mod math;
pub mod memory;
pub mod scope;
pub mod stack;
pub mod value;

// use std::{collections::HashMap, i64};

// use rust_ast::{AdditionExpression, BinaryExpression, Expression, NumberLiteral, Statement};

// pub enum Value {
//     Boolean(bool),
//     String(String),
//     Int(i32),
//     BigInt(i64),
//     Float(f32),
//     BigFloat(f64),
//     Array(Vec<Value>),
//     Object(HashMap<String, Value>),
// }

// impl Value {
//     fn get_human_type(&self) -> String {
//         match self {
//             Value::Boolean(_) => String::from("bool"),
//             Value::String(_) => String::from("str"),
//             Value::Int(_) => String::from("num"),
//             Value::BigInt(_) => String::from("num"),
//             Value::Float(_) => String::from("num"),
//             Value::BigFloat(_) => String::from("num"),
//             Value::Array(_) => String::from("array"),
//             Value::Object(_) => String::from("object"),
//         }
//     }
// }

// pub struct Interpreter {
//     memory: HashMap<String, Value>,
// }

// impl Interpreter {
//     fn interpret(&self, statement: &Statement) {}

//     fn interpret_expression(&self, expression: &Expression) -> Value {
//         match expression {
//             Expression::NumberLiteral(number_literal) => {}
//             Expression::BinaryExpression(binary_expression) => {
//                 self.interpret_binary_expression(binary_expression)
//             }
//             _ => panic!("Expression not supported"),
//         }
//     }

//     fn interpret_number_literal(&self, number_literal: &NumberLiteral) -> Value {}

//     fn interpret_binary_expression(&self, binary_expression: &BinaryExpression) -> Value {
//         match binary_expression {
//             BinaryExpression::AdditionExpression(addition_expression) => {
//                 self.interpret_addition_expression(addition_expression)
//             }
//             _ => panic!("BinaryExpression not supported"),
//         }
//     }

//     fn interpret_addition_expression(&self, addition_expression: &AdditionExpression) -> Value {
//         let left = self.interpret_expression(&addition_expression.left);
//         let right = self.interpret_expression(&addition_expression.right);

//         match left {
//             Value::Int(left_int) => match right {
//                 Value::Int(right_int) => Value::Int(left_int + right_int),
//                 // Value::BigInt(right_big_int) => {
//                 //     let left_big_int = i64::from(left_int);

//                 //     Value::BigInt(left_big_int + right_big_int)
//                 // }
//                 _ => panic!(
//                     "Left term ({}) cannot be added to right term ({})",
//                     left.get_human_type(),
//                     right.get_human_type()
//                 ),
//             },
//             _ => panic!(
//                 "Left term ({}) cannot be added to right term ({})",
//                 left.get_human_type(),
//                 right.get_human_type()
//             ),
//         }
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
