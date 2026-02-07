use crate::structure::{EpoxyClient, WispClient, WispNode};
use crate::util;
use anyhow::{Context, Result};
use std::path::PathBuf;
use std::process::{Child, Command};

impl WispNode {
    pub fn new(streams: usize, instances: usize) -> Self {
        let path = util::base().join("client/js");
        let name = if instances == 1 {
            format!("wisp-js ({})", streams)
        } else {
            format!("wisp-js ({}x{})", instances, streams)
        };

        Self {
            path,
            streams,
            instances,
            name,
        }
    }
}

impl WispClient for WispNode {
    fn name(&self) -> &str {
        &self.name
    }

    fn install(&self) -> Result<()> {
        std::fs::create_dir_all(&self.path)?;
        Command::new("npm")
            .arg("i")
            .current_dir(&self.path)
            .status()
            .context("Failed to run npm install")?;
        Ok(())
    }

    fn check_install(&self) -> bool {
        self.path.join("node_modules").exists()
    }

    fn run(&self, server_port: u16, target_port: u16, log_file: &PathBuf) -> Result<Vec<Child>> {
        let mut children = Vec::with_capacity(self.instances);

        for i in 0..self.instances {
            let instance_log = if self.instances > 1 {
                let parent = log_file.parent().unwrap();
                let stem = log_file.file_stem().unwrap().to_string_lossy();
                let ext = log_file.extension().unwrap_or_default().to_string_lossy();
                parent.join(format!("{}_{}.{}", stem, i, ext))
            } else {
                log_file.clone()
            };

            let child = util::run(
                "node",
                &[
                    "client.mjs",
                    &server_port.to_string(),
                    &target_port.to_string(),
                    &self.streams.to_string(),
                ],
                Some(&self.path),
                &instance_log,
            )?;

            children.push(child);
        }

        Ok(children)
    }
}

impl EpoxyClient {
    pub fn new(streams: usize, instances: usize) -> Self {
        let path = util::base().join("client/rust");
        let epoxy_src = path.join("simple-wisp-client");
        let name = if instances == 1 {
            format!("wisp-mux ({})", streams)
        } else {
            format!("wisp-mux ({}x{})", instances, streams)
        };

        Self {
            path,
            epoxy_src,
            streams,
            instances,
            name,
        }
    }
}

impl WispClient for EpoxyClient {
    fn name(&self) -> &str {
        &self.name
    }

    fn install(&self) -> Result<()> {
        if !self.path.exists() {
            Command::new("git")
                .args(&["clone", "https://github.com/MercuryWorkshop/epoxy-tls"])
                .arg(&self.path)
                .status()
                .context("Failed to clone epoxy-tls")?;
        }

        Command::new("cargo")
            .args(&["b", "-r"])
            .current_dir(&self.epoxy_src)
            .status()
            .context("Failed to build Rust client")?;

        Ok(())
    }

    fn check_install(&self) -> bool {
        self.path.join("target/release/simple-wisp-client").exists()
    }

    fn run(&self, server_port: u16, target_port: u16, log_file: &PathBuf) -> Result<Vec<Child>> {
        let mut children = Vec::with_capacity(self.instances);

        for i in 0..self.instances {
            let instance_log = if self.instances > 1 {
                let parent = log_file.parent().unwrap();
                let stem = log_file.file_stem().unwrap().to_string_lossy();
                let ext = log_file.extension().unwrap_or_default().to_string_lossy();
                parent.join(format!("{}_{}.{}", stem, i, ext))
            } else {
                log_file.clone()
            };

            let binary_path = self.path.join("target/release/simple-wisp-client");
            let child = util::run(
                binary_path.to_str().unwrap(),
                &[
                    "-w",
                    &format!("ws://127.0.0.1:{}/", server_port),
                    "-t",
                    &format!("127.0.0.1:{}", target_port),
                    "-s",
                    &self.streams.to_string(),
                    "-p",
                    "50",
                ],
                Some(&self.epoxy_src),
                &instance_log,
            )?;

            children.push(child);
        }

        Ok(children)
    }
}

pub fn get_implementations() -> Vec<Box<dyn WispClient>> {
    vec![
        Box::new(WispNode::new(10, 1)),
        Box::new(WispNode::new(10, 5)),
        Box::new(EpoxyClient::new(10, 1)),
        Box::new(EpoxyClient::new(10, 5)),
    ]
}
