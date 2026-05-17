use std::ops::{Index, IndexMut};

pub struct Program<'a> {
    program: &'a [u8],
    counter: usize,
}

impl<'a> Program<'a> {
    pub fn new(program: &'a [u8], pc: usize) -> Self {
        Self {
            program, counter: pc,
        }
    }
    #[inline(always)]
    pub fn read_u8(&mut self) -> u8 {
        let data = self.program[self.counter];
        self.counter += 1;
        data
    }
    #[inline(always)]
    pub fn read_u16(&mut self) -> u16 {
        let lo = self.program[self.counter] as u16;
        let hi = self.program[self.counter + 1] as u16;
        self.counter += 2;
        lo | (hi << 8)
    }
    #[inline(always)]
    pub fn read_i16(&mut self) -> i16 {
        self.read_u16() as i16
    }
    #[inline(always)]
    pub fn read_u32(&mut self) -> u32 {
        let a1 = self.program[self.counter] as u32;
        let a2 = self.program[self.counter+1] as u32;
        let a3 = self.program[self.counter+2] as u32;
        let a4 = self.program[self.counter+3] as u32;
        self.counter += 4;
        
        a4 |
        (a3 << 8) |
        (a2 << 16) |
        (a1 << 24)
    }

    #[inline(always)]
    pub fn jump(&mut self, addr: u32) {
        self.counter = addr as usize;
    }
}

pub struct Memory<'a> {
    memory: &'a mut [u8],
    offset: isize,
}
impl<'a> Memory<'a> {
    pub fn new(memory: &'a mut [u8], offset: isize) -> Self {
        Self {
            memory, offset
        }
    }
    pub fn get_offset(&self) -> isize {
        self.offset
    }
    pub fn offset(&mut self, delta: i16) {
        self.offset = self.offset.wrapping_add(delta as isize);
    }
}
impl<'a> Index<i16> for Memory<'a> {
    type Output = u8;
    fn index(&self, index: i16) -> &Self::Output {
        &self.memory[self.offset.wrapping_add(index as isize) as usize]
    }
}
impl<'a> IndexMut<i16> for Memory<'a> {
    fn index_mut(&mut self, index: i16) -> &mut Self::Output {
        &mut self.memory[self.offset.wrapping_add(index as isize) as usize]
    }
}
