# Substreams ABIs

This directory contains standard ABIs for the various blockchains that Substreams supports.

## EVM

> Ethereum, Base, BSC, ArbOne, Polygon,...

- [x] `erc-20`
- [x] `erc-721`
- [x] `erc-1155`
- [x] `erc-2981`
- [x] `erc-4626`
- [ ] `erc-777`

## Solana

> Solana, Eclipse, Solana Devnet

- [ ] `spl-token`
- [ ] `spl-memo`

## Antelope

> EOS, WAX, Telos, Ultra,...

- [ ] `eosio.token`
- [ ] `eosio.system`

## How to contribute?

- Compile the ABI from the source code.
- Add the ABI to the corresponding directory.
  - Only include `abi` section in the JSON file.
  - Name the file as the contract name (ex: `evm/token/ERC20.json`).
- Build `cargo build`
- Update/add new contract to `mod.rs` file(s).
- Run `cargo test` to ensure everything is working.
- Create a PR.

## Quickstart

```rust
...
use substreams_abi::erc20::events::Transfer;

// Iterates over successful transactions
for trx in block.transactions() {
  // Iterates over all logs in the transaction, excluding those from calls that were not recorded to the chain's state.
  // The logs are sorted by their ordinal and returned as pairs of (log, call) where call is the call that produced the log.
  for (log, call_view) in trx.logs_with_calls() {
    // -- Transfer --
    let transfer = match Transfer::decode(&log) {
        Some(transfer) => transfer,
        None => continue,
    };
    // transfer.from => 6D1D1ebe7dA598194293784252659e862d55b52c
    // transfer.to => c7bBeC68d12a0d1830360F8Ec58fA599bA1b0e9b
    // transfer.value => 3400000000
  }
}
```
