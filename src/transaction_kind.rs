use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransactionKind {
    Deposit,
    Withdraw,
    Dispute,
    Resolve,
    Chargeback,
}
