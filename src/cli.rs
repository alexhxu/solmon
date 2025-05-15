use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "solmon")]
#[command(about = "Solana Monitoring CLI", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands
}

#[derive(Subcommand)]
pub enum Commands {
    Epoch,
    Status,
    Validator {
        pubkey: String
    }
}
