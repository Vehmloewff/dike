use serde_json;
use std::env;
use std::fs;
use syn;

mod field;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file = args
        .get(1)
        .expect("Expected a path to a rust file as the first argument");
    let output_file = args
        .get(2)
        .expect("Expected an output path as the second argument");

    let text = fs::read_to_string(input_file).expect("Couldn't read rust input file");
    let ast = syn::parse_file(&text).expect("Failed to parse rust code");

    let mapped: Vec<field::Field> = ast
        .items
        .iter()
        .map(|item| field::map_items(item))
        .filter(|item| match item {
            Some(_) => true,
            None => false,
        })
        .map(|field| field.unwrap())
        .collect();

    let json = serde_json::to_string(&mapped).expect("Failed to print rust ast to json");

    fs::write(output_file, json).expect("Failed to write rust ast to output file")
}
