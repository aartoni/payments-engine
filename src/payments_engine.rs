use std::collections::HashMap;

use crate::{account::Account, transaction::Transaction, transaction_kind::TransactionKind};

pub struct PaymentsEngine {
    pub accounts: HashMap<u16, Account>
}

impl PaymentsEngine {
    #[must_use]
    pub fn new() -> Self {
        Self { accounts: HashMap::new() }
    }

    pub fn execute(&mut self, tx: Transaction) {
        match tx.kind {
            TransactionKind::Deposit => {
                let account = self.accounts
                    .entry(tx.client_id)
                    .or_insert_with(|| Account::new(tx.client_id));

                account.deposit(tx.amount);
            },
            _ => panic!("Unsupported")
        }
    }
}

#[cfg(test)]
mod tests {
    use rust_decimal_macros::dec;

    use super::*;

    #[test]
    fn test_deposit() {
        // Create transaction
        let tx = Transaction::new(TransactionKind::Deposit, 1, 1, dec!(1));

        // Create test engine and account
        let mut engine = PaymentsEngine::new();
        let mut expected = Account::new(1);

        // Deposit on both sides
        engine.execute(tx);
        expected.deposit(dec!(1));
        assert_eq!(engine.accounts.get(&1).unwrap(), &expected);
    }
}
