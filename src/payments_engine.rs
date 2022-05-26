use std::collections::HashMap;

use rust_decimal::Decimal;

use crate::{account::Account, transaction::Transaction, transaction_kind::TransactionKind};

pub struct PaymentsEngine {
    pub accounts: HashMap<u16, Account>,
    history: HashMap<u32, Transaction>,
}

impl PaymentsEngine {
    #[must_use]
    pub fn new() -> Self {
        Self {
            accounts: HashMap::new(),
            history: HashMap::new(),
        }
    }

    pub fn execute(&mut self, tx: Transaction) {
        match tx.kind {
            TransactionKind::Deposit | TransactionKind::Withdrawal => {
                let account = self.accounts
                    .entry(tx.client_id)
                    .or_insert_with(|| Account::new(tx.client_id));

                if handle_transfer(&tx.kind, account, tx.amount.unwrap()) {
                    self.history.insert(tx.id, tx);
                }
            },
            TransactionKind::Dispute => {
                let disputed_tx = self.history.get_mut(&tx.id);

                // If the target tx doesn't exist ignore this tx
                if disputed_tx.is_none() {
                    return;
                }

                let disputed_tx = disputed_tx.unwrap();

                // Set disputation flag for the target tx
                disputed_tx.disputed = true;

                let account = self.accounts.get_mut(&tx.client_id).unwrap();
                account.dispute(disputed_tx.amount.unwrap());
            },
            _ => panic!("Unsupported")
        }
    }
}

fn handle_transfer(kind: &TransactionKind, account: &mut Account, amount: Decimal) -> bool {
    if *kind == TransactionKind::Deposit {
        account.deposit(amount);
        return true;
    }

    account.withdraw(amount)
}

#[cfg(test)]
mod tests {
    use rust_decimal_macros::dec;

    use super::*;

    #[test]
    fn test_deposit() {
        // Create transaction
        let tx = Transaction::new(TransactionKind::Deposit, 1, 1, Some(dec!(1)));

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
        let deposit_tx = Transaction::new(TransactionKind::Deposit, 1, 1, Some(dec!(1)));
        let withdraw_tx = Transaction::new(TransactionKind::Withdrawal, 1, 1, Some(dec!(1)));

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

    #[test]
    fn test_dispute() {
        // Create transactions
        let deposit_tx = Transaction::new(TransactionKind::Deposit, 1, 1, Some(dec!(1)));
        let dispute_tx = Transaction::new(TransactionKind::Dispute, 1, 1, Some(dec!(1)));

        // Create test engine and account
        let mut engine = PaymentsEngine::new();
        let mut expected = Account::new(1);

        // Deposit on both sides
        engine.execute(deposit_tx);
        expected.deposit(dec!(1));
        assert_eq!(engine.accounts.get(&1).unwrap(), &expected);

        // Dispute on both sides
        engine.execute(dispute_tx);
        expected.dispute(dec!(1));
        assert_eq!(engine.accounts.get(&1).unwrap(), &expected);
    }
}
