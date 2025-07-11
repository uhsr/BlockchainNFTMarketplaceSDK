// src/main.rs
/*
 * Main executable for BlockchainNFTMarketplaceSDK
 */

use clap::Parser;
use blockchainnftmarketplacesdk::{Result, run};

#[derive(Parser)]
#[command(version, about = "BlockchainNFTMarketplaceSDK - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
