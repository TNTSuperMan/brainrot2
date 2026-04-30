use std::fmt::Debug;

pub enum Bytecode {
    Breakpoint(i16),
    Add(i16, u8),
    AddLoad(i16, i16),
    SubLoad(i16, i16),
    Set(i16, u8),
    SetLoad(i16, i16),
    MulAdd(i16, i16, u8),
    MulAddConst(i16, u8, i16, u8),
    Mul(i16, i16, u8),
    In(i16),
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
            Self::Breakpoint(p1) => write!(f, "break ${p1}"),
            Self::Add(p1, v2) => write!(f, "${p1} += {v2}"),
            Self::AddLoad(p1, p2) => write!(f, "${p1} += ${p2}"),
            Self::SubLoad(p1, p2) => write!(f, "${p1} -= ${p2}"),
            Self::Set(p1, v2) => write!(f, "${p1} = {v2}"),
            Self::SetLoad(p1, p2) => write!(f, "${p1} = ${p2}"),
            Self::MulAdd(p1, p2, v3) => write!(f, "${p1} += ${p2} * {v3}"),
            Self::MulAddConst(p1, v2, p3, v4) => write!(f, "${p1} = ${v2} + (${p3} * {v4})"),
            Self::Mul(p1, p2, v3) => write!(f, "${p1} = ${p2} * {v3}"),
            Self::In(p1) => write!(f, "${p1} < stdin"),
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
