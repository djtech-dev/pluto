use crate::{defs::{Client, LocalClient, Spawn, Transaction, Wallet}, amounts::CalculateAmount};

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
        let mut amount: u128 = 0;
        for x in 0..(self.transactions.len()) {
            let t = &mut self.transactions[x];
            t.check_history();
            amount = self.calculate_partial(x);
        }
        assert_eq!(amount, self.calculate_amount());
    }
}
impl CheckHistory for Transaction {
    fn check_history(&mut self) {
        if !self.from.transactions.contains(self) {
            self.from.transactions.push(self.clone());
        }
        if !self.to.transactions.contains(self) {
            self.to.transactions.push(self.clone());
        }
    }
}
impl CheckHistory for Spawn {
    fn check_history(&mut self) {
        assert_eq!(
            self.spawned_by.id,
            self.checked_transaction.checked_by.as_ref().unwrap().id
        );
        self.checked_transaction.check_history();
    }
}


