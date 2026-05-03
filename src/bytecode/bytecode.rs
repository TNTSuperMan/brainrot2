use std::fmt::Display;

#[derive(Debug)]
pub enum Bytecode {
    SetC(i16, u8),
    SetL(i16, i16),
    Add(i16, u8),
    AddC(i16, i16, u8),
    AddL(i16, i16, i16),
    SubLC(i16, i16, u8),
    SubCL(i16, u8, i16),
    SubLL(i16, i16, i16),
    MulC(i16, i16, u8),
    MulL(i16, i16, i16),

    MulAddC(i16, u8,  i16, u8),
    MulAddL(i16, i16, i16, u8),

    In(i16),

    Breakpoint(i16),
    Out(i16),
    OutConst(u8),
    Jump(i32),
    JumpIfZero(i16, i32),
    JumpIfNotZero(i16, i32),
    Offset(i16),
    OffsetWithRangeCheck(i16, i16, i16),
    End,

    OffsetRangeJumpZero {
        offset: i16,
        rb: i16,
        re: i16,
        ptr: i16,
        addr: i32,
    },
    OffsetRangeJumpNotZero {
        offset: i16,
        rb: i16,
        re: i16,
        ptr: i16,
        addr: i32,
    },
    SetCSetC(i16, u8, i16, u8),
    AddAdd  (i16, u8, i16, u8),
    AddSetC (i16, u8, i16, u8),
}

impl Display for Bytecode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SetC(p1, c2) => write!(f, "${p1} = {c2}"),
            Self::SetL(p1, p2) => write!(f, "${p1} = ${p2}"),
            Self::Add(ptr, val) => write!(f, "${ptr} += {val}"),
            Self::AddC(p1, p2, c3) => write!(f, "${p1} = ${p2} + {c3}"),
            Self::AddL(p1, p2, p3) => write!(f, "${p1} = ${p2} + ${p3}"),
            Self::SubLC(p1, p2, c3) => write!(f, "${p1} = ${p2} - {c3}"),
            Self::SubCL(p1, c2, p3) => write!(f, "${p1} = {c2} - ${p3}"),
            Self::SubLL(p1, p2, p3) => write!(f, "${p1} = ${p2} - ${p3}"),
            Self::MulC(p1, p2, c3) => write!(f, "${p1} = ${p2} * {c3}"),
            Self::MulL(p1, p2, p3) => write!(f, "${p1} = ${p2} * ${p3}"),

            Self::MulAddC(p1, c2, p3, c4) => write!(f, "${p1} = {c2} + ${p3} * {c4}"),
            Self::MulAddL(p1, p2, p3, c4) => write!(f, "${p1} = ${p2} + ${p3} * {c4}"),

            Self::In(p1) => write!(f, "${p1} = stdin"),
            
            Self::Breakpoint(p1) => write!(f, "break ${p1}"),
            Self::Out(p1) => write!(f, "stdout < ${p1}"),
            Self::OutConst(v1) => write!(f, "stdout < {v1}"),
            Self::Jump(a1) => write!(f, "jr {a1}"),
            Self::JumpIfZero(p1, a2) => write!(f, "jrz ${p1}, {a2}"),
            Self::JumpIfNotZero(p1, a2) => write!(f, "jrnz ${p1}, {a2}"),
            Self::Offset(o1) => write!(f, "offset {o1}"),
            Self::OffsetWithRangeCheck(o1, rb, re) => write!(f, "offset {o1}, rangecheck {rb}..={re}"),
            Self::End => write!(f, "end"),

            Self::OffsetRangeJumpZero { offset, rb, re, ptr, addr: jmp } => write!(f, "offset {offset}, rangecheck {rb}..={re}, jrz ${ptr} {jmp}"),
            Self::OffsetRangeJumpNotZero { offset, rb, re, ptr, addr: jmp } => write!(f, "offset {offset}, rangecheck {rb}..={re}, jrnz ${ptr} {jmp}"),
            Self::SetCSetC(p1, c1, p2, c2) => write!(f, "${p1} = {c1}, ${p2} = {c2}"),
            Self::AddAdd(p1, c1, p2, c2) => write!(f, "${p1} += {c1}, ${p2} += {c2}"),
            Self::AddSetC(p1, c1, p2, c2) => write!(f, "${p1} += {c1}, ${p2} = {c2}"),
        }
    }
}
