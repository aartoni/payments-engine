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
                // Find the account, insert if missing
                let account = self.accounts
                    .entry(tx.client_id)
                    .or_insert_with(|| Account::new(tx.client_id));

                // Perform the transaction
                if handle_transfer(&tx.kind, account, tx.amount.unwrap()) {
                    // Transaction succeded, add it to the history
                    self.history.insert(tx.id, tx);
                }
            },
            _ => {
                let disputed_tx = self.history.get_mut(&tx.id);

                // If the disputed tx doesn't exist ignore this tx
                if disputed_tx.is_none() {
                    return;
                }

                let disputed_tx = disputed_tx.unwrap();

                // Set/check disputation flag for the disputed tx
                if tx.kind == TransactionKind::Dispute {
                    disputed_tx.disputed = true;
                } else {
                    // If the disputed tx was never disputed ignore this tx
                    if !disputed_tx.disputed {
                        return;
                    }

                    disputed_tx.disputed = false;
                }

                let account = self.accounts.get_mut(&tx.client_id).unwrap();
                handle_claim(&tx.kind, account, disputed_tx.amount.unwrap());
            },
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

fn handle_claim(kind: &TransactionKind, client: &mut Account, amount: Decimal) {
    match kind {
        TransactionKind::Dispute => client.dispute(amount),
        TransactionKind::Resolve => client.resolve(amount),
        _ => panic!("Unsupported"),
    }
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
        let dispute_tx = Transaction::new(TransactionKind::Dispute, 1, 1, None);

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

    #[test]
    fn test_resolve() {
        // Create transactions
        let deposit_tx = Transaction::new(TransactionKind::Deposit, 1, 1, Some(dec!(1)));
        let dispute_tx = Transaction::new(TransactionKind::Dispute, 1, 1, None);
        let resolve_tx = Transaction::new(TransactionKind::Resolve, 1, 1, None);

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

        // Resolve on both sides
        engine.execute(resolve_tx);
        expected.resolve(dec!(1));
        assert_eq!(engine.accounts.get(&1).unwrap(), &expected);
    }

    #[test]
    fn test_resolve_without_dispute() {
        // Create transactions
        let deposit_tx = Transaction::new(TransactionKind::Deposit, 1, 1, Some(dec!(1)));
        let resolve_tx = Transaction::new(TransactionKind::Resolve, 1, 1, None);

        // Create test engine and account
        let mut engine = PaymentsEngine::new();
        let mut expected = Account::new(1);

        // Deposit on both sides
        engine.execute(deposit_tx);
        expected.deposit(dec!(1));
        assert_eq!(engine.accounts.get(&1).unwrap(), &expected);

        // Resolve on both sides
        engine.execute(resolve_tx);
        assert_eq!(engine.accounts.get(&1).unwrap(), &expected);
    }
}
