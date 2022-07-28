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
    pub connects: Vec<ClientString>
}
#[derive(Serialize, Deserialize, Debug)]
pub struct LocalClient {
    pub wallets: Vec<Wallet>,
    pub connects: Vec<ClientString>
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