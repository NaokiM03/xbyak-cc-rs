pub enum XbyakCore {}

pub enum XbyakReg {
    Rax,
    Rdi,
}

pub mod reg {
    use super::XbyakReg;

    pub const RAX: XbyakReg = XbyakReg::Rax;
    pub const RDI: XbyakReg = XbyakReg::Rdi;
}

pub struct Xbyak {
    jit: *mut XbyakCore,
}

extern "C" {
    fn _new() -> *mut XbyakCore;
    fn _gen_code(this: *mut XbyakCore) -> i32;
    fn _delete(this: *mut XbyakCore);
}

impl Xbyak {
    pub fn new() -> Self {
        Xbyak {
            jit: unsafe { _new() },
        }
    }
    pub fn gen_code(&mut self) -> i32 {
        unsafe { _gen_code(self.jit) }
    }
    pub fn delete(&mut self) {
        unsafe { _delete(self.jit) };
    }
}

extern "C" {
    fn _mov_r_i(this: *mut XbyakCore, reg: i32, n: i32);
    fn _mov_r_r(this: *mut XbyakCore, reg1: i32, reg2: i32);

    fn _add(this: *mut XbyakCore);
    fn _sub(this: *mut XbyakCore);

    fn _ret(this: *mut XbyakCore);
}

impl Xbyak {
    pub fn add(&mut self) {
        unsafe { _add(self.jit) }
    }
    pub fn sub(&mut self) {
        unsafe { _sub(self.jit) }
    }

    pub fn ret(&mut self) {
        unsafe { _ret(self.jit) };
    }
}

pub trait Move<T, U> {
    fn mov(&mut self, t: T, u: U);
}
impl Move<XbyakReg, i32> for Xbyak {
    fn mov(&mut self, t: XbyakReg, u: i32) {
        unsafe { _mov_r_i(self.jit, t as i32, u) }
    }
}
impl Move<XbyakReg, XbyakReg> for Xbyak {
    fn mov(&mut self, t: XbyakReg, u: XbyakReg) {
        unsafe { _mov_r_r(self.jit, t as i32, u as i32) }
    }
}

#[test]
fn test_mov_r_i() {
    let mut xbyak = Xbyak::new();
    xbyak.mov(XbyakReg::Rax, 1);
    xbyak.ret();
    let result = xbyak.gen_code();
    xbyak.delete();
    assert_eq!(result, 1);
}

#[test]
fn test_mov_r_r() {
    let mut xbyak = Xbyak::new();
    xbyak.mov(XbyakReg::Rax, 1);
    xbyak.mov(XbyakReg::Rdi, 2);
    xbyak.mov(XbyakReg::Rax, XbyakReg::Rdi);
    xbyak.ret();
    let result = xbyak.gen_code();
    xbyak.delete();
    assert_eq!(result, 2);
}

#[test]
fn test_add() {
    let mut xbyak = Xbyak::new();
    xbyak.mov(XbyakReg::Rax, 1);
    xbyak.mov(XbyakReg::Rdi, 2);
    xbyak.add();
    xbyak.ret();
    let result = xbyak.gen_code();
    xbyak.delete();
    assert_eq!(result, 3);

}

#[test]
fn test_sub() {
    let mut xbyak = Xbyak::new();
    xbyak.mov(XbyakReg::Rax, 1);
    xbyak.mov(XbyakReg::Rdi, 2);
    xbyak.sub();
    xbyak.ret();
    let result = xbyak.gen_code();
    xbyak.delete();
    assert_eq!(result, -1);

}
