mod benchmark;
mod client;
mod echo;
mod server;
mod structure;
mod util;

use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "wispmark")]
#[command(about = "A benchmarking tool for Wisp protocol implementations")]
struct Args {
    #[arg(long, default_value = "10")]
    duration: u64,
    #[arg(long, default_value = "wispmark-results.md")]
    output: PathBuf,
    #[arg(long, default_value = "true")]
    print_md: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    util::sudo()?;
    echo::get_echo().await?;
    let results = benchmark::benchmark(args.duration).await?;
    let cpu_info = util::get_cpu_info()?;
    let output = benchmark::format_results(&results, &cpu_info, args.duration);
    if args.print_md {
        println!("{}", output);
    }
    tokio::fs::write(&args.output, output).await?;
    println!("\nMarkdown results written to: {}", args.output.display());

    Ok(())
}
