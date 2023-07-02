use chrono::prelude::*;
use std::path::{Path, PathBuf};

use crate::{environment::structs::RelatedEnvironmentFiles, errors};

/// Refers to all the data that is used outside of environment setup.
///
/// Contains backups of the files that were replaced, last environment used,
/// the sha256sum of the files that were moved and etc.
pub struct SessionConfig {
    pub backup_dir: PathBuf,
}

impl SessionConfig {
    pub async fn new() -> errors::Result<Self> {
        let home_dir = std::env::var("HOME").unwrap();
        let backup_dir = Path::new(&home_dir).join(".confls");
        tokio::fs::create_dir_all(&backup_dir).await?;

        Ok(Self { backup_dir })
    }

    /// Back up files that will be replaced when setting the environment.
    pub async fn backup_env(&self, files: &RelatedEnvironmentFiles) -> errors::Result<()> {
        let time = Local::now();
        let backup_path = self.backup_dir.join(time.to_string());

        let backup_home = backup_path.join("home");
        let backup_root = backup_path.join("root");

        tokio::fs::create_dir_all(&backup_home).await?;
        tokio::fs::create_dir(&backup_root).await?;

        for file in files.home.iter() {
            let backup_file = backup_home.join(&file.source);

            if file.source.try_exists()? {
                tokio::fs::copy(&file.source, backup_file).await?;
            }
        }

        for file in files.root.iter() {
            let backup_file = backup_root.join(&file.source);

            if file.source.try_exists()? {
                tokio::fs::copy(&file.source, backup_file).await?;
            }
        }

        Ok(())
    }
}
