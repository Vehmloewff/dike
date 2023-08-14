pub struct Str(String);

impl Str {
    pub fn new(string: &str) -> Str {
        Str(String::from(string))
    }

    pub fn concat(&self, other: Str) -> Str {
        let mut a = self.0.clone();
        let b = &other.0;

        a.push_str(b);

        Str(a)
    }
}
