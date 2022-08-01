use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Wallet {
    pub id: u128,
    pub transactions: Vec<Transaction>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClientString {
    pub ip: String,
    pub port: u16,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Client {
    pub cs: ClientString,
    pub wallets: Vec<Wallet>,
    pub connects: Vec<ClientString>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct LocalClient {
    pub wallets: Vec<Wallet>,
    pub connects: Vec<ClientString>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Transaction {
    pub from: Wallet,
    pub to: Wallet,
    pub checked_by: Option<Wallet>,
    pub amount: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Spawn {
    pub checked_transaction: Transaction,
    pub spawned_by: Wallet,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Operation {
    Spawn(Spawn),
    Transfer(Transaction),
}

impl PartialEq for Wallet {
    fn eq(&self, other: &Wallet) -> bool {
        self.id == other.id
    }
}

pub enum Tokens {
    // First token of Project Solar.
    // Low value, 1 coin is given to all the client that verify a transaction (so a transiction generates 2 coin that are given to 2 clients).
    // Minimum value of a transaction is 2 coin.
    // Not fractional.
    Pluto,
    
    // ~Will be avaiable with pluto v2.0~
    // Second token of Project Solar.
    // High value, risky asset.
    // The verification of a transaction will be rewarded with the equivalent of 0.25-1.75 Pluto coin (the hash of the transaction is used as a seed) 
    // In the future, it will backed by external investments (cryptos, stocks and real estate).
    // Minimum value of a transaction is 0.0002 coin.
    Neptune,

    // ~Will be avaiable with pluto v3.0~ (It requires a way to calculate the value in dollars of the coin)
    // Third token of Project Solar.
    // Stablecoin, 1 coin is equivalent to 1 dollar.
    // Verification of a transaction will be rewarded with 1 Pluto coin or the equivalent in Neptune. 
    // Backed by the Uranus Community Vault (a wallet that recives the operation costs)
    // Every transaction requires 5 Pluto coins or the equivalent in Uranus (this is the operation costs)
    // In the future, Uranus will also be backed by Neptune.
    // Minimum value of a transaction is 0.5 coin.
    Uranus, 

    // ~Will be previewed with pluto v3.0 and fully released with pluto v4.0~
    // Fourth token of Project Solar.
    // 
    // Minimum value of a transaction is 1 coin.
    Mercury,
    
    // NOTE: pluto v2.0 will also include Sprinting fee
    //   | An user can add a certain amount of tokens to his transaction as a Sprinting fee.
    //   | The transaction with the highest Sprinting fee will be chosen first by the verifiers.
    //   | The Sprinting fee will then be splitted equally between the clients that verifies the transaction.
}
