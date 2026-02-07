use anyhow::{Context, Result, anyhow};
use regex::Regex;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};
use tokio::net::TcpStream;
use tokio::time::sleep;
use once_cell::sync::Lazy;

pub const WISP_PORT: u16 = 6001;
pub const ECHO_PORT: u16 = 6002;
pub const SERVER_TIMEOUT: u64 = 5;

static P_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r":(\d+).+?(\d+)/").unwrap()
});

static IFT_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"Cumulative.+?:.+?([\d.]+)([A-Z]+)\n").unwrap()
});

static CPU_NAME_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"model name.+?: (.+?)\n").unwrap()
});

static CPU_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"processor.+?: (.+?)\n").unwrap()
});

pub fn base() -> PathBuf {
    std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
}

pub fn sudo() -> Result<()> {
    let status = Command::new("sudo")
        .arg("true")
        .status()
        .context("Failed to run sudo")?;
    
    if !status.success() {
        return Err(anyhow!("Failed to run sudo"));
    }
    Ok(())
}

pub async fn wait_for_http(port: u16, _timeout_secs: u64) -> Result<()> {
    let timeout = Duration::from_secs(SERVER_TIMEOUT);
    let start = Instant::now();
    
    while start.elapsed() < timeout {
        match reqwest::get(format!("http://127.0.0.1:{}/", port)).await {
            Ok(_) => return Ok(()),
            Err(_) => sleep(Duration::from_millis(500)).await,
        }
    }
    
    Err(anyhow!("Server failed to start"))
}

pub async fn wait_for_tcp(port: u16, _timeout_secs: u64) -> Result<()> {
    let timeout = Duration::from_secs(SERVER_TIMEOUT);
    let start = Instant::now();
    
    while start.elapsed() < timeout {
        match TcpStream::connect(format!("127.0.0.1:{}", port)).await {
            Ok(_) => return Ok(()),
            Err(_) => sleep(Duration::from_millis(500)).await,
        }
    }
    
    Err(anyhow!("Failed to start TCP server."))
}

pub fn kill(port: u16) -> Result<()> {
    let output = Command::new("sudo")
        .args(&["netstat", "-tulpn"])
        .output()
        .context("Failed to run netstat")?;
    
    let netstat_out = String::from_utf8_lossy(&output.stdout);
    
    for cap in P_REGEX.captures_iter(&netstat_out) {
        if let (Some(port_match), Some(pid_match)) = (cap.get(1), cap.get(2)) {
            if port_match.as_str() == port.to_string() {
                let pid = pid_match.as_str();
                let _ = Command::new("kill")
                    .args(&["-s", "SIGTERM", pid])
                    .status();
            }
        }
    }
    
    Ok(())
}

pub async fn get_bandwidth(port: u16, duration: u64) -> Result<f64> {
    let start = Instant::now();
    
    let output = Command::new("timeout")
        .arg((duration * 2).to_string())
        .arg("sudo")
        .arg("iftop")
        .args(&[
            "-i", "lo",
            "-f", &format!("port {}", port),
            "-t",
            "-s", &duration.to_string(),
            "-B"
        ])
        .stderr(Stdio::null())
        .output()
        .context("Failed to run iftop")?;
    
    let end = Instant::now();
    let elapsed = (end - start).as_secs_f64();
    
    let iftop_out = String::from_utf8_lossy(&output.stdout);
    
    let cap = IFT_REGEX.captures(&iftop_out)
        .ok_or_else(|| anyhow!("Failed to parse iftop output"))?;
    
    let amount: f64 = cap[1].parse()
        .context("Failed to parse bandwidth amount")?;
    let unit = &cap[2];
    
    let multiplier = match unit {
        "B" => 1.0,
        "KB" => 1024.0,
        "MB" => 1024.0 * 1024.0,
        "GB" => 1024.0 * 1024.0 * 1024.0,
        "TB" => 1024.0 * 1024.0 * 1024.0 * 1024.0,
        _ => return Err(anyhow!("Unknown unit: {}", unit)),
    };
    
    Ok(amount * multiplier / elapsed)
}

fn is_wsl() -> bool {
    if let Ok(entries) = std::fs::read_dir("/proc/sys/fs/binfmt_misc/") {
        for entry in entries.flatten() {
            if entry.file_name().to_string_lossy().starts_with("WSL") {
                return true;
            }
        }
    }
    false
}

pub fn get_cpu_info() -> Result<String> {
    let cpu_name = if is_wsl() {
        let output = Command::new("/mnt/c/Windows/System32/WindowsPowershell/v1.0/powershell.exe")
            .args(&["-command", "Get-CimInstance -ClassName Win32_Processor | Select-Object -ExpandProperty Name"])
            .output()
            .context("Failed to get CPU info from PowerShell")?;
        String::from_utf8_lossy(&output.stdout).trim().to_string()
    } else {
        let cpuinfo = std::fs::read_to_string("/proc/cpuinfo")
            .context("Failed to read /proc/cpuinfo")?;
        
        CPU_NAME_REGEX.captures(&cpuinfo)
            .and_then(|cap| cap.get(1))
            .map(|m| m.as_str().to_string())
            .unwrap_or_else(|| {
                let arch = Command::new("uname")
                    .arg("-m")
                    .output()
                    .ok()
                    .and_then(|o| String::from_utf8(o.stdout).ok())
                    .unwrap_or_else(|| "unknown".to_string())
                    .trim()
                    .to_string();
                format!("Unknown {} CPU", arch)
            })
    };
    
    let cpuinfo = std::fs::read_to_string("/proc/cpuinfo")
        .context("Failed to read /proc/cpuinfo")?;
    let cpu_count = CPU_REGEX.find_iter(&cpuinfo).count();
    
    Ok(format!("{} (x{})", cpu_name, cpu_count))
}

pub fn run(
    command: &str,
    args: &[&str],
    working_dir: Option<&PathBuf>,
    log_file: &PathBuf,
) -> Result<std::process::Child> {
    let log = std::fs::File::create(log_file)
        .context("Failed to create log file")?;
    
    let mut cmd = Command::new(command);
    cmd.args(args)
        .stdout(Stdio::from(log.try_clone()?))
        .stderr(Stdio::from(log));
    
    if let Some(dir) = working_dir {
        cmd.current_dir(dir);
    }
    
    cmd.spawn().context("Failed to run command")
}