use serde::Deserialize;
use std::fmt::{self, Display};

#[derive(Clone, Debug, Deserialize)]
pub struct Auth {
    pub ticket: String,
    #[serde(rename = "CSRFPreventionToken")]
    pub csrf: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SpiceConfig {
    pub host: String,
    pub password: String,
    pub proxy: String,
    #[serde(rename = "tls-port")]
    pub tls_port: u16,
    #[serde(rename = "type")]
    pub conn_type: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Guest {
    pub name: String,
    pub vmid: u32,
    pub node: String,
    #[serde(rename = "type")]
    pub kind: GuestKind,
}

#[derive(Clone, Debug, Deserialize)]
pub enum GuestKind {
    #[serde(rename = "qemu")]
    Qemu,
    #[serde(rename = "lxc")]
    Lxc,
}

impl Display for GuestKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Self::Qemu => write!(f, "Qemu"),
            Self::Lxc => write!(f, "LXC"),
        }
    }
}
