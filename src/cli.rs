use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "kv_cli",
    version = "1.0",
    author = "Reid Xanders",
    about = "Provides the KV service."
)]
pub struct Cli {
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Clone)]
pub enum Commands {
    Set {
        #[arg(short, long)]
        key: String,
        #[arg(short, long)]
        value: String,
    },

    Get {
        #[arg(short, long)]
        key: String,
    },

    Remove {
        #[arg(short, long)]
        key: String,
    },
}
