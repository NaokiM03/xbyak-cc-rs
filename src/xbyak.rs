pub enum XbyakCore {}

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
    fn _mov(this: *mut XbyakCore, n: i32);
    fn _ret(this: *mut XbyakCore);
}

impl Xbyak {
    pub fn mov(&mut self, n: i32) {
        unsafe { _mov(self.jit, n) };
    }
    pub fn ret(&mut self) {
        unsafe { _ret(self.jit) };
    }
}
