use std::{fmt::Debug, ops::Range};

#[derive(Clone)]
pub struct IR {
    pub pointer: isize,
    pub opcode: IROp,
    pub loc: Range<usize>,
}
impl Debug for IR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.pointer.fmt(f)?;
        f.write_str(" ")?;
        self.opcode.fmt(f)?;
        f.write_str("\n")?;
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum IROp {
    Breakpoint,
    Add(u8),
    Set(u8),
    MulAdd(isize, u8), // [pointer] = [pointer] + [opcode.0] * opcode.1
    In,
    Out,
    JumpZero(usize),
    JumpNotZero(usize),
    JumpNotZeroWithOffset(isize, usize),
}
