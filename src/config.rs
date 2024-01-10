
#[derive(Debug, Clone)]
pub struct Repository {
    /// The url of the source repository
    pub url: String,

    /// Last tiime the local config was updated as a unix timestamp
    pub last_updated: Option<u32>,

    /// Commit used when last updating the local config
    pub last_commit: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Settings {
    /// The source repository
    pub source: Repository,
}

impl Default for Repository {
    fn default() -> Self {
        Self {
            url: "https://github.com/Square-face/example_prusa_config.git".to_string(),
            last_updated: None,
            last_commit: None,
        }
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            source: Repository::default(),
        }
    }
}

