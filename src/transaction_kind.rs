use serde::Deserialize;

#[derive(Deserialize)]
pub enum TransactionKind {
    Deposit,
    Withdraw,
    Dispute,
    Resolve,
    Chargeback,
}
