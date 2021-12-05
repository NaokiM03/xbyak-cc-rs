#[cfg(test)]
mod test;

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
        unsafe { _delete(self.jit) }
    }
}

extern "C" {
    fn _mov_r_i(this: *mut XbyakCore, reg: i32, n: i32);
    fn _mov_r_r(this: *mut XbyakCore, reg1: i32, reg2: i32);

    fn _push_i(this: *mut XbyakCore, n: i32);
    fn _push_r(this: *mut XbyakCore, reg: i32);
    fn _pop(this: *mut XbyakCore, reg: i32);

    fn _add(this: *mut XbyakCore);
    fn _sub(this: *mut XbyakCore);
    fn _mul(this: *mut XbyakCore);
    fn _div(this: *mut XbyakCore);

    fn _cmp(this: *mut XbyakCore);
    fn _sete(this: *mut XbyakCore);
    fn _setne(this: *mut XbyakCore);
    fn _setl(this: *mut XbyakCore);
    fn _setle(this: *mut XbyakCore);
    fn _movzx(this: *mut XbyakCore);

    fn _ret(this: *mut XbyakCore);
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

pub trait Push<T> {
    fn push(&mut self, t: T);
}
impl Push<i32> for Xbyak {
    fn push(&mut self, t: i32) {
        unsafe { _push_i(self.jit, t) }
    }
}
impl Push<XbyakReg> for Xbyak {
    fn push(&mut self, t: XbyakReg) {
        unsafe { _push_r(self.jit, t as i32) }
    }
}
pub trait Pop {
    fn pop(&mut self, t: XbyakReg);
}
impl Pop for Xbyak {
    fn pop(&mut self, t: XbyakReg) {
        unsafe { _pop(self.jit, t as i32) }
    }
}

pub trait Add {
    fn add(&mut self);
}
impl Add for Xbyak {
    fn add(&mut self) {
        unsafe { _add(self.jit) }
    }
}
pub trait Sub {
    fn sub(&mut self);
}
impl Sub for Xbyak {
    fn sub(&mut self) {
        unsafe { _sub(self.jit) }
    }
}
pub trait Mul {
    fn mul(&mut self);
}
impl Mul for Xbyak {
    fn mul(&mut self) {
        unsafe { _mul(self.jit) }
    }
}
pub trait Div {
    fn div(&mut self);
}
impl Div for Xbyak {
    fn div(&mut self) {
        unsafe { _div(self.jit) }
    }
}

pub trait Cmp {
    fn cmp(&mut self);
    fn sete(&mut self);
    fn setne(&mut self);
    fn setl(&mut self);
    fn setle(&mut self);
    fn movzx(&mut self);
}
impl Cmp for Xbyak {
    fn cmp(&mut self) {
        unsafe { _cmp(self.jit) }
    }
    fn sete(&mut self) {
        unsafe { _sete(self.jit) }
    }
    fn setne(&mut self) {
        unsafe { _setne(self.jit) }
    }
    fn setl(&mut self) {
        unsafe { _setl(self.jit) }
    }
    fn setle(&mut self) {
        unsafe { _setle(self.jit) }
    }
    fn movzx(&mut self) {
        unsafe { _movzx(self.jit) }
    }
}

pub trait Ret {
    fn ret(&mut self);
}
impl Ret for Xbyak {
    fn ret(&mut self) {
        unsafe { _ret(self.jit) }
    }
}
