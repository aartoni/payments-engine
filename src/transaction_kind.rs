use serde::Deserialize;

#[derive(Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TransactionKind {
    Deposit,
    Withdrawal,
    Dispute,
    Resolve,
    Chargeback,
}
