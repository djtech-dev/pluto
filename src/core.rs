use crate::{defs::Transaction, net_client};

pub fn send(amount: u64, from: &str, to: &str) {
    let wallet_from = serde_json::from_str(from);
    let wallet_to = serde_json::from_str(to);
    
    let transaction = Transaction {
        from: wallet_from.unwrap(),
        to: wallet_to.unwrap(),
        checked_by: None,
        amount: amount,
    };
    println!("Transaction result: {:#?}", net_client::ship(transaction));
}

pub fn balance_local(_wallet: &str) {
    /*let wallet;
    let balance = wallet.calculate_amount();
    println!("Balance of the wallet: {}", balance);*/
    todo!();
}
pub fn balance_remote(_wallet: &str) {
    /*let wallet;
    let balance = wallet.calculate_amount();
    println!("Balance of the wallet: {}", balance);*/
    todo!();    
}