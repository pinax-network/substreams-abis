# JustSwap

The first DEX on the Tron blockchain, based on the Uniswap V1 constant product model.

## Contracts

| Contract | Address | Chain | Explorer |
|----------|---------|-------|----------|
| Factory | Various deployments | Tron | — |
| Exchange | Various deployments | Tron | — |

## Key Events

- **`TokenPurchase`** — Emitted when TRX is swapped for tokens
- **`TrxPurchase`** — Emitted when tokens are swapped for TRX
- **`AddLiquidity`** — Emitted when liquidity is added to a pool
- **`RemoveLiquidity`** — Emitted when liquidity is removed from a pool
- **`NewExchange`** — Emitted when a new exchange contract is created for a token

## Overview

### Factory
Factory contract that deploys new Exchange contracts for each token, following the Uniswap V1 pattern.

### Exchange
Individual exchange contract for each token/TRX pair. Handles swaps, liquidity provision, and pricing via the constant product formula.

## Links

- [Website](https://justswap.org)
