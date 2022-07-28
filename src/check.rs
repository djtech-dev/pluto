use crate::defs::{Transaction, Wallet, Client, LocalClient};

pub trait CheckHistory {
    fn check_history(&mut self);
}
impl CheckHistory for Client {
    fn check_history(&mut self) {
        for w in &mut self.wallets {
            w.check_history();
        }
    }
}
impl CheckHistory for LocalClient {
    fn check_history(&mut self) {
        for w in &mut self.wallets {
            w.check_history();
        }
    }
}
impl CheckHistory for Wallet {
    fn check_history(&mut self) {
        for t in &mut self.transactions {
            t.check_history();
        }
    }
}
impl CheckHistory for Transaction {
    fn check_history(&mut self) {
        if !self.from.transactions.contains(&self) {
            self.from.transactions.push(self.clone());
        }
        if !self.to.transactions.contains(&self) {
            self.to.transactions.push(self.clone());
        }
    }
}