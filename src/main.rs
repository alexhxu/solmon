mod rpc;
mod cli;
mod output;

use rpc::*;
use clap::Parser;
use cli::{Cli, Commands};
use output::*;
use std::cmp::Reverse;

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Epoch => {
            match get_epoch_info().await {
                Ok(info) => {
                    print_header("Epoch Information");
                    print_kv("Epoch", info.epoch);
                    print_kv("Slot", format!("{}/{}", info.slot_index, info.slots_in_epoch));
                    print_kv("Absolute Slot", info.absolute_slot);
                },
                Err(err) => eprintln!("Error: {}", err)
            }
        },
        Commands::Status => {
            print_title("Cluster Status");
        
            match get_performance_samples().await {
                Ok(samples) => {
                    if let Some(sample) = samples.first() {
                        let tps = sample.num_transactions / sample.sample_period_secs;
                        print_kv("Recent TPS", tps);
                        print_kv("Sample Slot", sample.slot);
                    }
                }
                Err(err) => eprintln!("Failed to fetch performance samples: {}", err),
            }
        
            match get_block_production().await {
                Ok(prod) => {
                    print_kv("Block Production Range", format!("{} - {}", prod.range.first_slot, prod.range.last_slot));
                    println!();
                    print_title("Top 5 Validators by Blocks Produced");
                    println!("{:<45} {:>10} {:>10} {:>10}", "Validator", "Assigned", "Produced", "Success %");
                    println!("{}", "-".repeat(80));
        
                    let mut rows: Vec<_> = prod.by_identity.iter().collect();
                    rows.sort_by_key(|(_, stats)| Reverse(stats.produced));
                    for (validator, stats) in rows.iter().take(5) {
                        let success_rate = if stats.assigned > 0 {
                            100.0 * (stats.produced as f64) / (stats.assigned as f64)
                        } else {
                            0.0
                        };
                        println!("{:<45} {:>10} {:>10} {:>9.2}%", validator, stats.assigned, stats.produced, success_rate);
                    }
                }
                Err(err) => eprintln!("Failed to fetch block production: {}", err),
            }
        },
        Commands::Validator { pubkey } => {
            print_title(&format!("Validator Info: {}", pubkey));

            match get_vote_accounts().await {
                Ok(accounts) => {
                    if let Some(account) = accounts.current.iter().find(|v| v.node_pubkey == pubkey) {
                        print_kv("Stake", format!("{:.2} SOL", account.activated_stake as f64 / 1_000_000_000.0));
                        print_kv("Commission", format!("{}%", account.commission));
                        print_kv("Last Vote Slot", account.last_vote);
                        print_kv("Root Slot", account.root_slot);
                    } else if let Some(account) = accounts.delinquent.iter().find(|v| v.node_pubkey == pubkey) {
                        println!("Validator is delinquent.");
                        print_kv("Stake", format!("{:.2} SOL", account.activated_stake as f64 / 1_000_000_000.0));
                        print_kv("Commission", format!("{}%", account.commission));
                        print_kv("Last Vote Slot", account.last_vote);
                        print_kv("Root Slot", account.root_slot);
                    } else {
                        eprintln!("Validator not found in current or delinquent vote accounts.");
                    }
                }
                Err(err) => eprintln!("Failed to fetch vote accounts: {}", err),
            }
        }
    }
}
