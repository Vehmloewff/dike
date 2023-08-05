use super::{
    base::{Location, ParseResult, State, Token},
    expression::Expression,
    whitespace::Whitespace,
};

#[derive(Debug)]
pub enum AdditiveOperator {
    Addition(Location),
    Subtraction(Location),
}

impl AdditiveOperator {
    pub fn from(chars: &Vec<char>, start: usize) -> Option<ParseResult<AdditiveOperator>> {
        match chars.get(start).unwrap() {
            &'+' => Some(ParseResult {
                node: AdditiveOperator::Addition(Location::new(start, start + 1)),
                consumed: 1,
                tokens: vec![Token {
                    location: Location::new(start, start + 1),
                    name: String::from("operator.addition"),
                }],
                diagnostics: Vec::new(),
            }),
            &'-' => Some(ParseResult {
                node: AdditiveOperator::Subtraction(Location::new(start, start + 1)),
                consumed: 1,
                tokens: vec![Token {
                    location: Location::new(start, start + 1),
                    name: String::from("operator.addition"),
                }],
                diagnostics: Vec::new(),
            }),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct AdditiveExpression {
    location: Location,
    operator: AdditiveOperator,
    left: Box<Expression>,
    right: Box<Expression>,
}

impl AdditiveExpression {
    pub fn from(chars: &Vec<char>, start: usize) -> Option<ParseResult<AdditiveExpression>> {
        let mut state = State::new(start);

        let left =
            Box::new(state.accept(Expression::from(chars, start, vec!["AdditiveExpression"]))?);
        state.accept(Whitespace::from(chars, start)); // Notice no "?" here. This rule is optional

        let operator = state.accept(AdditiveOperator::from(chars, start))?;
        state.accept(Whitespace::from(chars, start)); // Again, no "?" here. This rule is optional

        let right = Box::new(state.accept(Expression::from(chars, start))?);

        let expr = AdditiveExpression {
            left,
            operator,
            right,
            location: state.get_location(),
        };

        Some(ParseResult {
            node: expr,
            consumed: state.consumed,
            tokens: state.tokens,
            diagnostics: state.diagnostics,
        })
    }
}
