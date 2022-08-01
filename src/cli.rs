/*
Commands:
    - `wallet-new` : Create a new wallet.
    - `wallet-list` : List all the wallets.
    - `verify` : Enter the verification mode.
    - `send` : Send an amount of tokens from one wallet to another.
    - `balance` : Check the balance of a wallet.
    - `transactions` : List transactions
    - `donate` : Dontate to the Solar project. (0x1)

    - `mindstate check` : Check the state of the network.
    - `mindstate avability` : Check the avability of a token in the market.
    - `mindstate price` : Check the price of a token in the market.
    - `mindstate swap` : Swap a Solar token for another.
    - `mindstate eth-sell`: Sell a Solar token for ETH.
    - `mindstate eth-buy`: Buy a Solar token for ETH.
*/

pub use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(value_parser)]
    name: Option<String>,

    //#[clap(short, long, value_parser, value_name = "FILE")]
    //config: Option<PathBuf>,

    //#[clap(short, long, action = clap::ArgAction::Count)]
    //debug: u8,

    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Create a new wallet
    WalletNew {}, 
    /// List all the wallets
    WalletList {},
    /// Enter the verifying mode
    Verify {},
    /// Send an amount of tokens from one wallet to another
    Send {},
    /// Check the balance of a wallet
    Balance {},
    /// List transactions of a wallet
    Transactions {},
    /// Donate to the Solar project. (address: 0x2)
    Donate {},    
}

pub fn elaborate(cli: &mut Cli) {
    match &cli.command {
        Some(Commands::WalletNew {}) => {
            println!("wallet-new");
        }
        Some(Commands::WalletList {}) => {
            println!("wallet-list");
        }
        Some(Commands::Verify {}) => {
            println!("verify");
        }
        Some(Commands::Send {}) => {
            println!("send");
        }
        Some(Commands::Balance {}) => {
            println!("balance");
        }
        Some(Commands::Transactions {}) => {
            println!("transactions");
        }
        Some(Commands::Donate {}) => {
            println!("donate");
        }
        None => {
            // Print help message
            println!("pluto - CLI client for Pluto");
            println!("");
            println!("Use --help for more information");
        }
    }
}