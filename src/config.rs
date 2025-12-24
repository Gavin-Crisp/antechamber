use serde::{Deserialize, Serialize};
use std::{
    fmt::{Display, Formatter},
    net::IpAddr,
    path::Path,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub clusters: Vec<Cluster>,
    pub viewer_args: Vec<String>,
}

impl Config {
    #[allow(clippy::unnecessary_wraps)]
    pub fn load_file(_path: impl AsRef<Path>) -> Option<Self> {
        // TODO: load from file
        Some(Self {
            clusters: vec![],
            viewer_args: vec![],
        })
    }
}

#[derive(Clone, Debug, PartialOrd, PartialEq, Eq, Serialize, Deserialize)]
pub struct Cluster {
    pub name: String,
    pub hosts: Vec<Host>,
    pub users: Vec<User>,
}

impl Display for Cluster {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.name.fmt(f)
    }
}

#[derive(Clone, Debug, PartialOrd, PartialEq, Eq, Serialize, Deserialize)]
pub enum Host {
    Ip(IpAddr),
    Dns(String),
}

#[derive(Clone, Debug, PartialOrd, PartialEq, Eq, Serialize, Deserialize)]
pub struct User {
    pub name: String,
    pub auth_method: AuthMethod,
}

impl Display for User {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.name.fmt(f)
    }
}

#[derive(Clone, Debug, PartialOrd, PartialEq, Eq, Serialize, Deserialize)]
pub enum AuthMethod {
    Password,
    ApiToken(String),
}
