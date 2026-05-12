use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub struct SSAVersion {
    pub pointer: i16,
    pub version: u32,
}

impl Display for SSAVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "${}#{}", self.pointer, self.version)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum SSAValue {
    Version(SSAVersion),
    Const(u8),
}

impl Display for SSAValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SSAValue::Version(ver) => write!(f, "{ver}"),
            SSAValue::Const(val) => write!(f, "{val}"),
        }
    }
}
