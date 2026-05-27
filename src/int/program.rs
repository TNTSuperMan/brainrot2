use std::marker::PhantomData;

use crate::bytecode::bytecode::Bytecode;

pub struct UnsafeProgram<'a> {
    ptr: *const Bytecode,
    _marker: PhantomData<&'a [Bytecode]>,
}

impl<'a> UnsafeProgram<'a> {
    pub fn new(bytecodes: &[Bytecode], pc: usize) -> Self {
        Self {
            ptr: unsafe { bytecodes.as_ptr().add(pc) },
            _marker: PhantomData,
        }
    }
    pub fn get_op(&self) -> &Bytecode {
        unsafe { &*self.ptr }
    }
    pub fn next(&mut self) {
        self.ptr = unsafe { self.ptr.add(1) };
    }
    pub fn jump_relative(&mut self, addr: i32) {
        self.ptr = unsafe { self.ptr.offset(addr as isize) };
    }
}
