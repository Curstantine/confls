use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentConfig {
    pub info: EnvironmentConfigInfo,
}

impl Default for EnvironmentConfig {
    fn default() -> Self {
        panic!("EnvironmentConfig::default() should never be called")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentConfigInfo {
    /// Name of the environment.
    pub name: String,

    /// Positive whole integer.
    pub version: u32,

    /// Name of the user these changes should take place in.
    ///
    /// Used to resolve the home directory of the user.
    pub username: String,

    /// List of dependencies available from the AUR.
    pub requires: Vec<String>,

    /// Use shared config.
    pub use_shared: bool,
}

impl Default for EnvironmentConfigInfo {
    fn default() -> Self {
        panic!("EnvironmentConfigInfo::default() should never be called")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharedEnvironmentConfig {
    pub info: SharedEnvironmentConfigInfo,
}

impl Default for SharedEnvironmentConfig {
    fn default() -> Self {
        panic!("SharedEnvironmentConfig::default() should never be called")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharedEnvironmentConfigInfo {
    pub version: u32,
    pub requires: Vec<String>,
}

impl Default for SharedEnvironmentConfigInfo {
    fn default() -> Self {
        panic!("SharedEnvironmentConfigInfo::default() should never be called")
    }
}

#[derive(Debug, Clone)]
pub struct SharedEnvironmentFiles {
    /// Path to the home directory of the shared environment.
    pub home: PathBuf,

    /// Path to the root directory of the shared environment if it exists, and --no-root is not passed.
    pub root: Option<PathBuf>,
    pub config: SharedEnvironmentConfig,
}

#[derive(Debug, Clone)]
pub struct Environment {
    /// Path to the home directory of the environment.
    pub home: PathBuf,

    /// Path to the root directory of the environment if it exists, and --no-root is not passed.
    pub root: Option<PathBuf>,

    /// Shared environment files if it exists and use_shared is true.
    pub shared: Option<SharedEnvironmentFiles>,

    pub setup: EnvironmentConfig,

    /// Only available if an environment contains a `destroy.toml` file.
    pub destroy: Option<EnvironmentConfig>,
}

#[derive(Debug)]
pub struct EnvironmentFiles {
    pub home: Vec<PathBuf>,
    pub shared_home: Vec<PathBuf>,
    pub root: Vec<PathBuf>,
    pub shared_root: Vec<PathBuf>,
}

#[derive(Debug, Clone)]
pub struct RelatedPath {
    pub source: PathBuf,
    pub destination: PathBuf,
}

#[derive(Debug)]
pub struct RelatedEnvironmentFiles {
    pub home: Vec<RelatedPath>,
    pub root: Vec<RelatedPath>,
}
