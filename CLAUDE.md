# CLAUDE.md - substreams-abis

## What This Repo Is
A Rust library of auto-generated ABI bindings for EVM smart contracts, used as a dependency in Substreams projects. Provides typed Rust structs for decoding events and function calls from blockchain data.

## Build & Test
```bash
cargo build    # Generates Rust from ABI JSONs via build.rs, then compiles
cargo test     # Run all tests
```
- Target: `wasm32-unknown-unknown` (Substreams WASM)
- Rust toolchain: 1.85

## Architecture
```
abi/erc20-tokens/{TOKEN}.json  →  build.rs  →  src/erc20_tokens/{token}.rs
abi/dex/uniswap/v2/Pair.json  →  build.rs  →  src/dex/uniswap/v2/pair.rs
```
- `build.rs` walks `abi/` recursively, generates `.rs` from every `.json` using `substreams_ethereum::Abigen`
- Hyphens in ABI directory names are converted to underscores in the generated Rust paths
- Module declarations in `mod.rs` files are **manual** - must be updated when adding tokens
- Multi-version tokens (USDC, USDT) use subdirectories with `#[path = "..."]` attributes

## Adding New Tokens
```bash
# Option 1: Use scripts
./scripts/fetch-token.sh TOKEN_NAME 0xContractAddress eth|base
./scripts/update-modules.sh

# Option 2: Manual
# 1. Save ABI JSON to abi/erc20-tokens/TOKEN.json
# 2. cargo build
# 3. Add `pub mod token;` to src/erc20_tokens/mod.rs
# 4. cargo build && cargo test
```

## Naming Conventions
- ABI files: ALL CAPS (`SHIB.json`, `LINK.json`)
- Solidity source: ALL CAPS (`SHIB.sol`) - reference only, not used in build
- Rust modules: lowercase (`shib.rs`, `link.rs`)
- Module declarations: `pub mod shib;` (lowercase, alphabetical)
- ABI directories use hyphens (`erc20-tokens/`), Rust modules use underscores (`erc20_tokens/`)

## Key Dependencies
- `substreams-ethereum` - ABI code generation and Ethereum types
- `ethabi` - ABI encoding/decoding
- `substreams` - Core Substreams SDK

## Scripts
- `scripts/fetch-token.sh` - Fetch single token ABI+source from Etherscan/Basescan
- `scripts/fetch-all-tokens.sh` - Batch fetch from built-in registry of 44 tokens
- `scripts/update-modules.sh` - Auto-generate mod.rs + verify build
- `scripts/keccak.js` - Keccak256 event signature finder utility
