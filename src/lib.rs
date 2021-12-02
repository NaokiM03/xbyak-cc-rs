mod xbyak;
use xbyak::{reg::*, Move, Xbyak};

mod code;
use code::Code;

mod token;
use token::Tokens;

pub fn cc(s: &str) -> i32 {
    let mut code = Code::new(s);

    let mut tokens = Tokens::tokenize(&mut code);

    let mut xbyak = Xbyak::new();
    xbyak.mov(RAX, tokens.expect_number());

    while tokens.is_not_end() {
        if tokens.consume('+') {
            xbyak.mov(RDI, tokens.expect_number());
            xbyak.add();
            continue;
        }

        tokens.expect('-');
        xbyak.mov(RDI, tokens.expect_number());
        xbyak.sub();
    }

    xbyak.ret();
    let result = xbyak.gen_code();
    xbyak.delete();
    result
}

#[test]
fn test() {
    assert_eq!(cc("0"), 0);
    assert_eq!(cc("42"), 42);
    assert_eq!(cc("5+20-4"), 21);
    assert_eq!(cc(" 12 + 34 - 5"), 41);
}
