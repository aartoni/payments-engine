use serde::Deserialize;

/// Possible transaction types, used for the `kind` field in the `Transaction` type.
#[derive(Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TransactionKind {
    Deposit,
    Withdrawal,
    Dispute,
    Resolve,
    Chargeback,
}
