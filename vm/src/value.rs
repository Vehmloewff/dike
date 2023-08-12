use core::panic;
use std::cell::Ref;

use crate::{array::Array, memory::Memory, number::Number, string::Str};

pub enum Value {
    Null,
    Boolean(bool),
    String(Str),
    Number(Number),
    Array(Array),
    SweepPointer(usize),
    Ref(usize),
}

pub enum InimitablePrimitive<'a, T> {
    Raw(T),
    Ref(Ref<'a, T>, usize),
}

impl Value {
    pub fn get_human_type(&self) -> String {
        match self {
            Value::Null => String::from("null"),
            Value::Boolean(_) => String::from("bool"),
            Value::String(_) => String::from("str"),
            Value::Number(_) => String::from("num"),
            Value::Array(_) => String::from("array"),
            Value::Ref(address) => format!("*{}", address),
            Value::SweepPointer(pointer) => format!("sweep *{}", pointer),
        }
    }

    pub fn get_num(self, memory: &Memory) -> Number {
        match self {
            Value::Number(num) => num,
            Value::Ref(address) => match *memory.get(address) {
                Value::Number(num) => num,
                _ => panic!("Expected a number"),
            },
            _ => panic!("Expected a number"),
        }
    }

    pub fn get_boolean(self, memory: &Memory) -> bool {
        match self {
            Value::Boolean(boolean) => boolean,
            Value::Ref(address) => match *memory.get(address) {
                Value::Boolean(boolean) => boolean,
                _ => panic!("Expected a boolean"),
            },
            _ => panic!("Expected a boolean"),
        }
    }

    pub fn get_sweep_pointer(self, memory: &Memory) -> usize {
        match self {
            Value::SweepPointer(pointer) => pointer,
            Value::Ref(address) => match *memory.get(address) {
                Value::SweepPointer(pointer) => pointer,
                _ => panic!("Expected a sweep pointer"),
            },
            _ => panic!("Expected a sweep pointer"),
        }
    }

    pub fn get_string(self, memory: &Memory) -> InimitablePrimitive<Str> {
        match self {
            Value::String(string) => InimitablePrimitive::Raw(string),
            Value::Ref(address) => InimitablePrimitive::Ref(
                Ref::map(memory.get(address), |value| match value {
                    Value::String(string) => string,
                    _ => panic!("Expected a string"),
                }),
                address,
            ),
            _ => panic!("Expected a string"),
        }
    }

    pub fn get_array(self, memory: &Memory) -> InimitablePrimitive<Array> {
        match self {
            Value::Array(array) => InimitablePrimitive::Raw(array),
            Value::Ref(address) => InimitablePrimitive::Ref(
                Ref::map(memory.get(address), |value| match value {
                    Value::Array(array) => array,
                    _ => panic!("Expected an array"),
                }),
                address,
            ),
            _ => panic!("Expected an array"),
        }
    }
}
