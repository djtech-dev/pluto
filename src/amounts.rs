use crate::defs::Wallet;

pub trait CalculateAmount {
    fn calculate_amount(&self) -> u128;
    fn calculate_partial(&self, last: usize) -> u128;
}
impl CalculateAmount for Wallet {
    fn calculate_amount(&self) -> u128 {
        let mut amount: u128 = 0;
        for t in &self.transactions {
            if *self == t.from {
                amount -= t.amount as u128;
            } else {
                amount += t.amount as u128;
            }
        }
        amount
    }

    fn calculate_partial(&self, last: usize) -> u128 {
        let mut amount: u128 = 0;
        for t in &self.transactions[..last] {
            if *self == t.from {
                amount -= t.amount as u128;
            } else {
                amount += t.amount as u128;
            }
        }
        amount
    }
}
