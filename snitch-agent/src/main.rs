mod config;
mod collect;

use config::{load_config, AgentConfiguration};
use anyhow::Result;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    config: String,

    #[arg(short, long, default_value = "info")]
    log_level: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
    println!("{:?}", args);
    let cfg: AgentConfiguration = load_config(args.config)?;
    println!("{:?}", cfg);
    // initialize agent runtime with cfg
    Ok(())
}
