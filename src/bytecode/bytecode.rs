use std::fmt::Display;

use crate::cfg::range::OffsetRange;

#[derive(Debug)]
pub enum Bytecode {
    SetC(i16, u8),
    SetL(i16, i16),
    Add(i16, u8),
    AddC(i16, i16, u8),
    AddL(i16, i16, i16),
    AddLA(i16, i16),
    SubLC(i16, i16, u8),
    SubCL(i16, u8, i16),
    SubLL(i16, i16, i16),
    MulC(i16, i16, u8),
    MulL(i16, i16, i16),

    MulAddC(i16, u8, i16, u8),
    MulAddL(i16, i16, i16, u8),

    In(i16),

    Out(i16),
    OutConst(u8),
    Jump(i32),
    JumpIfZero(i16, i32),
    JumpIfNotZero(i16, i32),
    Offset(i16),
    OffsetWithRangeCheck(i16, OffsetRange),
    RangeCheck(OffsetRange),
    FindZero(i16, i16),
    End,

    SetCSetC(i16, u8, i16, u8),
    AddAdd(i16, u8, i16, u8),
    AddSetC(i16, u8, i16, u8),
    MulAddMulAdd {
        src: i16,
        dst1: i16,
        dst2_rel: i8,
        val1: u8,
        val2: u8,
    },
    OutOut(i16, i16),
}

impl Display for Bytecode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SetC(p1, c2) => write!(f, "${p1} = {c2}"),
            Self::SetL(p1, p2) => write!(f, "${p1} = ${p2}"),
            Self::Add(ptr, val) => write!(f, "${ptr} += {val}"),
            Self::AddC(p1, p2, c3) => write!(f, "${p1} = ${p2} + {c3}"),
            Self::AddL(p1, p2, p3) => write!(f, "${p1} = ${p2} + ${p3}"),
            Self::AddLA(p1, p2) => write!(f, "${p1} += ${p2}"),
            Self::SubLC(p1, p2, c3) => write!(f, "${p1} = ${p2} - {c3}"),
            Self::SubCL(p1, c2, p3) => write!(f, "${p1} = {c2} - ${p3}"),
            Self::SubLL(p1, p2, p3) => write!(f, "${p1} = ${p2} - ${p3}"),
            Self::MulC(p1, p2, c3) => write!(f, "${p1} = ${p2} * {c3}"),
            Self::MulL(p1, p2, p3) => write!(f, "${p1} = ${p2} * ${p3}"),

            Self::MulAddC(p1, c2, p3, c4) => write!(f, "${p1} = {c2} + ${p3} * {c4}"),
            Self::MulAddL(p1, p2, p3, c4) => write!(f, "${p1} = ${p2} + ${p3} * {c4}"),

            Self::In(p1) => write!(f, "${p1} = stdin"),

            Self::Out(p1) => write!(f, "stdout < ${p1}"),
            Self::OutConst(v1) => write!(f, "stdout < {v1}"),
            Self::Jump(a1) => write!(f, "jr {a1}"),
            Self::JumpIfZero(p1, a2) => write!(f, "jrz ${p1}, {a2}"),
            Self::JumpIfNotZero(p1, a2) => write!(f, "jrnz ${p1}, {a2}"),
            Self::Offset(o1) => write!(f, "offset {o1}"),
            Self::OffsetWithRangeCheck(o1, range) => write!(f, "offset {o1}, rangecheck {range:?}"),
            Self::RangeCheck(range) => write!(f, "rangecheck {range:?}"),
            Self::FindZero(ptr, delta) => write!(f, "findzero {ptr} {delta}"),
            Self::End => write!(f, "end"),

            Self::SetCSetC(p1, c1, p2, c2) => write!(f, "${p1} = {c1}, ${p2} = {c2}"),
            Self::AddAdd(p1, c1, p2, c2) => write!(f, "${p1} += {c1}, ${p2} += {c2}"),
            Self::AddSetC(p1, c1, p2, c2) => write!(f, "${p1} += {c1}, ${p2} = {c2}"),
            Self::MulAddMulAdd {
                src,
                dst1,
                dst2_rel,
                val1,
                val2,
            } => write!(
                f,
                "${dst1} += ${src} * {val1}, ${} += ${src} * {val2}",
                src + (*dst2_rel as i16)
            ),
            Self::OutOut(p1, p2) => write!(f, "stdout < ${p1} ${p2}"),
        }
    }
}
