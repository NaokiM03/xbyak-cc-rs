mod xbyak;
use xbyak::Xbyak;

pub fn cc() -> i32 {
    let mut xbyak = Xbyak::new();
    xbyak.mov(42);
    xbyak.ret();
    let result = xbyak.gen_code();
    xbyak.delete();
    result
}

#[test]
fn test() {
    assert_eq!(cc(), 42);
}
