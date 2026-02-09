mod benchmark;
mod client;
mod echo;
mod server;
mod structure;
mod util;

use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "wispmark")]
#[command(about = "A benchmarking tool for Wisp protocol implementations")]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
    #[arg(long, default_value = "10")]
    duration: u64,
    #[arg(long, default_value = "wispmark-results.md")]
    output: PathBuf,
    #[arg(long, default_value = "true")]
    print_md: bool,
    #[arg(long)]
    base_dir: Option<PathBuf>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    SetBaseDir {
        path: PathBuf,
    },
    ShowConfig,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    
    if let Some(command) = args.command {
        match command {
            Commands::SetBaseDir { path } => {
                util::save_default_base_dir(path)?;
                return Ok(());
            }
            Commands::ShowConfig => {
                match util::get_default_base_dir()? {
                    Some(dir) => println!("Default base directory: {}", dir.display()),
                    None => println!("No default base directory set."),
                }
                return Ok(());
            }
        }
    }
    
    util::sudo()?;
    
    let base_dir = if let Some(dir) = args.base_dir {
		dir
    } else if let Some(dir) = util::get_default_base_dir()? {
        println!("Using base directory: {}", dir.display());
        dir
    } else {
        std::env::current_dir()?
    };
    
    util::set_base_dir(base_dir)?;
    
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
