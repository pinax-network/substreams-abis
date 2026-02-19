# CLAUDE.md - substreams-abis

## What This Repo Is
A Rust library of auto-generated ABI bindings for EVM smart contracts, used as a dependency in Substreams projects. Provides typed Rust structs for decoding events and function calls from blockchain data.

## Agent Skills
Detailed instructions for working on this repo are in **`.agents/skills/`**:
- [add-abi.md](.agents/skills/add-abi.md) — How to add new protocol or token ABIs
- [abi-naming.md](.agents/skills/abi-naming.md) — Naming conventions and special cases
- [build-and-test.md](.agents/skills/build-and-test.md) — Build system and testing
- [pr-workflow.md](.agents/skills/pr-workflow.md) — PR process and releases

## Build & Test
```bash
cargo build    # Generates Rust from ABI JSONs via build.rs, then compiles
cargo test     # Run all tests
cargo check --target wasm32-unknown-unknown  # Verify WASM compatibility
```
- Target: `wasm32-unknown-unknown` (Substreams WASM)
- Rust toolchain: 1.85

## Architecture
```
abi/dex/uniswap/v2/Pair.json  →  build.rs  →  src/dex/uniswap/v2/pair.rs
abi/tokens/erc20/LINK.json    →  build.rs  →  src/tokens/erc20/link.rs
```
- `build.rs` walks `abi/` recursively, generates `.rs` from every `.json` using `substreams_ethereum::Abigen`
- Hyphens in ABI directory names are converted to underscores in the generated Rust paths
- File stems are lowercased; directory names are NOT lowercased
- Module declarations in `mod.rs` files are **manual** — must be updated when adding ABIs
- Each protocol directory should have a `README.md` with contract addresses, key events, and docs links

## Category Structure
```
abi/
├── bridge/        — Cross-chain bridges (Across, Stargate, LayerZero)
├── dex/           — DEXes and swap protocols (Uniswap, Curve, Balancer, ...)
├── lending/       — Lending protocols (Aave, Compound, Morpho)
├── naming/        — Name services (ENS)
├── oracle/        — Price oracles (Chainlink)
├── perps/         — Perpetual exchanges (GMX)
├── prediction/    — Prediction markets (Polymarket)
├── restaking/     — Restaking (EigenLayer)
├── stablecoin/    — Stablecoin systems (Maker)
├── staking/       — Liquid staking (Lido, Rocket Pool, Coinbase)
├── standard/      — Token standards (ERC-20, ERC-721, WETH9, etc.)
├── tokens/        — Specific token ABIs (80+ ERC-20, 7 ERC-721)
└── yield/         — Yield protocols (Convex)
```

## Key Dependencies
- `substreams-ethereum` — ABI code generation and Ethereum types
- `ethabi` — ABI encoding/decoding
- `substreams` — Core Substreams SDK
