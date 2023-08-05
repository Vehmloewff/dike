// mod ast;
// mod interpreter;
// mod parser;

fn main() {
    let string = String::from("Hello World!");
    let chars: Vec<char> = string.chars().collect();
    let start = 0;
    let mut consumed = 0;

    loop {
        let maybe_character = string.chars().nth(start + consumed);

        if maybe_character == None {
            break;
        }

        let character = maybe_character.unwrap();

        if character == ' ' {
            break;
        }

        consumed += 1;
    }

    println!("{}", consumed);
}

enum Expression {
    AdditiveExpression(AdditiveExpression),
}

impl Expression {
    pub fn from(chars: &Vec<char>, start: i32) -> Option<ParseResult<Expression>> {
        AdditiveExpression::from(chars, start).map(|additive| ParseResult {
            consumed: additive.consumed,
            tokens: additive.tokens,
            node: Expression::AdditiveExpression(additive.node),
        })
    }
}

struct Location(i32, i32);

enum AdditiveOperator {
    Addition(Location),
    Subtraction(Location),
}

struct AdditiveExpression {
    location: Location,
    operator: AdditiveOperator,
    left: Box<Expression>,
    right: Box<Expression>,
}

impl AdditiveExpression {
    pub fn from(chars: &Vec<char>, start: i32) -> Option<ParseResult<AdditiveExpression>> {
        let state = State::new(start);

        let left = Box::new(state.expect(Expression::from(chars, start))?);
        // state.optional(Whitespace::from(state.text));

        // let operator = state.expect(AdditiveOperator::from(state.text))?;
        // state.optional(Whitespace::from(state.text));

        let right = Box::new(state.expect(Expression::from(chars, start))?);

        state.result(AdditiveExpression {
            left,
            operator,
            right,
            location: state.get_location(),
        })
    }
}

struct State {
    pub start: i32,
    pub consumed: i32,
}

impl State {
    pub fn new(start: i32) -> State {
        State { start, consumed: 0 }
    }

    pub fn expect<T>(&mut self, rule: Option<ParseResult<T>>) -> Option<T> {
        let res = rule?;

        Some(res.node)
    }
}

struct ParseResult<T> {
    node: T,         // The node that was just parsed
    consumed: i32,   // The number of charaters that were consumed
    tokens: Vec<()>, // This list of tokens in the rule that was just parsed
}
