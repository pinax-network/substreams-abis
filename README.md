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

### Stablecoins & Wrapped Assets
- [x] SAI/DAI
- [x] USDC
- [x] USDT
- [x] WETH
- [x] WBTC
- [x] stETH

### ETH Mainnet Tokens
- [x] SHIB (Shiba Inu)
- [x] LINK (Chainlink)
- [x] UNI (Uniswap)
- [x] MATIC (Polygon)
- [x] ARB (Arbitrum)
- [x] LDO (Lido DAO)
- [x] AAVE (Aave)
- [x] MKR (Maker)
- [x] CRV (Curve DAO)
- [x] PEPE (Pepe)
- [x] APE (ApeCoin)
- [x] SNX (Synthetix)
- [x] COMP (Compound)
- [x] GRT (The Graph)
- [x] FET (Fetch.ai)

### Base Chain Tokens
- [x] AERO (Aerodrome)
- [x] BRETT (Brett)
- [x] DEGEN (Degen)
- [x] TOSHI (Toshi)
- [x] WELL (Moonwell)

## EVM Contracts

- [x] `Balancer`
- [x] `CurveFi`
- [x] `Bancor`
- [x] `Uniswap V1`
- [x] `Uniswap V2`
  - [x] `Pair`
  - [x] `Factory`
- [x] `Uniswap V3`
  - [x] `Pool`
  - [x] `Factory`
- [x] `Uniswap V4`
  - [x] `PoolManager`
- [x] `ENS V1`
  - [x] `ENSRegistry`
  - [x] `EthRegistrarController`
  - [x] `PublicResolver`
  - [x] `ReverseRegistrar`
  - [x] `NameWrapper`
- [x] `Seaport`
  - [x] `OrderFulfilled`
  - [x] `OrderCancelled`
  - [x] `OrderValidated`
  - [x] `OrdersMatched`

## NFTs Collections

- [x] Azuki
- [x] BoredApeYachtClub
- [x] Doodles
- [x] LilPudgys
- [x] MutantApeYachtClub
- [x] PudgyPenguins

## Tron EVM

- [x] SunSwap
- [x] JustSwap

## SunSwap V2

- Factory: `TKWJdrQkqHisa1X8HUdHEfREvTzw4pMAaY`
- Pair contract (for USDT/TRX pair): `TFGDbUyP8xez44C76fin3bn3Ss6jugoUwJ`
- ~~Router: `TKzxdSv2FZKQrEqkKVgp5DcwEXBEKMg2Ax`~~
- ~~Smart routing (aggregator): `TJ4NNy8xZEqsowCBhLvZ45LCqPdGjkET5j`~~

## JustSwap V1

- Exchange: `TPavNqt8xhoBp4NNBSdEx3FBP24NBfVRxU`

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
