use rust_decimal::Decimal;
use serde::Deserialize;

use crate::transaction_kind::TransactionKind;

#[derive(Deserialize)]
pub struct Transaction {
    #[serde(rename = "type")]
    pub kind: TransactionKind,
    #[serde(rename = "client")]
    pub client_id: u16,
    #[serde(rename = "tx")]
    pub id: u32,
    pub amount: Decimal,
}
