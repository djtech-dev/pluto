use crate::defs::{Transaction, Wallet};

#[derive(Debug)]
pub enum GetWalletError {
    NotExistingWallet,
    NotExistingTransaction,
    NetworkingError
}
#[derive(Debug)]
pub enum ShippingResult {
    Success
}

/// Internal Use Only (not used by the frontend)
fn search_wallet() -> Option<Result<Wallet, GetWalletError>> {
    todo!();
}
pub fn get_wallet() -> Result<Wallet, GetWalletError> {
    todo!();
}
pub fn get_transaction() -> Result<Transaction, GetWalletError> {
    todo!();
}

pub fn ship(t: Transaction) -> ShippingResult {
    println!("[1/3] Sending the transaction to the network...");
    
    println!("[2/3] Waiting for the transaction to be confirmed...");
    
    println!("[3/3] Transaction confirmed!");
    
    todo!();
}

