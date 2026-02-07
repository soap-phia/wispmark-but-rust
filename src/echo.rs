use crate::util;
use anyhow::{Context, Result};
use std::process::{Child, Command};

pub async fn get_echo() -> Result<()> {
    let echo_dir = util::base().join("echo");
    let echo_repo = echo_dir.join("tokio");

    tokio::fs::create_dir_all(&echo_dir).await?;

    if !echo_repo.exists() {
        println!("Installing echo...");
        Command::new("git")
            .args(&["clone", "https://github.com/tokio-rs/tokio"])
            .arg(&echo_repo)
            .status()
            .context("Failed to clone tokio")?;

        Command::new("cargo")
            .args(&["build", "--release", "--example", "echo-tcp"])
            .current_dir(&echo_repo)
            .status()
            .context("Failed to build echo")?;
    }

    Ok(())
}

pub fn run_echo() -> Result<Child> {
    let echo_repo = util::base().join("echo/tokio");

    Command::new("cargo")
        .args(&[
            "run",
            "--release",
            "--example",
            "echo-tcp",
            &format!("127.0.0.1:{}", util::ECHO_PORT),
        ])
        .current_dir(&echo_repo)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
        .context("Failed to run echo")
}
