#[derive(Debug)]
pub struct Location(usize, usize);

impl Location {
    pub fn new(start: usize, end: usize) -> Location {
        Location(start, end)
    }
}

pub struct Token {
    pub location: Location,
    pub name: String,
}

pub struct Diagnostic {
    message: String,
}

pub struct ParseResult<T> {
    pub node: T,
    pub consumed: usize,
    pub tokens: Vec<Token>,
    pub diagnostics: Vec<Diagnostic>,
}

pub struct State {
    pub start: usize,
    pub consumed: usize,
    pub tokens: Vec<Token>,
    pub diagnostics: Vec<Diagnostic>,
}

impl State {
    pub fn new(start: usize) -> State {
        State {
            start,
            consumed: 0,
            tokens: Vec::new(),
            diagnostics: Vec::new(),
        }
    }

    pub fn accept<T>(&mut self, rule: Option<ParseResult<T>>) -> Option<T> {
        rule.map(|rule| {
            self.consumed += rule.consumed;
            self.tokens.extend(rule.tokens.into_iter());
            self.diagnostics.extend(rule.diagnostics.into_iter());

            rule.node
        })
    }

    // pub fn result<T>(&self, node: T) -> ParseResult<T> {
    //     let mut tokens = Vec::<Token>::new();

    //     tokens.extend(self.tokens.into_iter());

    //     ParseResult {
    //         node,
    //         consumed: self.consumed,
    //         tokens,
    //         diagnostics: Vec::new(),
    //     }
    // }

    pub fn get_location(&self) -> Location {
        Location::new(self.start, self.start + self.consumed)
    }
}

// struct FileState {
//     pub chars: Vec<char>,
//     pub tokens: Vec<Token>,
//     pub diagnostics: Vec<Diagnostic>,
// }

// struct RuleState {
//     pub tokens: Vec<Token>,
// }
