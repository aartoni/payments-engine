use std::collections::HashMap;

use rust_decimal::Decimal;

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
            TransactionKind::Deposit | TransactionKind::Withdrawal => {
                let account = self.accounts
                    .entry(tx.client_id)
                    .or_insert_with(|| Account::new(tx.client_id));

                handle_transfer(tx.kind, account, tx.amount);
            },
            _ => panic!("Unsupported")
        }
    }
}

fn handle_transfer(kind: TransactionKind, account: &mut Account, amount: Decimal) {
    if kind == TransactionKind::Deposit {
        account.deposit(amount);
    } else {
        account.withdraw(amount);
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

    #[test]
    fn test_withdraw() {
        // Create transactions
        let deposit_tx = Transaction::new(TransactionKind::Deposit, 1, 1, dec!(1));
        let withdraw_tx = Transaction::new(TransactionKind::Withdrawal, 1, 1, dec!(1));

        // Create test engine and account
        let mut engine = PaymentsEngine::new();
        let mut expected = Account::new(1);

        // Deposit on both sides
        engine.execute(deposit_tx);
        expected.deposit(dec!(1));
        assert_eq!(engine.accounts.get(&1).unwrap(), &expected);

        // Withdraw on both sides
        engine.execute(withdraw_tx);
        expected.withdraw(dec!(1));
        assert_eq!(engine.accounts.get(&1).unwrap(), &expected);
    }
}
