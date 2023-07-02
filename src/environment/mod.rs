use async_recursion::async_recursion;
use std::{
    collections::HashSet,
    path::{Path, PathBuf},
    sync::Arc,
};
use tokio::task::JoinHandle;

use structs::{
    Environment, EnvironmentConfig, EnvironmentFiles, RelatedEnvironmentFiles, RelatedPath,
    SharedEnvironmentConfig, SharedEnvironmentFiles,
};

use crate::errors::{self, Error};

pub mod structs;

#[async_recursion]
async fn recursively_collect_files(path: &Path) -> errors::Result<Vec<PathBuf>> {
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
    pub async fn from_options(
        name: &str,
        conf_dir: &str,
        no_root: bool,
    ) -> errors::Result<Environment> {
        if name == "shared" || name == "default" {
            return Err(Error::Descriptive(
                "{name} is a reserved name and cannot be used as an environment name.".to_string(),
            ));
        }

        let dir_path = Path::new(conf_dir);
        if !dir_path.try_exists()? {
            return Err(Error::Descriptive(format!(
                "Could not find the configuration directory at: {}, are you sure it exists?",
                conf_dir
            )));
        }

        let env_root = dir_path.join(name);
        if !env_root.try_exists()? {
            return Err(Error::Descriptive(format!(
                "No environment found with name: {}",
                name
            )));
        }

        let shared_path = dir_path.join("shared");
        let setup_path = env_root.join("setup.toml");
        let destroy_path = env_root.join("destroy.toml");
        let home_path = env_root.join("home");
        let root_path = env_root.join("root");

        let shared_home_path = shared_path.join("home");
        let shared_root_path = shared_path.join("root");

        let setup: EnvironmentConfig = confy::load_path(setup_path).map_err(|e| {
            errors::descriptive!(
                "Failed to load setup.toml, most likely due to a syntax error:\n{}",
                e
            )
        })?;

        let destroy: Option<EnvironmentConfig> = if destroy_path.try_exists()? {
            confy::load_path(destroy_path).map_err(|e| {
                errors::descriptive!(
                    "Failed to load destroy.toml, most likely due to a syntax error:\n{}",
                    e
                )
            })?
        } else {
            None
        };

        if !home_path.try_exists()? {
            return Err(Error::Descriptive(
                "No home directory found for environment.".to_string(),
            ));
        };

        let shared = if setup.info.use_shared && shared_path.try_exists()? {
            let shared_config: SharedEnvironmentConfig =
                confy::load_path(shared_path.join("setup.toml")).map_err(|e| {
                    errors::descriptive!(
                        "Failed to load shared/setup.toml, most likely due to a syntax error:\n{}",
                        e
                    )
                })?;

            if !shared_home_path.try_exists()? {
                return Err(Error::Descriptive(
                    "No shared home directory found.".to_string(),
                ));
            };

            let shared_root = if !no_root && shared_root_path.try_exists()? {
                Some(shared_root_path)
            } else {
                None
            };

            Some(SharedEnvironmentFiles {
                home: shared_home_path,
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
            home: home_path,
            shared,
            root,
            setup,
            destroy,
        })
    }

    pub async fn read(&self) -> errors::Result<EnvironmentFiles> {
        let self_arc = Arc::new(self.clone());
        type Handle = JoinHandle<errors::Result<(Vec<PathBuf>, Vec<PathBuf>)>>;

        let self_c = self_arc.clone();
        let home_handle: Handle = tokio::task::spawn(async move {
            let env_home_files = recursively_collect_files(&self_c.home).await?;
            let mut shared_home_files = Vec::<PathBuf>::new();

            if self_c.shared.is_some() {
                let shared = self_c.shared.as_ref().unwrap();
                let shared_home = &shared.home;
                let shared_files = recursively_collect_files(shared_home).await?;

                let set = HashSet::<PathBuf>::from_iter(
                    env_home_files
                        .clone()
                        .into_iter()
                        .map(|file| file.strip_prefix(&self_c.home).unwrap().to_path_buf()),
                );

                for shared_file in shared_files {
                    let stripped_shared_file = match shared_file.strip_prefix(shared_home) {
                        Ok(stripped) => stripped.to_path_buf(),
                        Err(_) => {
                            return Err(Error::Descriptive(format!(
                                "Could not strip prefix from shared file: {}",
                                shared_file.display()
                            )));
                        }
                    };

                    if !set.contains(&stripped_shared_file) {
                        shared_home_files.push(shared_file);
                    }
                }
            }

            Ok((env_home_files, shared_home_files))
        });

        let self_c = self_arc.clone();
        let root_handle: Handle = tokio::task::spawn(async move {
            let env_root_files = if let Some(root) = &self_c.root {
                recursively_collect_files(root).await?
            } else {
                Vec::new()
            };

            let mut shared_root_files = Vec::<PathBuf>::new();
            let shared_root_path_opt = &self_c
                .shared
                .as_ref()
                .and_then(|shared| shared.root.as_ref());

            if shared_root_path_opt.is_some() {
                let shared_root = shared_root_path_opt.unwrap();
                let shared_files = recursively_collect_files(shared_root).await?;

                let set =
                    HashSet::<PathBuf>::from_iter(env_root_files.clone().into_iter().map(|file| {
                        if let Some(root) = &self_c.root {
                            file.strip_prefix(root).unwrap().to_path_buf()
                        } else {
                            // Best case scenario, this should not be set as there are no root files
                            // in the environment directory, so shared root files should freely be added.
                            file
                        }
                    }));

                for shared_file in shared_files {
                    let relative_shared_file = match shared_file.strip_prefix(shared_root) {
                        Ok(stripped) => stripped.to_path_buf(),
                        Err(_) => {
                            return Err(Error::Descriptive(format!(
                                "Could not strip prefix from shared file: {}",
                                shared_file.display()
                            )));
                        }
                    };

                    if !set.contains(&relative_shared_file) {
                        shared_root_files.push(shared_file);
                    }
                }
            }

            Ok((env_root_files, shared_root_files))
        });

        let (home, shared_home) = home_handle.await??;
        let (root, shared_root) = root_handle.await??;

        Ok(EnvironmentFiles {
            home,
            root,
            shared_home,
            shared_root,
        })
    }
}

impl EnvironmentFiles {
    pub fn to_related(&self, env: &Environment) -> errors::Result<RelatedEnvironmentFiles> {
        let mut home = Vec::with_capacity(self.home.len() + self.shared_home.len());
        let mut root = Vec::with_capacity(self.root.len() + self.shared_root.len());

        let user_root_path = Path::new("/");
        let user_home_path = Path::new("/home").join(&env.setup.info.username);

        for file in &self.home {
            home.push(RelatedPath {
                source: file.to_path_buf(),
                destination: user_home_path.join(file.strip_prefix(&env.home).unwrap()),
            });
        }

        if let Some(root_path) = &env.root {
            for file in &self.root {
                root.push(RelatedPath {
                    source: file.to_path_buf(),
                    destination: user_root_path.join(file.strip_prefix(root_path).unwrap()),
                })
            }
        }

        if let Some(shared) = &env.shared {
            for file in &self.shared_home {
                home.push(RelatedPath {
                    source: file.to_path_buf(),
                    destination: user_home_path.join(file.strip_prefix(&shared.home).unwrap()),
                });
            }

            if let Some(shared_root_path) = &shared.root {
                for file in &self.shared_root {
                    root.push(RelatedPath {
                        source: file.to_path_buf(),
                        destination: user_root_path
                            .join(file.strip_prefix(shared_root_path).unwrap()),
                    });
                }
            }
        }

        Ok(RelatedEnvironmentFiles { home, root })
    }
}
