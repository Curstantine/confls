use anyhow::{anyhow, Result};
use async_recursion::async_recursion;
use std::{
    collections::HashSet,
    path::{Path, PathBuf},
    sync::Arc,
};
use tokio::task::JoinHandle;

use structs::{
    Environment, EnvironmentConfig, EnvironmentFiles, SharedEnvironmentConfig,
    SharedEnvironmentFiles,
};

pub mod structs;

#[async_recursion]
async fn recursively_collect_files(path: &Path) -> Result<Vec<PathBuf>> {
    let mut entries = tokio::fs::read_dir(path).await?;
    let mut files = Vec::<PathBuf>::new();

    while let Some(entry) = entries.next_entry().await? {
        let path = entry.path();

        if path.is_dir() {
            let mut rec_files = recursively_collect_files(&path).await?;
            files.append(&mut rec_files);
        } else {
            files.push(path);
        }
    }

    Ok(files)
}

impl Environment {
    pub async fn from_options(name: &str, conf_dir: &str, no_root: bool) -> Result<Environment> {
        let dir_path = Path::new(conf_dir);
        if !dir_path.try_exists()? {
            return Err(anyhow!(
                "Could not find the configuration directory at: {}, are you sure it exists?",
                conf_dir
            ));
        }

        let env_root = dir_path.join(name);
        if !env_root.try_exists()? {
            return Err(anyhow!("No environment found with name: {}", name));
        }

        let shared_path = dir_path.join("shared");
        let setup_path = env_root.join("setup.toml");
        let destroy_path = env_root.join("destroy.toml");
        let home_path = env_root.join("home");
        let root_path = env_root.join("root");

        let shared_home_path = shared_path.join("home");
        let shared_root_path = shared_path.join("root");

        let setup: EnvironmentConfig = confy::load_path(setup_path)?;

        let destroy: Option<EnvironmentConfig> = if destroy_path.try_exists()? {
            Some(confy::load_path(destroy_path)?)
        } else {
            None
        };

        let home = if home_path.try_exists()? {
            home_path
        } else {
            return Err(anyhow::anyhow!("No home directory found for environment."));
        };

        let shared = if setup.info.use_shared && shared_path.try_exists()? {
            let shared_config: SharedEnvironmentConfig =
                confy::load_path(shared_path.join("setup.toml"))?;

            let shared_home = if shared_home_path.try_exists()? {
                shared_home_path
            } else {
                return Err(anyhow::anyhow!("No shared home directory found."));
            };

            let shared_root = if !no_root && shared_root_path.try_exists()? {
                Some(shared_root_path)
            } else {
                None
            };

            Some(SharedEnvironmentFiles {
                home: shared_home,
                root: shared_root,
                config: shared_config,
            })
        } else {
            if setup.info.use_shared {
                println!(
                    "use_shared was set to true, but no shared directory was found. Ignoring..."
                );
            }

            None
        };

        let root = if !no_root && root_path.try_exists()? {
            Some(root_path)
        } else {
            None
        };

        Ok(Environment {
            home,
            shared,
            root,
            setup,
            destroy,
        })
    }

    pub async fn read(&self) -> Result<EnvironmentFiles> {
        let self_arc = Arc::new(self.clone());

        let self_c = self_arc.clone();
        let home_handle: JoinHandle<Result<Vec<PathBuf>>> = tokio::task::spawn(async move {
            let mut env_home_files = recursively_collect_files(&self_c.home).await?;

            if self_c.shared.is_some() {
                let shared = self_c.shared.as_ref().unwrap();
                let shared_home = &shared.home;
                let shared_home_files = recursively_collect_files(shared_home).await?;

                let set = HashSet::<PathBuf>::from_iter(
                    env_home_files
                        .clone()
                        .into_iter()
                        .map(|file| file.strip_prefix(&self_c.home).unwrap().to_path_buf()),
                );

                for shared_file in shared_home_files {
                    let relative_shared_file = shared_file.strip_prefix(shared_home)?.to_path_buf();

                    if !set.contains(&relative_shared_file) {
                        env_home_files.push(shared_file);
                    } else {
                        println!(
                            "Shared file {:?} is already present in the home directory. Ignoring...",
                            shared_file,
                        );
                    }
                }
            }

            Ok(env_home_files)
        });

        let self_c = self_arc.clone();
        let root_handle: JoinHandle<Result<Vec<PathBuf>>> = tokio::task::spawn(async move {
            let mut root_files = if let Some(root) = &self_c.root {
                recursively_collect_files(root).await?
            } else {
                Vec::new()
            };

            let shared_root_files_opt = &self_c
                .shared
                .as_ref()
                .and_then(|shared| shared.root.as_ref());

            if shared_root_files_opt.is_some() {
                let shared_root = shared_root_files_opt.unwrap();
                let shared_root_files = recursively_collect_files(shared_root).await?;

                let set =
                    HashSet::<PathBuf>::from_iter(root_files.clone().into_iter().map(|file| {
                        if let Some(root) = &self_c.root {
                            file.strip_prefix(root).unwrap().to_path_buf()
                        } else {
                            // Best case scenario, this should not be set as there are no root files
                            // in the environment directory, so shared root files should freely be added.
                            file
                        }
                    }));

                for shared_file in shared_root_files {
                    let relative_shared_file = shared_file.strip_prefix(shared_root)?.to_path_buf();

                    if !set.contains(&relative_shared_file) {
                        root_files.push(shared_file);
                    } else {
                        println!(
                            "Shared file {:?} is already present in the root directory. Ignoring...",
                            shared_file,
                        );
                    }
                }
            }

            Ok(root_files)
        });

        Ok(EnvironmentFiles {
            home: home_handle.await??,
            root: root_handle.await??,
        })
    }
}
