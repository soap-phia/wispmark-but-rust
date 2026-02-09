use crate::structure::{BenchmarkResult, BenchmarkResults, WispClient, WispServer};
use crate::{client, echo, server, util};
use anyhow::Result;
use std::path::PathBuf;
use std::process::Child;
use tokio::time::{sleep, Duration};

fn kill(child: &mut Child) {
    let _ = child.kill();
    let _ = child.wait();
}

pub async fn benchmark(test: u64) -> Result<BenchmarkResults> {
    let mut echo_process = echo::run_echo()?;
    util::wait_for_tcp(util::ECHO_PORT, util::SERVER_TIMEOUT).await?;
    let servers = server::get_implementations();
    let clients = client::get_implementations();

    for server in &servers {
        if !server.check_install() {
            println!("Installing {}", server.name());
            server.install()?;
        }
    }

    for client in &clients {
        if !client.check_install() {
            println!("Installing {}", client.name());
            client.install()?;
        }
    }

    let log_dir = util::base().join("log");
    tokio::fs::create_dir_all(&log_dir).await?;
    let mut results = BenchmarkResults::new();
    for server in &servers {
        let server_name = server.name().to_string();

        for client in &clients {
            let client_name = client.name().to_string();
            println!("Testing {} with {}", server_name, client_name);

            let server_log = log_dir.join(format!("SERVER_{}_{}.log", server_name, client_name));
            let client_log = log_dir.join(format!("CLIENT_{}_{}.log", server_name, client_name));

            let result = single(
                server.as_ref(),
                client.as_ref(),
                &server_log,
                &client_log,
                test,
            )
            .await;

            results.add(server_name.clone(), client_name.clone(), result);
        }
    }

    kill(&mut echo_process);
    println!("WispMark has finished.");

    Ok(results)
}

async fn single(
    server: &dyn WispServer,
    client: &dyn WispClient,
    server_log: &PathBuf,
    client_log: &PathBuf,
    test: u64,
) -> BenchmarkResult {
    if let Err(e) = util::kill(util::WISP_PORT) {
        eprintln!("Warning: Failed to stop existing server: {}", e);
    }
    sleep(Duration::from_secs(1)).await;
    let mut server_process = match server.run(util::WISP_PORT, server_log) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Error starting server: {}", e);
            return BenchmarkResult::Failed("Server failed to start".to_string());
        }
    };

    if let Err(e) = util::wait_for_http(util::WISP_PORT, util::SERVER_TIMEOUT).await {
        eprintln!("Error: Server not ready: {}", e);
        kill(&mut server_process);
        return BenchmarkResult::Failed("Server timeout".to_string());
    }

    let mut client_processes = match client.run(util::WISP_PORT, util::ECHO_PORT, client_log) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Error starting clients: {}", e);
            kill(&mut server_process);
            return BenchmarkResult::Failed("Client failed to start".to_string());
        }
    };

    sleep(Duration::from_secs(1)).await;
    println!("Recording speeds for {}s...", test);
    let result = match util::get_bandwidth(util::ECHO_PORT, test).await {
        Ok(speed) => {
            let mib_s = speed / (1024.0 * 1024.0);
            println!("Result: {:.2} MiB/s", mib_s);
            BenchmarkResult::Success(mib_s)
        }
        Err(e) => {
            eprintln!("Error measuring bandwidth: {}", e);
            BenchmarkResult::Failed("DNF".to_string())
        }
    };

    kill(&mut server_process);
    for client in &mut client_processes {
        kill(client);
    }

    result
}

pub fn format_results(results: &BenchmarkResults, cpu_info: &str, test: u64) -> String {
    let mut output = String::new();

    output.push_str(&format!("CPU: {}\n\n", cpu_info));
    output.push_str(&format!("Test duration: {}s\n", test));
    let mut table = vec![vec!["".to_string()]];
    for client in &results.client_order {
        table[0].push(client.clone());
    }

    for server in &results.server_order {
        let mut row = vec![server.clone()];
        for client in &results.client_order {
            let result = results
                .get(server, client)
                .map(|r| r.to_string())
                .unwrap_or_else(|| "N/A".to_string());
            row.push(result);
        }
        table.push(row);
    }

    output.push_str(&format_table(&table));
    output
}

fn format_table(table: &[Vec<String>]) -> String {
    let mut col_widths = vec![0; table[0].len()];
    for row in table {
        for (i, cell) in row.iter().enumerate() {
            col_widths[i] = col_widths[i].max(cell.len());
        }
    }

    let mut output = String::new();

    for (row_idx, row) in table.iter().enumerate() {
        output.push('|');

        for (i, cell) in row.iter().enumerate() {
            output.push(' ');
            output.push_str(&format!("{:width$}", cell, width = col_widths[i]));
            output.push_str(" |");
        }

        output.push('\n');

        if row_idx == 0 {
            output.push('|');
            for &width in &col_widths {
                output.push_str(&"-".repeat(width + 2));
                output.push('|');
            }
            output.push('\n');
        }
    }

    output
}
