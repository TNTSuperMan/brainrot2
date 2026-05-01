use std::fmt::Debug;

pub enum Bytecode {
    SetC(i16, u8),
    SetL(i16, i16),
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
    Jump(u32),
    JumpIfZero(i16, u32),
    JumpIfNotZero(i16, u32),
    Branch {
        ptr: i16,
        zero: u32,
        nonzero: u32,
    },
    Offset(i8),
    End,
}

impl Debug for Bytecode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SetC(p1, c2) => write!(f, "${p1} = {c2}"),
            Self::SetL(p1, p2) => write!(f, "${p1} = ${p2}"),
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
            Self::Jump(a1) => write!(f, "jump %{a1}"),
            Self::JumpIfZero(p1, a2) => write!(f, "jz ${p1}, %{a2}"),
            Self::JumpIfNotZero(p1, a2) => write!(f, "jnz ${p1}, %{a2}"),
            Self::Branch { ptr, zero, nonzero } => write!(f, "branch ${ptr} ? %{nonzero} : %{zero}"),
            Self::Offset(o1) => write!(f, "offset {o1}"),
            Self::End => write!(f, "end"),
        }
    }
}
