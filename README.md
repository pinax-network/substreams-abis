# Substreams ABIs

This directory contains standard ABIs for the various blockchains that Substreams supports.

## EVM

> Ethereum, Base, BSC, ArbOne, Polygon,...

- [x] `ERC-20`
- [x] `ERC-721`
- [x] `ERC-1155`
- [x] `ERC-2981`
- [x] `ERC-4626`
- [x] `ERC-777`
- [x] `ERC-3643`

## EVM Tokens

- [x] SAI/DAI
- [x] USDC
- [x] USDT
- [x] WETH
- [x] WBTC
- [x] stETH

## EVM Contracts

- [x] `Uniswap V2`
  - [x] `Pair`
  - [x] `Factory`
- [x] `Uniswap V3`
  - [x] `Pool`
  - [x] `Factory`
- [x] `Uniswap V4`
  - [x] `PoolManager`
- [x] `ENS`
  - [x] `ENSRegistry`
  - [x] `EthRegistrarController`
  - [x] `PublicResolver`
  - [x] `ReverseRegistrar`
  - [x] `NameWrapper`

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
use substreams_abi::evm::token::erc20::events::Transfer;

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
