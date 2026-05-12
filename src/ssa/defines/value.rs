#[derive(Debug, Clone, Copy)]
pub struct SSAVersion {
    pub pointer: i16,
    pub version: u32,
}

#[derive(Debug, Clone, Copy)]
pub enum SSAValue {
    Version(SSAVersion),
    Const(u8),
}
