use super::base::{Location, ParseResult};

pub struct Whitespace(Location);

impl Whitespace {
    pub fn from(chars: &Vec<char>, start: usize) -> Option<ParseResult<Whitespace>> {
        let mut consumed = 0;

        loop {
            let maybe_character = chars.get(start + consumed);
            if maybe_character == None {
                break;
            }

            let character = maybe_character.unwrap();
            if character != &' ' && character != &'\t' {
                break;
            }

            consumed += 1;
        }

        let node = Whitespace(Location::new(start, start + consumed));

        Some(ParseResult {
            node,
            tokens: Vec::new(),
            consumed,
            diagnostics: vec![],
        })
    }
}
