use crate::number::Number;

pub struct Str(String);

impl Str {
    pub fn new(string: &String) -> Str {
        Str(string.clone())
    }

    pub fn concat(&self, other: &Str) -> Str {
        let mut a = self.0.clone();
        let b = &other.0;

        a.push_str(b);

        Str(a)
    }

    pub fn get_length(&self) -> Number {
        Number::from_usize(self.0.len())
    }
}

impl Clone for Str {
    fn clone(&self) -> Self {
        Str(self.0.clone())
    }
}
