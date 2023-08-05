use crate::parser::expression::Expression;

mod parser;

fn main() {
    println!(
        "{:#?}",
        Expression::from(&String::from("3 + 9").chars().collect(), 0, vec![])
            .unwrap()
            .node
    )
}
