#[derive(Debug)]
pub struct Code {
    chars: Vec<char>,
    cur: usize,
}

impl Code {
    pub fn cur(&self) -> usize {
        self.cur
    }
}

impl Code {
    pub fn new(s: &str) -> Self {
        Code {
            chars: s.chars().collect(),
            cur: 0,
        }
    }

    pub fn next(&mut self) {
        self.cur += 1;
    }

    pub fn peek(&self) -> char {
        self.chars[self.cur]
    }

    pub fn is_not_end(&self) -> bool {
        self.chars.len() > self.cur
    }

    pub fn take_char(&mut self) -> char {
        let result = self.peek();
        self.next();
        result
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
