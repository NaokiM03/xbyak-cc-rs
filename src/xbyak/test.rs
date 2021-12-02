use super::{reg::*, Xbyak, Move};

#[test]
fn test_mov_r_i() {
    let mut xbyak = Xbyak::new();
    xbyak.mov(RAX, 1);
    xbyak.ret();
    let result = xbyak.gen_code();
    xbyak.delete();
    assert_eq!(result, 1);
}

#[test]
fn test_mov_r_r() {
    let mut xbyak = Xbyak::new();
    xbyak.mov(RAX, 1);
    xbyak.mov(RDI, 2);
    xbyak.mov(RAX, RDI);
    xbyak.ret();
    let result = xbyak.gen_code();
    xbyak.delete();
    assert_eq!(result, 2);
}

#[test]
fn test_add() {
    let mut xbyak = Xbyak::new();
    xbyak.mov(RAX, 1);
    xbyak.mov(RDI, 2);
    xbyak.add();
    xbyak.ret();
    let result = xbyak.gen_code();
    xbyak.delete();
    assert_eq!(result, 3);
}

#[test]
fn test_sub() {
    let mut xbyak = Xbyak::new();
    xbyak.mov(RAX, 1);
    xbyak.mov(RDI, 2);
    xbyak.sub();
    xbyak.ret();
    let result = xbyak.gen_code();
    xbyak.delete();
    assert_eq!(result, -1);
}
