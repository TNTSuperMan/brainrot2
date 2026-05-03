use std::{fmt::Display, marker::PhantomData};

use crate::TAPE_LENGTH;

pub struct OutOfRangeError {
    index: i16,
    offset: i16,
    kind: String,
}
impl Display for OutOfRangeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let OutOfRangeError { index, offset, kind } = self;
        write!(f, "Out of range: {kind} ${index}(offset: {offset})")
    }
}

pub struct Tape {
    tape: [u8; TAPE_LENGTH],
    offset: i16,
}

impl Tape {
    pub fn new(mul_offset: u8) -> Self {
        Self {
            tape: [0; TAPE_LENGTH],
            offset: mul_offset as i16,
        }
    }
    pub fn get(&self, index: i16) -> Result<u8, OutOfRangeError> {
        match self.tape.get((index + self.offset) as usize) {
            Some(cell) => Ok(*cell),
            None => Err(OutOfRangeError { index, offset: self.offset, kind: String::from("reading") }),
        }
    }
    pub fn get_mut(&mut self, index: i16) -> Result<&mut u8, OutOfRangeError> {
        match self.tape.get_mut((index + self.offset) as usize) {
            Some(cell) => Ok(cell),
            None => Err(OutOfRangeError { index, offset: self.offset, kind: String::from("writing") }),
        }
    }
    pub fn get_offset(&self) -> i16 {
        self.offset
    }
    pub fn offset(&mut self, size: i16) {
        self.offset += size;
    }
}

pub struct UnsafeTape<'a> {
    root: *const u8,
    curr: *mut u8,
    _marker: PhantomData<&'a mut [u8]>,
    inner_offset: &'a mut i16,
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
    pub unsafe fn get(&self, index: i16) -> u8 {
        unsafe { *self.curr.offset(index as isize) }
    }
    pub unsafe fn get_mut(&mut self, index: i16) -> &mut u8 {
        unsafe { &mut *self.curr.offset(index as isize) }
    }
    pub fn get_offset(&self) -> i16 {
        unsafe { self.curr.offset_from(self.root) as i16 }
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
