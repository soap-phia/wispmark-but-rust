mod benchmark;
mod client;
mod echo;
mod embedded;
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
    #[arg(long)]
    base_dir: Option<PathBuf>,

    #[arg(long)]
    set_base_dir: Option<PathBuf>,

    #[arg(long)]
    show_config: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    if let Some(path) = args.set_base_dir {
        util::save_default_base_dir(path)?;
        return Ok(());
    }

    if args.show_config {
        match util::get_default_base_dir()? {
            Some(dir) => println!("Default base directory: {}", dir.display()),
            None => println!("No default base directory set"),
        }
        return Ok(());
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
    util::write_wispjs_files(&base_dir)?;
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
