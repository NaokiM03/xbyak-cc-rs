use crate::Code;

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    Reserved(String),
    Num(i32),
    Eof,
}

#[derive(Debug)]
pub struct Token {
    kind: TokenKind,
    cur: usize,
    len: usize,
}

impl Token {
    fn new_reserved_token(s: &str, cur: usize) -> Self {
        Token {
            kind: TokenKind::Reserved(s.to_owned()),
            cur,
            len: s.len(),
        }
    }

    fn new_num_token(n: i32, cur: usize) -> Self {
        Token {
            kind: TokenKind::Num(n),
            cur,
            len: 0,
        }
    }

    fn new_eof_token() -> Self {
        Token {
            kind: TokenKind::Eof,
            cur: 0,
            len: 0,
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

            if code.remaining_number() > 2
                && (code.peek_n(2) == "=="
                    || code.peek_n(2) == "!="
                    || code.peek_n(2) == "<="
                    || code.peek_n(2) == ">=")
            {
                let cur = code.cur();
                tokens
                    .inner
                    .push(Token::new_reserved_token(&code.take_string(2), cur));
                continue;
            }

            if code.peek() == '+'
                || code.peek() == '-'
                || code.peek() == '*'
                || code.peek() == '/'
                || code.peek() == '('
                || code.peek() == ')'
                || code.peek() == '<'
                || code.peek() == '>'
            {
                let cur = code.cur();
                tokens
                    .inner
                    .push(Token::new_reserved_token(&code.take_string(1), cur));
                continue;
            }

            if code.peek().is_ascii_digit() {
                let cur = code.cur();
                tokens
                    .inner
                    .push(Token::new_num_token(code.take_number(), cur));
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

    pub fn consume(&mut self, op: &str) -> bool {
        match &self.peek().kind {
            TokenKind::Reserved(s) if s == op => {
                self.next();
                true
            }
            _ => false,
        }
    }

    pub fn expect(&mut self, op: &str) {
        match &self.peek().kind {
            TokenKind::Reserved(s) if s == op => {
                self.next();
            }
            _ => panic!("unexpect token. token: {:?}", self.peek()),
        }
    }

    pub fn expect_number(&mut self) -> i32 {
        match self.peek().kind {
            TokenKind::Num(n) => {
                self.next();
                n
            }
            _ => panic!("expect number. but token: {:?}", self.peek()),
        }
    }
}
