use std::{error::Error, fs::File, env, io};

use csv::ReaderBuilder;
use payments::{transaction::Transaction, payments_engine::PaymentsEngine};

fn main() -> Result<(), Box<dyn Error>> {
    // Get the CSV reader
    let file_path = get_first_arg()?;
    let file = File::open(&file_path)?;
    let mut reader = ReaderBuilder::new()
        .trim(csv::Trim::All)
        .has_headers(true)
        .comment(Some(b'#'))
        .from_reader(&file);

    // Create a payments engine
    let mut engine = PaymentsEngine::new();

    // Parse each line and perform the transaction
    for result in reader.deserialize() {
        let transaction: Transaction = result?;
        engine.execute(transaction);
    }

    // Get the CSV writer
    let mut writer = csv::Writer::from_writer(io::stdout());

    // Print each customer's account data
    for account in engine.accounts.values() {
        writer.serialize(account)?;
    }

    // Flush CSV buffer to stdout
    writer.flush()?;
    Ok(())
}

fn get_first_arg() -> Result<String, Box<dyn Error>> {
    env::args().nth(1)
        .ok_or_else(|| From::from("No argument provided"))
}
