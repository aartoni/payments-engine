# Payments Engine

## Requirements and assumptions

I completed every requirement, and made the following assumptions:

- the file must be well-formed to a certain extent, e.g. each deposit should have exactly 4 comma-separated values;
- as the assignment specifies that a dispute always causes a decrease in available funds, I suppose only deposits can be disputed, so I'm not keeping track of the withdrawals;
> the clients available funds should decrease by the amount disputed
- deposit and withdrawal transactions always have a non-empty amount (panics otherwise);
- the `client_id` field on a dispute, resolve or chargeback transaction always matches the one for the disputed transaction (panics otherwise).

Moreover, most of the project has been developed with ad TDD approach, it also ships with a very rich documentation that you can open issuing the following command

```bash
cargo doc --open
```

## Workflow

The computation is single-threaded: the file is parsed and processed line by line, the CSV output gets printed once the EOF is reached. The following pseudocode will help understand the workflow.

```
main:
    for transaction in file:
        engine.process(transaction)

    for account in engine.accounts:
        output(account)

```

The processing step handles transfers (deposit, withdraw) and claims (dispute, resolve, chargeback) separately. The following pseudocode will help understand the workflow.

```
process(tx):
    if transfer:
        find_or_insert_account(tx.client_id)
        perform_actual_transfer(tx)
        append_to_history_if_needed(tx)

    if claim:
        disputed = find_disputed(tx.id)
        return if missing(disputed)
        return if undisputed(disputed) and not disputing_now(tx)
        disputed.toggle_flag_if_needed()
        perform_actual_claim(tx)
```

## Program structure

The program revolves around the `PaymentsEngine` data structure, which keeps track of the accounts and the transaction history via two `HashMap`s.

## Complexity

Everything can be done in *O*(1) thanks to the `HashMap`s.

## Running

You can run via cargo:

    cargo run

or build and run via Docker:

    docker build -t orderbook . && docker run -it orderbook

## Testing

To run the test you can

    cargo test
