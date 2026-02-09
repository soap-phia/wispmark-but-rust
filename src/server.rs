use crate::structure::{WispGoOld, WispGoNew, WispServer};
use crate::util;
use anyhow::{Context, Result};
use std::path::PathBuf;
use std::process::{Child, Command};

impl WispGoOld {
    pub fn new() -> Self {
        Self {
            path: util::base().join("server/goOld"),
        }
    }
}

impl WispGoNew {
    pub fn new() -> Self {
        Self {
            path: util::base().join("server/goNew"),
        }
    }
}

impl WispServer for WispGoOld {
    fn name(&self) -> &str {
        "go-wisp"
    }

    fn install(&self) -> Result<()> {
        if !self.path.exists() {
            Command::new("git")
                .args(&["clone", "https://github.com/TheFalloutOf76/go-wisp"])
                .arg(&self.path)
                .status()
                .context("Failed to clone go-wisp")?;
        }
        Command::new("go")
            .args(&["get", "."])
            .current_dir(&self.path)
            .status()
            .context("Failed to go get")?;
        Command::new("go")
            .args(&["build", "-ldflags", "-s -w", "-o", "go-wisp", "main.go"])
            .current_dir(&self.path)
            .status()
            .context("Failed to build Go server")?;
        Ok(())
    }

    fn check_install(&self) -> bool {
        self.path.join("go-wisp").exists()
    }

    fn run(&self, port: u16, log_file: &PathBuf) -> Result<Child> {
        let config = serde_json::json!({
            "port": port.to_string(),
            "disableUDP": true,
            "tcpBufferSize": 131072,
            "bufferRemainingLength": 256,
            "tcpNoDelay": false,
            "websocketTcpNoDelay": false,
            "blacklist": { "hostnames": [] },
            "whitelist": { "hostnames": [] },
            "proxy": "",
            "websocketPermessageDeflate": false,
            "dnsServer": ""
        });
        let config_path = self.path.join("config.json");
        std::fs::write(&config_path, serde_json::to_string_pretty(&config)?)?;
        util::run("./go-wisp", &[], Some(&self.path), log_file)
    }
}

impl WispServer for WispGoNew {
    fn name(&self) -> &str {
        "go-wisp"
    }

    fn install(&self) -> Result<()> {
        if !self.path.exists() {
            Command::new("git")
                .args(&["clone", "https://github.com/TheFalloutOf76/go-wisp"])
                .arg(&self.path)
                .status()
                .context("Failed to clone go-wisp")?;
        }
        Command::new("go")
            .args(&["get", "."])
            .current_dir(&self.path)
            .status()
            .context("Failed to go get")?;
        Command::new("go")
            .args(&["build", "-ldflags", "-s -w", "-o", "go-wisp", "main.go"])
            .current_dir(&self.path)
            .status()
            .context("Failed to build Go server")?;
        Ok(())
    }

    fn check_install(&self) -> bool {
        self.path.join("go-wisp").exists()
    }

    fn run(&self, port: u16, log_file: &PathBuf) -> Result<Child> {
        let config = serde_json::json!({
            "port": port.to_string(),
            "disableUDP": true,
            "tcpBufferSize": 131072,
            "bufferRemainingLength": 256,
            "tcpNoDelay": false,
            "websocketTcpNoDelay": false,
            "blacklist": { "hostnames": [] },
            "whitelist": { "hostnames": [] },
            "proxy": "",
            "websocketPermessageDeflate": false,
            "dnsServer": ""
        });
        let config_path = self.path.join("config.json");
        std::fs::write(&config_path, serde_json::to_string_pretty(&config)?)?;
        util::run("./go-wisp", &[], Some(&self.path), log_file)
    }
}

pub fn get_implementations() -> Vec<Box<dyn WispServer>> {
    vec![
        Box::new(WispGoNew::new()),
        Box::new(WispGoOld::new()),
    ]
}
