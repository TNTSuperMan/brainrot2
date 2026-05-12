#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SSAVersion {
    pub pointer: i16,
    pub version: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SSAValue {
    Version(SSAVersion),
    Const(u8),
    Load(i16),
}
