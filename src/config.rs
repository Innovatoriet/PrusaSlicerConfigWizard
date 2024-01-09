use std::path::PathBuf;

use fast_config::{error::ConfigError, Config};
use serde::{Deserialize, Serialize};

use crate::utils;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Repository {
    /// The url of the source repository
    pub url: String,

    /// Last tiime the local config was updated as a unix timestamp
    pub last_updated: Option<u32>,

    /// Commit used when last updating the local config
    pub last_commit: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    /// If true, use exesive logging
    pub verbose: bool,

    /// The source repository
    pub source: Repository,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            verbose: false,
            source: Repository {
                url: "".to_string(),
                last_updated: None,
                last_commit: None,
            },
        }
    }
}

/// Load and parse config file
///
/// If the file doens't exist, use defaults
pub fn load() -> Result<Config<Settings>, ConfigError> {
    let mut path = utils::get_config_dir().expect("Failed to get config dir");
    path.push("config.yaml");

    let config = Config::new(path, Settings::default());

    // Write config if it successfully loaded
    if let Ok(ref res) = config {
        res.save().expect("Failed to write config file");
    }

    config
}
