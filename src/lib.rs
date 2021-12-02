mod xbyak;
use xbyak::{reg::*, Move, Xbyak};

pub fn cc() -> i32 {
    let mut xbyak = Xbyak::new();
    xbyak.mov(RAX, 42);
    xbyak.ret();
    let result = xbyak.gen_code();
    xbyak.delete();
    result
}

#[test]
fn test() {
    assert_eq!(cc(), 42);
}
