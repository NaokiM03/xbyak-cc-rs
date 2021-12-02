mod xbyak;
use xbyak::{reg::*, Move, Xbyak};

mod code;
pub use code::Code;

pub fn cc(s: &str) -> i32 {
    let mut code = Code::new(s);

    let mut xbyak = Xbyak::new();
    xbyak.mov(RAX, code.take_number());
    xbyak.ret();
    let result = xbyak.gen_code();
    xbyak.delete();
    result
}

#[test]
fn test() {
    assert_eq!(cc("0"), 0);
    assert_eq!(cc("42"), 42);
}
