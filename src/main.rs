mod rpc;
mod cli;

use rpc::get_epoch_info;
use clap::Parser;
use cli::{Cli, Commands};

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Epoch => {
            match get_epoch_info().await {
                Ok(info) => println!("Epoch: {}\nSlot Index: {}/{}\nAbsolute Slot: {}",
                                                info.epoch, info.slot_index, info.slots_in_epoch, info.absolute_slot),
                Err(err) => eprintln!("Error: {}", err)
            }
        }
    }
}
