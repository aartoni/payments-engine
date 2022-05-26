use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::Serialize;

#[derive(Debug, PartialEq, Serialize)]
pub struct Account {
    pub id: u16,
    pub available: Decimal,
    pub held: Decimal,
    pub total: Decimal,
    pub locked: bool,
}

impl Account {
    #[must_use]
    pub const fn new(id: u16) -> Self {
        Self {
            id,
            available: dec!(0),
            held: dec!(0),
            total: dec!(0),
            locked: false,
        }
    }

    pub fn deposit(&mut self, amount: Decimal) {
        self.available += amount;
        self.total += amount;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deposit() {
        let mut client = Account::new(1);

        // Deposit an integer amount
        client.deposit(dec!(1));
        assert_eq!(client.available, dec!(1));
        assert_eq!(client.total, dec!(1));

        // Deposit a decimal amount
        client.deposit(dec!(0.0001));
        assert_eq!(client.available, dec!(1.0001));
        assert_eq!(client.total, dec!(1.0001));
    }
}
