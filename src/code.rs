#[derive(Debug)]
pub struct Code {
    chars: Vec<char>,
    pos: usize,
}

impl Code {
    pub fn new(s: &str) -> Self {
        Code {
            chars: s.chars().collect(),
            pos: 0,
        }
    }

    pub fn next(&mut self) {
        self.pos += 1;
    }

    pub fn peek(&self) -> char {
        self.chars[self.pos]
    }

    pub fn is_not_end(&self) -> bool {
        self.chars.len() > self.pos
    }

    pub fn take_number(&mut self) -> i32 {
        let mut result = String::new();
        while self.is_not_end() && self.peek().is_ascii_digit() {
            result += &self.peek().to_string();
            self.next();
        }
        result.parse::<i32>().unwrap()
    }
}
