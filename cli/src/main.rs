use soroban_sdk::{Address};

use clap::Parser;

#[derive(Parser)]
struct Cli {
    id: String,
}

fn main() {
    let args = Cli::parse();
    let address = Address::from_contract_id(&args.id);
    println!(address.into());
}