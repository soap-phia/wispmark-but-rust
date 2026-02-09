use anyhow::Result;
use std::collections::HashMap;
use std::path::PathBuf;
use std::process::Child;

pub trait WispClient: Send + Sync {
    fn name(&self) -> &str;
    fn install(&self) -> Result<()>;
    fn check_install(&self) -> bool;
    fn run(&self, server_port: u16, target_port: u16, log_file: &PathBuf) -> Result<Vec<Child>>;
}

pub struct WispNode {
    pub path: PathBuf,
    pub streams: usize,
    pub instances: usize,
    pub name: String,
}

pub struct EpoxyClient {
    pub path: PathBuf,
    pub epoxy_src: PathBuf,
    pub streams: usize,
    pub instances: usize,
    pub name: String,
}

pub trait WispServer: Send + Sync {
    fn name(&self) -> &str;
    fn install(&self) -> Result<()>;
    fn check_install(&self) -> bool;
    fn run(&self, port: u16, log_file: &PathBuf) -> Result<Child>;
}

pub struct WispJS {
    pub path: PathBuf,
}

pub struct WispPy {
    pub path: PathBuf,
    pub repo: PathBuf,
    pub venv: PathBuf,
    pub python: String,
    pub name: String,
}

pub struct EpoxyServer {
    pub path: PathBuf,
    pub epoxy_src: PathBuf,
    pub threading: String,
    pub name: String,
}

pub struct WispGo {
    pub path: PathBuf,
}

#[derive(Debug, Clone)]
pub enum BenchmarkResult {
    Success(f64),
    Failed(String),
}

pub struct BenchmarkResults {
    pub results: HashMap<String, HashMap<String, BenchmarkResult>>,
    pub server_order: Vec<String>,
    pub client_order: Vec<String>,
}

impl BenchmarkResult {
    pub fn to_string(&self) -> String {
        match self {
            BenchmarkResult::Success(speed) => format!("{:.2} MiB/s", speed),
            BenchmarkResult::Failed(reason) => reason.clone(),
        }
    }
}

impl BenchmarkResults {
    pub fn new() -> Self {
        Self {
            results: HashMap::new(),
            server_order: Vec::new(),
            client_order: Vec::new(),
        }
    }

    pub fn add(&mut self, server: String, client: String, result: BenchmarkResult) {
        if !self.server_order.contains(&server) {
            self.server_order.push(server.clone());
        }
        if !self.client_order.contains(&client) {
            self.client_order.push(client.clone());
        }

        self.results
            .entry(server)
            .or_insert_with(HashMap::new)
            .insert(client, result);
    }

    pub fn get(&self, server: &str, client: &str) -> Option<&BenchmarkResult> {
        self.results.get(server)?.get(client)
    }
}
