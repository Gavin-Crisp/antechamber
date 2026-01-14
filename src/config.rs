use crate::NAME_LOWER;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::{
    fmt::{Display, Formatter},
    fs::File,
    net::IpAddr,
    path::{Path, PathBuf},
};

#[derive(Clone, Debug)]
pub struct ConfigManager {
    pub config: Config,
    config_dir: PathBuf,
}

impl ConfigManager {
    // TODO: Better error handling
    pub fn from_config(config: Config) -> Option<Self> {
        let config_dir = Self::config_dir()?;

        Some(Self { config, config_dir })
    }

    // TODO: Better error handling
    pub fn load() -> Option<Self> {
        let config_dir = Self::config_dir()?;
        let config_file = File::options()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(Self::config_file_path(&config_dir))
            .ok()?;

        Some(Self {
            config: serde_yaml::from_reader(config_file).ok()?,
            config_dir,
        })
    }

    #[must_use]
    // TODO: Better error handling
    pub fn save(&self) -> bool {
        // TODO: consider creating dir if missing
        let Ok(config_file) = File::create(Self::config_file_path(&self.config_dir)) else {
            return false;
        };

        serde_yaml::to_writer(config_file, &self.config).is_ok()
    }

    fn config_dir() -> Option<PathBuf> {
        const DEBUG_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/dev_conf");

        if cfg!(feature = "dev_mode") {
            Some(DEBUG_PATH.into())
        } else {
            ProjectDirs::from("", "", NAME_LOWER).map(|dirs| dirs.config_dir().to_owned())
        }
    }

    fn config_file_path(config_dir: &Path) -> PathBuf {
        const CONFIG_FILE_NAME: &str = "config.yaml";

        config_dir.join(CONFIG_FILE_NAME)
    }
}

// TODO: Validate Config creation
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Config {
    pub default_cluster: Option<usize>,
    pub clusters: Vec<Cluster>,
    pub default_user: Option<usize>,
    pub users: Vec<User>,
    pub viewer_args: Vec<String>,
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

// TODO: api token auth does not require username; this should be reflected
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
