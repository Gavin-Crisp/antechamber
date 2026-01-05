use crate::NAME_LOWER;
use confy::ConfyError;
use serde::{Deserialize, Serialize};
use std::{
    fmt::{Display, Formatter},
    net::IpAddr,
};

// TODO: Validate Config creation
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Config {
    pub default_cluster: Option<usize>,
    pub clusters: Vec<Cluster>,
    pub default_user: Option<usize>,
    pub users: Vec<User>,
    pub viewer_args: Vec<String>,
}

impl Config {
    const FILE_NAME: &str = "config";
    const DEBUG_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/debug_conf.toml");

    pub fn load() -> Self {
        let res = if cfg!(feature = "dev_mode") {
            confy::load_path(Self::DEBUG_PATH)
        } else {
            confy::load(NAME_LOWER, Self::FILE_NAME)
        };

        res.unwrap_or_else(|_| {
            let _ = Self::default().store();
            Self::default()
        })
    }

    pub fn store(&self) -> Result<(), ConfyError> {
        if cfg!(feature = "dev_mode") {
            confy::store_path(Self::DEBUG_PATH, self)
        } else {
            confy::store(NAME_LOWER, Self::FILE_NAME, self)
        }
    }
}

#[derive(Clone, Debug, PartialOrd, PartialEq, Eq, Serialize, Deserialize)]
pub struct Cluster {
    pub name: String,
    pub hosts: Vec<Host>,
}

impl Display for Cluster {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.name.fmt(f)
    }
}

#[derive(Clone, Debug, PartialOrd, PartialEq, Eq, Serialize, Deserialize)]
pub struct Host {
    address: Address,
    port: u16,
}

#[derive(Clone, Debug, PartialOrd, PartialEq, Eq, Serialize, Deserialize)]
pub enum Address {
    Ip(IpAddr),
    Dns(String),
}

#[derive(Clone, Debug, Default, PartialOrd, PartialEq, Eq, Serialize, Deserialize)]
pub struct User {
    pub name: String,
    pub display_name: String,
    pub auth_method: AuthMethod,
}

impl Display for User {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.display_name.fmt(f)
    }
}

#[derive(Clone, Debug, Default, PartialOrd, PartialEq, Eq, Serialize, Deserialize)]
pub enum AuthMethod {
    #[default]
    Password,
    ApiToken(String),
}
