# PancakeSwap

Multi-chain DEX originally built on BNB Chain, offering V2 constant product and V3 concentrated liquidity pools.

## Contracts

| Contract | Address | Chain | Explorer |
|----------|---------|-------|----------|
| V2 Factory | Various deployments | Multi-chain | — |
| V2 Pair | Various deployments | Multi-chain | — |
| V3 Factory | Various deployments | Multi-chain | — |
| V3 Pool | Various deployments | Multi-chain | — |

## Key Events

- **`Swap`** — Emitted on every token swap in V2 and V3 pools
- **`PairCreated`** — (V2) Emitted when a new trading pair is deployed
- **`PoolCreated`** — (V3) Emitted when a new concentrated liquidity pool is deployed
- **`Mint`** — Emitted when liquidity is added to a pool
- **`Burn`** — Emitted when liquidity is removed from a pool

## Overview

### V2 Factory
Factory deploying constant product AMM pairs, forked from Uniswap V2.

### V2 Pair
Individual trading pair contract holding reserves and executing swaps via the x*y=k formula.

### V3 Factory
Factory for deploying concentrated liquidity pools with multiple fee tiers.

### V3 Pool
Concentrated liquidity pool where LPs can provide liquidity within specific price ranges for higher capital efficiency.

## Links

- [Website](https://pancakeswap.finance)
- [Docs](https://docs.pancakeswap.finance)
- [GitHub](https://github.com/pancakeswap)
