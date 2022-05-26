use rust_decimal::Decimal;
use serde::Deserialize;

use crate::transaction_kind::TransactionKind;

/// Represents a single transaction, this type is meant to be constructed from
/// the CSV file, except for the `disputed` field.
#[derive(Deserialize)]
pub struct Transaction {
    #[serde(rename = "type")]
    pub kind: TransactionKind,
    #[serde(rename = "client")]
    pub client_id: u16,
    #[serde(rename = "tx")]
    pub id: u32,
    pub amount: Option<Decimal>,
    #[serde(skip)]
    pub disputed: bool,
}

impl Transaction {
    #[must_use]
    pub fn new(kind: TransactionKind, client_id: u16, id: u32, amount: Option<Decimal>) -> Self {
        Self { kind, client_id, id, amount, disputed: false }
    }
}
