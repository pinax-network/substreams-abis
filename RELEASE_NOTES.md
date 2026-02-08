# Release Notes - v0.10.0

## Summary

Restructured the entire repository to remove the `evm/` and `tvm/` namespace nesting. Modules are now organized by category: `erc20_tokens`, `erc721_tokens`, `dex`, `token`, `ens`, `polymarket`, `seaport`, and `dca_dot_fun`.

## Breaking Changes

All import paths have changed. Update your `use` statements:

| Before (v0.9.0) | After (v0.10.0) |
|------------------|-----------------|
| `substreams_abis::evm::tokens::*` | `substreams_abis::erc20_tokens::*` |
| `substreams_abis::evm::nfts::*` | `substreams_abis::erc721_tokens::*` |
| `substreams_abis::evm::uniswap::*` | `substreams_abis::dex::uniswap::*` |
| `substreams_abis::evm::curvefi::*` | `substreams_abis::dex::curvefi::*` |
| `substreams_abis::evm::balancer::*` | `substreams_abis::dex::balancer::*` |
| `substreams_abis::evm::bancor::*` | `substreams_abis::dex::bancor::*` |
| `substreams_abis::evm::cow::*` | `substreams_abis::dex::cow::*` |
| `substreams_abis::tvm::justswap::*` | `substreams_abis::dex::justswap::*` |
| `substreams_abis::tvm::sunpump::*` | `substreams_abis::dex::sunpump::*` |
| `substreams_abis::evm::token::*` | `substreams_abis::token::*` |
| `substreams_abis::evm::ens::*` | `substreams_abis::ens::*` |
| `substreams_abis::evm::polymarket::*` | `substreams_abis::polymarket::*` |
| `substreams_abis::evm::seaport::*` | `substreams_abis::seaport::*` |
| `substreams_abis::evm::dca_dot_fun::*` | `substreams_abis::dca_dot_fun::*` |

## New Directory Layout

```
abi/                              src/
├── erc20-tokens/                 ├── erc20_tokens/
│   ├── *.json, *.sol             │   ├── mod.rs
│   ├── usdc/                     │   ├── usdc/
│   └── usdt/                     │   └── usdt/
├── erc721-tokens/                ├── erc721_tokens/
│   └── *.json                    │   └── mod.rs
├── dex/                          ├── dex/
│   ├── uniswap/                  │   ├── mod.rs
│   ├── curvefi/                  │   ├── uniswap/
│   ├── balancer/                 │   ├── curvefi/
│   ├── bancor/                   │   ├── balancer/
│   ├── cow/                      │   ├── bancor/
│   ├── justswap/                 │   ├── cow/
│   └── sunpump/                  │   ├── justswap/
├── token/                        │   └── sunpump/
├── ens/                          ├── token/
├── polymarket/                   ├── ens/
├── seaport.json                  ├── polymarket/
└── dca_dot_fun.json              ├── seaport.rs
                                  ├── dca_dot_fun.rs
                                  └── lib.rs
```

## Usage Examples

```rust
use substreams_abis::erc20_tokens::dai;
use substreams_abis::dex::uniswap::v2::pair::events::Sync;
use substreams_abis::token::erc20::events::Transfer;
use substreams_abis::ens::v1::publicresolver::events::AddrChanged;
use substreams_abis::seaport::events::OrderFulfilled;
use substreams_abis::dex::sunpump::v1::launchpadproxy::events::TokenCreate;
```

## Build & Test

- `cargo build` passes
- All 11 tests pass with updated imports
- Previous: 0.9.0
- New: 0.10.0
