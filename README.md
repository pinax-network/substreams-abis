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

## How to contribute

- Compile the ABI from the source code.
- Add the ABI to the corresponding directory.
  - Only include `abi` section in the JSON file.
  - Name the file as the contract name.
  - Add the contract name to the `mod.rs` file.
- Add contract to `mod.rs` file.
- Build `cargo build`
- Run `cargo test` to ensure everything is working.
- Create a PR.
