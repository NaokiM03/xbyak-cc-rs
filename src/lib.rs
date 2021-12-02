mod xbyak;
use xbyak::{Xbyak, XbyakReg, Move};

pub fn cc() -> i32 {
    let mut xbyak = Xbyak::new();
    xbyak.mov(XbyakReg::Rax, 42);
    xbyak.ret();
    let result = xbyak.gen_code();
    xbyak.delete();
    result
}

#[test]
fn test() {
    assert_eq!(cc(), 42);
}
