use super::{additive_expression::AdditiveExpression, base::ParseResult};

#[derive(Debug)]
pub enum Expression {
    AdditiveExpression(AdditiveExpression),
}

impl Expression {
    pub fn from(
        chars: &Vec<char>,
        start: usize,
        omit: Vec<String>,
    ) -> Option<ParseResult<Expression>> {
        if omit.contains(&String::from("AdditiveExpression")) == false {
            return AdditiveExpression::from(chars, start).map(|additive| ParseResult {
                consumed: additive.consumed,
                tokens: additive.tokens,
                node: Expression::AdditiveExpression(additive.node),
                diagnostics: Vec::new(),
            });
        };

        None
    }
}
