mod xbyak;
use xbyak::{reg::*, Move, Xbyak};

mod code;
pub use code::Code;

pub fn cc(s: &str) -> i32 {
    let mut code = Code::new(s);

    let mut xbyak = Xbyak::new();
    xbyak.mov(RAX, code.take_number());

    while code.is_not_end() {
        if code.peek() == '+' {
            code.next();
            xbyak.mov(RDI, code.take_number());
            xbyak.add();
            continue;
        }

        if code.peek() == '-' {
            code.next();
            xbyak.mov(RDI, code.take_number());
            xbyak.sub();
            continue;
        }

        panic!("char: {}", code.peek().to_string());
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
}
