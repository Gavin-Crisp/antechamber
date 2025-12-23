use serde::{Deserialize, Serialize};
use std::{
    net::IpAddr,
    path::Path
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub clusters: Vec<Cluster>,
    pub viewer_args: Vec<String>,
}

impl Config {
    #[allow(clippy::unnecessary_wraps)]
    pub fn load_file(path: impl AsRef<Path>) -> Option<Self> {
        // TODO: load from file
        Some(Self {
            clusters: vec![],
            viewer_args: vec![],
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Cluster {
    pub name: String,
    pub hosts: Vec<Host>,
    pub users: Vec<User>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Host {
    Ip(IpAddr),
    Dns(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub name: String,
    pub auth_method: AuthMethod,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum AuthMethod {
    Password,
    ApiToken(String),
}
