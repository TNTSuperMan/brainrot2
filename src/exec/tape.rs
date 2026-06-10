use std::{fmt::Debug, marker::PhantomData, ops::Range};

use crate::TAPE_LENGTH;

pub struct OutOfRangeError {
    index: i16,
    offset: i32,
    kind: String,
}
impl Debug for OutOfRangeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let OutOfRangeError { index, offset, kind } = self;
        write!(f, "Out of range: {kind} ${index}(offset: {offset})")
    }
}

pub struct Tape {
    tape: [u8; TAPE_LENGTH],
    offset: i32,
}

impl Tape {
    pub fn new(mul_offset: u8) -> Self {
        Self {
            tape: [0; TAPE_LENGTH],
            offset: mul_offset as i32,
        }
    }
    pub fn get(&self, index: i16) -> Result<u8, OutOfRangeError> {
        match self.tape.get((index as i32 + self.offset) as usize) {
            Some(cell) => Ok(*cell),
            None => Err(OutOfRangeError { index, offset: self.offset, kind: String::from("reading") }),
        }
    }
    pub fn get_mut(&mut self, index: i16) -> Result<&mut u8, OutOfRangeError> {
        match self.tape.get_mut((index as i32 + self.offset) as usize) {
            Some(cell) => Ok(cell),
            None => Err(OutOfRangeError { index, offset: self.offset, kind: String::from("writing") }),
        }
    }
    pub fn get_offset(&self) -> i32 {
        self.offset
    }
    pub fn offset(&mut self, size: i16) {
        self.offset += size as i32;
    }
}

const TAPE_RANGE: Range<isize> = 0..65536;

pub struct UnsafeTape<'a> {
    root: *const u8,
    curr: *mut u8,
    _marker: PhantomData<&'a mut [u8]>,
    inner_offset: &'a mut i32,
}

impl<'a> UnsafeTape<'a> {
    pub fn new(tape: &'a mut Tape) -> UnsafeTape<'a> {
        let ptr = tape.tape.as_mut_ptr();
        UnsafeTape {
            root: ptr,
            curr: ptr.wrapping_add(tape.offset as isize as usize),
            _marker: PhantomData,
            inner_offset: &mut tape.offset,
        }
    }

    fn ptr(&self, ptr: *const u8) -> isize {
        unsafe { ptr.offset_from(self.root) }
    }

    pub fn get_safe(&self, index: i16) -> Result<u8, OutOfRangeError> {
        let ptr = unsafe { self.curr.offset(index as isize) };
        if TAPE_RANGE.contains(&self.ptr(ptr)) {
            Ok(unsafe { *ptr })
        } else {
            Err(OutOfRangeError {
                index,
                offset: self.get_offset(),
                kind: "reading".to_string(),
            })
        }
    }
    pub unsafe fn get(&self, index: i16) -> u8 {
        let ptr = unsafe { self.curr.offset(index as isize) };
        if cfg!(feature = "debug") {
            if !TAPE_RANGE.contains(&self.ptr(ptr)) {
                panic!("[UNSAFE!!]: OUT OF RANGE AT UNSAFE CODE, ptr: {}", self.ptr(ptr));
            }
        }
        unsafe { *ptr }
    }
    pub unsafe fn get_mut(&mut self, index: i16) -> &mut u8 {
        let ptr = unsafe { self.curr.offset(index as isize) };
        if cfg!(feature = "debug") {
            if !TAPE_RANGE.contains(&self.ptr(ptr)) {
                panic!("[UNSAFE!!]: OUT OF RANGE AT UNSAFE CODE, ptr: {}", self.ptr(ptr));
            }
        }
        unsafe { &mut *ptr }
    }
    pub fn get_offset(&self) -> i32 {
        unsafe { self.curr.offset_from(self.root) as i32 }
    }
    pub unsafe fn offset(&mut self, size: i16) {
        self.curr = unsafe { self.curr.offset(size as isize) };
    }
}

impl<'a> Drop for UnsafeTape<'a> {
    fn drop(&mut self) {
        *self.inner_offset = self.get_offset();
    }
}
