mod cli;
mod kv;

use clap::Parser;
use cli::*;
use kv::KVStore;

fn main() {
    let cli = Cli::parse();
    let mut store = KVStore::new();
    match cli.command {
        Commands::Set { key, value } => {
            store.set(key, value);
        }
        Commands::Get { key } => {
            store.get(&key).unwrap();
        }
        Commands::Remove { key } => {
            store.remove(&key).unwrap();
        }
    }
}
