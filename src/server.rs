use crate::structure::{EpoxyServer, WispGo, WispJS, WispPy, WispServer};
use crate::util;
use anyhow::{Context, Result};
use std::path::PathBuf;
use std::process::{Child, Command};

impl WispJS {
    pub fn new() -> Self {
        Self {
            path: util::base().join("server/js"),
        }
    }
}

impl WispServer for WispJS {
    fn name(&self) -> &str {
        "wisp-js"
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

    fn run(&self, port: u16, log_file: &PathBuf) -> Result<Child> {
        util::run(
            "node",
            &["server.mjs", &port.to_string()],
            Some(&self.path),
            log_file,
        )
    }
}

impl WispPy {
    pub fn new(python: &str) -> Self {
        let path = util::base().join("server/python");
        let repo = path.join("wisp-server-python");
        let venv = path.join(format!(".venv_{}", python));
        let name = if python == "python3" {
            "wisp-server-python".to_string()
        } else {
            format!("wisp-server-python ({})", python)
        };

        Self {
            path,
            repo,
            venv,
            python: python.to_string(),
            name,
        }
    }
}

impl WispServer for WispPy {
    fn name(&self) -> &str {
        &self.name
    }

    fn install(&self) -> Result<()> {
        std::fs::create_dir_all(&self.path)?;
        if !self.repo.exists() {
            Command::new("git")
                .args(&[
                    "clone",
                    "https://github.com/MercuryWorkshop/wisp-server-python",
                ])
                .arg(&self.repo)
                .status()
                .context("Failed to clone wisp-server-python")?;
        }
        Command::new(&self.python)
            .args(&["-m", "venv"])
            .arg(&self.venv)
            .current_dir(&self.repo)
            .status()
            .context("Failed to create venv")?;
        let activate_cmd = format!(
            "source {}/bin/activate; pip3 install -e .",
            self.venv.display()
        );
        Command::new("bash")
            .args(&["-c", &activate_cmd])
            .current_dir(&self.repo)
            .status()
            .context("Failed to install Python package")?;
        Ok(())
    }

    fn check_install(&self) -> bool {
        self.venv.exists()
    }

    fn run(&self, port: u16, log_file: &PathBuf) -> Result<Child> {
        let cmd =
            format!(
            "source {}/bin/activate; python3 -m wisp.server --port={} --allow-loopback 2>&1 >'{}'",
            self.venv.display(), port, log_file.display()
        );
        Command::new("bash")
            .args(&["-c", &cmd])
            .current_dir(&self.repo)
            .spawn()
            .context("Failed to spawn Python server")
    }
}

impl EpoxyServer {
    pub fn new(threading: &str) -> Self {
        let path = util::base().join("server/rust");
        let epoxy_src = path.join("server");
        Self {
            path,
            epoxy_src,
            threading: threading.to_string(),
            name: format!("epoxy-server ({})", threading),
        }
    }
}

impl WispServer for EpoxyServer {
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
            .args(&["build", "--release"])
            .current_dir(&self.epoxy_src)
            .status()
            .context("Failed to build Rust server")?;
        Ok(())
    }

    fn check_install(&self) -> bool {
        self.path.join("target/release/epoxy-server").exists()
    }

    fn run(&self, port: u16, log_file: &PathBuf) -> Result<Child> {
        let config = format!(
            "[server]\nbind = [\"tcp\", \"127.0.0.1:{}\"]\nruntime = \"{}\"",
            port, self.threading
        );
        let config_path = self.epoxy_src.join("config.toml");
        std::fs::write(&config_path, config)?;
        util::run(
            self.path
                .join("target/release/epoxy-server")
                .to_str()
                .unwrap(),
            &[config_path.to_str().unwrap()],
            Some(&self.epoxy_src),
            log_file,
        )
    }
}

impl WispGo {
    pub fn new() -> Self {
        Self {
            path: util::base().join("server/go"),
        }
    }
}

impl WispServer for WispGo {
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
        Box::new(WispJS::new()),
        Box::new(WispPy::new("python3")),
        Box::new(EpoxyServer::new("singlethread")),
        Box::new(EpoxyServer::new("multithread")),
        Box::new(WispGo::new()),
    ]
}
