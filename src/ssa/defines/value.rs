pub struct SSAVersion {
    pub pointer: i16,
    pub version: u32,
}

pub enum SSAValue {
    Version(SSAVersion),
    Const(u8),
}
