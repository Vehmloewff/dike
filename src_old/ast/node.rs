use super::{location::Location, token::Token};
use std::collections::HashMap;

pub enum Property {
    Node(Node),
    Bool(bool),
    String(String),
    Int(i32),
    BigInt(i64),
    Float(f32),
    BigFloat(f64),
}

pub struct Node {
    pub name: String,
    pub location: Location,
    pub error_count: i16,
    properties: HashMap<String, Property>,
}

impl Node {
    pub fn new(name: String) -> Node {
        Node {
            name,
            location: Location::new(),
            error_count: 0,
            properties: HashMap::new(),
        }
    }

    pub fn set_child(mut self, name: String, node: Node) -> Node {
        self.properties.insert(name, Property::Node(node));

        self
    }

    pub fn set_bool(mut self, name: String, value: bool) -> Node {
        self.properties.insert(name, Property::Bool(value));

        self
    }

    pub fn set_string(mut self, name: String, value: String) -> Node {
        self.properties.insert(name, Property::String(value));

        self
    }

    pub fn set_int(mut self, name: String, value: i32) -> Node {
        self.properties.insert(name, Property::Int(value));

        self
    }

    pub fn set_big_int(mut self, name: String, value: i64) -> Node {
        self.properties.insert(name, Property::BigInt(value));

        self
    }

    pub fn set_float(mut self, name: String, value: f32) -> Node {
        self.properties.insert(name, Property::Float(value));

        self
    }

    pub fn set_big_float(mut self, name: String, value: f64) -> Node {
        self.properties.insert(name, Property::BigFloat(value));

        self
    }

    pub fn get_child(&self, name: String) -> Option<&Node> {
        match self.properties.get(&name)? {
            Property::Node(node) => Some(node),
            _ => None,
        }
    }

    pub fn get_bool(&self, name: String) -> Option<&bool> {
        match self.properties.get(&name)? {
            Property::Bool(value) => Some(value),
            _ => None,
        }
    }

    pub fn get_string(&self, name: String) -> Option<&String> {
        match self.properties.get(&name)? {
            Property::String(value) => Some(value),
            _ => None,
        }
    }

    pub fn get_int(&self, name: String) -> Option<&i32> {
        match self.properties.get(&name)? {
            Property::Int(value) => Some(value),
            _ => None,
        }
    }

    pub fn get_big_int(&self, name: String) -> Option<&i64> {
        match self.properties.get(&name)? {
            Property::BigInt(value) => Some(value),
            _ => None,
        }
    }

    pub fn get_float(&self, name: String) -> Option<&f32> {
        match self.properties.get(&name)? {
            Property::Float(value) => Some(value),
            _ => None,
        }
    }

    pub fn get_big_float(&self, name: String) -> Option<&f64> {
        match self.properties.get(&name)? {
            Property::BigFloat(value) => Some(value),
            _ => None,
        }
    }
}
