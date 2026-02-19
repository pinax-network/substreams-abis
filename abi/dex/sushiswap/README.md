# SushiSwap

Community-driven DEX forked from Uniswap V2 with additional yield farming and governance features, deployed across multiple chains.

## Contracts

| Contract | Address | Chain | Explorer |
|----------|---------|-------|----------|
| V1 Factory | Various deployments | Multi-chain | — |
| V1 Pair | Various deployments | Multi-chain | — |

## Key Events

- **`Swap`** — Emitted on every token swap in a pair
- **`PairCreated`** — Emitted when a new trading pair is deployed via the Factory
- **`Mint`** — Emitted when liquidity is added to a pair
- **`Burn`** — Emitted when liquidity is removed from a pair
- **`Sync`** — Emitted after each swap or liquidity change with updated reserves

## Overview

### V1 Factory
Factory contract deploying new constant product AMM pairs. Follows the Uniswap V2 factory pattern.

### V1 Pair
Individual trading pair holding token reserves and executing swaps via the x*y=k invariant.

## Links

- [Website](https://sushi.com)
- [Docs](https://docs.sushi.com)
- [GitHub](https://github.com/sushiswap)
