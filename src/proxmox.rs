use std::{
    fmt::{
        self,
        Display
    }
};

#[derive(Clone, Debug)]
pub struct Auth {
    pub ticket: String,
    pub csrf: String,
}

#[derive(Clone, Debug)]
pub struct SpiceConfig {
    pub host: String,
    pub password: String,
    pub proxy: String,
    pub tls_port: u16,
    pub conn_type: String,
}

#[derive(Clone, Debug)]
pub struct Guest {
    pub name: String,
    pub vmid: u32,
    pub node: String,
    pub engine: Engine,
}

#[derive(Clone, Debug)]
pub enum Engine {
    Qemu,
    Lxc,
}

impl Display for Engine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Self::Qemu => write!(f, "Qemu"),
            Self::Lxc => write!(f, "LXC"),
        }
    }
}
