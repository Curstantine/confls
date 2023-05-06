use anyhow::Ok;
use std::io::{self, Write};

/// This function runs on a newly spawned blocking thread.
pub async fn install_packages(
    mut env_packages: Vec<String>,
    shared_packages: Option<Vec<String>>,
) -> anyhow::Result<()> {
    let packages = match shared_packages {
        Some(mut shared_packages) => {
            env_packages.append(&mut shared_packages);
            env_packages
        }
        None => env_packages,
    };

    tokio::task::spawn_blocking(move || {
        println!(
            "The following packages will be installed:\n ({} packages) {}",
            packages.len(),
            packages.join(" ")
        );

        let mut input = String::new();
        print!("Do you want to continue? [Y/n] ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut input)?;

        if input.trim().to_lowercase().as_str() != "y" {
            println!(
                "Exiting. You can explicitly install these packages with paru -Syu {}",
                packages.join(" ")
            );
            std::process::exit(0);
        }

        std::process::Command::new("paru")
            .args(["-Syu", "--noconfirm"])
            .args(&packages)
            .spawn()?
            .wait()?;

        Ok(())
    })
    .await?
}
