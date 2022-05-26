use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::Serialize;

/// A client account stating available, held and total funds, along with its
/// locked/unlocked state flag and its identifier.
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

    /// Deposit funds on the client account by increasing the available and
    /// total amounts.
    ///
    /// # Example
    /// ```
    /// use payments::account::Account;
    /// use rust_decimal_macros::dec;
    ///
    /// let mut account = Account::new(1);
    /// account.deposit(dec!(1));
    ///
    /// assert_eq!(account.available, dec!(1));
    /// assert_eq!(account.total, dec!(1));
    /// ```
    pub fn deposit(&mut self, amount: Decimal) {
        self.available += amount;
        self.total += amount;
    }

    /// Withdraw funds on the client account by decreasing the available and
    /// total amounts. The method has no effect if funds are insufficients and
    /// returns true in case of success.
    ///
    /// # Example
    /// ```
    /// use payments::account::Account;
    /// use rust_decimal_macros::dec;
    ///
    /// let mut account = Account::new(1);
    /// account.deposit(dec!(1));
    /// account.withdraw(dec!(1));
    ///
    /// assert_eq!(account.available, dec!(0));
    /// assert_eq!(account.total, dec!(0));
    /// ```
    pub fn withdraw(&mut self, amount: Decimal) -> bool {
        if amount > self.available {
            return false;
        }

        self.available -= amount;
        self.total -= amount;
        true
    }

    /// Dispute a transaction by witholding funds.
    ///
    /// # Example
    /// ```
    /// use payments::account::Account;
    /// use rust_decimal_macros::dec;
    ///
    /// let mut account = Account::new(1);
    /// account.deposit(dec!(1));
    /// account.dispute(dec!(1));
    ///
    /// assert_eq!(account.available, dec!(0));
    /// assert_eq!(account.held, dec!(1));
    /// assert_eq!(account.total, dec!(1));
    /// ```
    pub fn dispute(&mut self, amount: Decimal) {
        if amount > self.available {
            return;
        }

        self.available -= amount;
        self.held += amount;
    }

    /// Resolve a dispute by releasing funds.
    ///
    /// # Example
    /// ```
    /// use payments::account::Account;
    /// use rust_decimal_macros::dec;
    ///
    /// let mut account = Account::new(1);
    /// account.deposit(dec!(1));
    /// account.dispute(dec!(1));
    /// account.resolve(dec!(1));
    ///
    /// assert_eq!(account.available, dec!(1));
    /// assert_eq!(account.held, dec!(0));
    /// assert_eq!(account.total, dec!(1));
    /// ```
    pub fn resolve(&mut self, amount: Decimal) {
        if amount > self.held {
            return;
        }

        self.held -= amount;
        self.available += amount;
    }

    /// Resolve a dispute by charging funds back.
    ///
    /// # Example
    /// ```
    /// use payments::account::Account;
    /// use rust_decimal_macros::dec;
    ///
    /// let mut account = Account::new(1);
    /// account.deposit(dec!(1));
    /// account.dispute(dec!(1));
    /// account.chargeback(dec!(1));
    ///
    /// assert_eq!(account.available, dec!(0));
    /// assert_eq!(account.held, dec!(0));
    /// assert_eq!(account.total, dec!(0));
    /// ```
    pub fn chargeback(&mut self, amount: Decimal) {
        if amount > self.held {
            return;
        }

        self.held -= amount;
        self.total -= amount;
        self.locked = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deposit() {
        let mut account = Account::new(1);

        // Deposit an integer amount
        account.deposit(dec!(1));
        assert_eq!(account.available, dec!(1));
        assert_eq!(account.total, dec!(1));

        // Deposit a decimal amount
        account.deposit(dec!(0.0001));
        assert_eq!(account.available, dec!(1.0001));
        assert_eq!(account.total, dec!(1.0001));
    }

    #[test]
    fn test_withdraw() {
        let mut account = Account::new(1);
        account.deposit(dec!(1));

        // Try to withdraw an invalid amount
        account.withdraw(dec!(2));
        assert_eq!(account.available, dec!(1));
        assert_eq!(account.total, dec!(1));

        // Withdraw a decimal amount
        account.withdraw(dec!(0.5));
        assert_eq!(account.available, dec!(0.5));
        assert_eq!(account.total, dec!(0.5));
    }

    #[test]
    fn test_dispute() {
        let mut account = Account::new(1);
        account.deposit(dec!(1));

        // Try to dispute an invalid amount
        account.dispute(dec!(2));
        assert_eq!(account.available, dec!(1));
        assert_eq!(account.held, dec!(0));
        assert_eq!(account.total, dec!(1));

        // Dispute a valid amount
        account.dispute(dec!(0.5));
        assert_eq!(account.available, dec!(0.5));
        assert_eq!(account.held, dec!(0.5));
        assert_eq!(account.total, dec!(1));
    }

    #[test]
    fn test_resolve() {
        let mut account = Account::new(1);
        account.deposit(dec!(10));

        // Dispute a valid amount
        account.dispute(dec!(5));
        assert_eq!(account.available, dec!(5));
        assert_eq!(account.held, dec!(5));
        assert_eq!(account.total, dec!(10));

        // Resolve a valid amount
        account.resolve(dec!(5));
        assert_eq!(account.available, dec!(10));
        assert_eq!(account.held, dec!(0));
        assert_eq!(account.total, dec!(10));

        // Try to resolve an invalid amount
        account.resolve(dec!(10));
        assert_eq!(account.available, dec!(10));
        assert_eq!(account.held, dec!(0));
        assert_eq!(account.total, dec!(10));
    }

    #[test]
    fn test_chargeback() {
        let mut account = Account::new(1);
        account.deposit(dec!(10));

        // Dispute a valid amount
        account.dispute(dec!(5));
        assert_eq!(account.available, dec!(5));
        assert_eq!(account.held, dec!(5));
        assert_eq!(account.total, dec!(10));

        // Charge a valid amount back
        account.chargeback(dec!(5));
        assert_eq!(account.available, dec!(5));
        assert_eq!(account.held, dec!(0));
        assert_eq!(account.total, dec!(5));

        // Try to charge an invalid amount back
        account.chargeback(dec!(5));
        assert_eq!(account.available, dec!(5));
        assert_eq!(account.held, dec!(0));
        assert_eq!(account.total, dec!(5));
    }
}
