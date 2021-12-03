use crate::Code;

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    Reserved(char),
    Num(i32),
    Eof,
}

#[derive(Debug)]
pub struct Token {
    kind: TokenKind,
    cur: usize,
}

impl Token {
    fn new_reserved_token(c: char, cur: usize) -> Self {
        Token {
            kind: TokenKind::Reserved(c),
            cur,
        }
    }

    fn new_num_token(n: i32, cur: usize) -> Self {
        Token {
            kind: TokenKind::Num(n),
            cur,
        }
    }

    fn new_eof_token() -> Self {
        Token {
            kind: TokenKind::Eof,
            cur: 0,
        }
    }
}

#[derive(Debug, Default)]
pub struct Tokens {
    inner: Vec<Token>,
    pos: usize,
}

impl Tokens {
    pub fn tokenize(code: &mut Code) -> Self {
        let mut tokens = Tokens::default();

        while code.is_not_end() {
            if code.peek().is_whitespace() {
                code.next();
                continue;
            }

            if code.peek() == '+'
                || code.peek() == '-'
                || code.peek() == '*'
                || code.peek() == '/'
                || code.peek() == '('
                || code.peek() == ')'
            {
                let pos = code.cur();
                tokens
                    .inner
                    .push(Token::new_reserved_token(code.take_char(), pos));
                continue;
            }

            if code.peek().is_ascii_digit() {
                let pos = code.cur();
                tokens
                    .inner
                    .push(Token::new_num_token(code.take_number(), pos));
                continue;
            }

            panic!("failed to tokenize. char: {}", code.peek().to_string());
        }

        tokens.inner.push(Token::new_eof_token());
        tokens
    }

    fn next(&mut self) {
        self.pos += 1;
    }

    fn peek(&self) -> &Token {
        &self.inner[self.pos]
    }

    pub fn is_not_end(&self) -> bool {
        self.peek().kind != TokenKind::Eof
    }

    pub fn consume(&mut self, op: char) -> bool {
        if let TokenKind::Reserved(c) = self.peek().kind {
            if c == op {
                self.next();
                return true;
            }
        }
        false
    }

    pub fn expect(&mut self, op: char) {
        if let TokenKind::Reserved(c) = self.peek().kind {
            if c == op {
                self.next();
                return;
            }
        }
        panic!("unexpect token. token: {:?}", self.peek());
    }

    pub fn expect_number(&mut self) -> i32 {
        if let TokenKind::Num(n) = self.peek().kind {
            self.next();
            n
        } else {
            panic!("expect number. but token: {:?}", self.peek());
        }
    }
}
