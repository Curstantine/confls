use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::environment::structs::Environment;

/// Refers to all the data that is used outside of environment setup.
///
/// Contains backups of the files that were replaced, last environment used,
/// the sha256sum of the files that were moved and etc.
pub struct Data;

impl Data {
    pub const ROOT_PATH: &'static str = "/etc/confls";
    pub const BACKUP_PATH: &'static str = "/etc/confls/backups";

    pub async fn new() -> anyhow::Result<Self> {
        let root_path = Path::new(Self::ROOT_PATH);
        let backup_path = Path::new(Self::BACKUP_PATH);

        if !root_path.try_exists()? {
            tokio::fs::create_dir_all(root_path).await?;
            tokio::fs::create_dir(backup_path).await?;
        }

        Ok(Self)
    }

    /// Backs up files that will be replaced by the given environment.
    pub async fn backup_env(&self, env: &Environment) -> anyhow::Result<()> {
        unimplemented!()
    }
}
