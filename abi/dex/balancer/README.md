# Balancer

Generalized AMM protocol supporting weighted pools, stable pools, and custom pool types with a single vault architecture.

## Contracts

| Contract | Address | Chain | Explorer |
|----------|---------|-------|----------|
| V2 Vault | `0xBA12222222228d8Ba445958a75a0704d566BF2C8` | Ethereum | [Etherscan](https://etherscan.io/address/0xBA12222222228d8Ba445958a75a0704d566BF2C8) |
| V2 WeightedPool | Various deployments | Ethereum | — |
| V3 Vault | Various deployments | Ethereum | — |
| V3 StablePool | Various deployments | Ethereum | — |

## Key Events

- **`Swap`** — Emitted on every token swap through the Vault
- **`PoolBalanceChanged`** — Emitted when liquidity is added or removed from a pool
- **`PoolRegistered`** — Emitted when a new pool is registered with the Vault
- **`FlashLoan`** — Emitted when a flash loan is executed
- **`LiquidityAdded`** — (V3) Emitted when liquidity is added to a pool

## Overview

### V2 Vault
The singleton vault that holds all pool tokens and manages swaps. All Balancer V2 pools share this single vault for capital efficiency.

### V2 WeightedPool
Pool type supporting arbitrary token weights (e.g., 80/20 pools). Extends beyond the traditional 50/50 AMM model.

### V3 Vault
Next-generation vault with improved gas efficiency, native support for yield-bearing tokens, and transient accounting.

### V3 StablePool
Pool optimized for tokens that trade near parity (stablecoins, LSTs), using amplified invariant curves.

## Links

- [Website](https://balancer.fi)
- [Docs](https://docs.balancer.fi)
- [GitHub](https://github.com/balancer)
