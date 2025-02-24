# Substreams ABIs

This directory contains the ABIs for the various blockchains that Substreams supports.

## EVM

> Ethereum, Base, BSC, ArbOne, Polygon,...

- [x] `erc-20`
- [x] `erc-721`
- [x] `erc-1155`
- [x] `erc-2981`
- [x] `erc-4626`
- [ ] `erc-777`

## Solana

- [ ] `spl-token`
- [ ] `spl-memo`

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
use substreams_abi::erc20::events::Transfer;
...
for trx in block.transactions() {
  // Iterates over all logs in the transaction
  // Excluding those from calls that were not recorded to the chain's state.
  for (log, call_view) in trx.logs_with_calls() {
    // -- Transfer --
    let transfer = match Transfer::match_and_decode(log) {
        Some(transfer) => transfer,
        None => continue,
    };
  }
}
```
