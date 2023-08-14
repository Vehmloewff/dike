use std::vec;

use serde::{Deserialize, Serialize};
use syn::{Item, ItemImpl, ItemStruct, PathSegment, Type, Visibility};

#[derive(Serialize, Deserialize)]

pub struct RustType {}

#[derive(Serialize, Deserialize)]
pub struct Argument {
    name: String,
    rust_type: RustType,
}

#[derive(Serialize, Deserialize)]
pub struct Function {
    name: String,
    arguments: Vec<Argument>,
    return_type: RustType,
}

#[derive(Serialize, Deserialize)]
pub struct Impl {
    name: String,
    functions: Vec<Function>,
}

#[derive(Serialize, Deserialize)]
pub struct Struct {
    name: String,
}

#[derive(Serialize, Deserialize)]
pub enum Field {
    Struct(Struct),
    Impl(Impl),
}

pub fn map_items(item: &Item) -> Option<Field> {
    let struct_item = match item {
        Item::Struct(structure) => match structure.vis {
            Visibility::Public(_) => Some(map_struct(structure)),
            _ => None,
        },
        _ => None,
    }?;
}

fn map_struct(structure: &ItemStruct) -> Struct {
    let name = structure.ident.to_string();

    Struct { name }
}

fn map_impl(implementation: &ItemImpl) -> Impl {
    let segment: &PathSegment = match *implementation.self_ty {
        Type::Path(ref fat) => fat.path.segments.last().unwrap().value(),
        _ => panic!("fad"),
    };

    Impl {
        name: segment.ident.to_string(),
        functions: vec![],
    }
}
