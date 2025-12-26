use serde::{Deserialize, Serialize};
use std::{
    fmt::{Display, Formatter},
    net::IpAddr,
    path::Path,
};

// TODO: Validate Config creation
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub default_cluster: Option<usize>,
    pub clusters: Vec<Cluster>,
    pub viewer_args: Vec<String>,
}

impl Config {
    #[allow(clippy::unnecessary_wraps)]
    pub fn load(_path: impl AsRef<Path>) -> Option<Self> {
        unimplemented!()
    }

    #[allow(clippy::unused_async)]
    pub fn save(&self, _path: impl AsRef<Path>) {
        unimplemented!()
    }
}

#[derive(Clone, Debug, PartialOrd, PartialEq, Eq, Serialize, Deserialize)]
pub struct Cluster {
    pub name: String,
    pub hosts: Vec<Host>,
    pub default_user: Option<usize>,
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

#[derive(Clone, Debug, Default, PartialOrd, PartialEq, Eq, Serialize, Deserialize)]
pub struct User {
    pub name: String,
    pub auth_method: AuthMethod,
}

impl Display for User {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.name.fmt(f)
    }
}

#[derive(Clone, Debug, Default, PartialOrd, PartialEq, Eq, Serialize, Deserialize)]
pub enum AuthMethod {
    #[default]
    Password,
    ApiToken(String),
}
