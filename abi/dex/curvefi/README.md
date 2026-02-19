# Curve Finance

Stablecoin-focused AMM optimized for low-slippage swaps between pegged assets, also supporting volatile crypto pools.

## Contracts

| Contract | Address | Chain | Explorer |
|----------|---------|-------|----------|
| StableSwap (3pool) | `0xbEbc44782C7dB0a1A60Cb6fe97d0b483032FF1C7` | Ethereum | [Etherscan](https://etherscan.io/address/0xbEbc44782C7dB0a1A60Cb6fe97d0b483032FF1C7) |
| CryptoSwap (TriCrypto2) | `0xD51a44d3FaE010294C616388b506AcdA1bfAAE46` | Ethereum | [Etherscan](https://etherscan.io/address/0xD51a44d3FaE010294C616388b506AcdA1bfAAE46) |
| Factory | Various deployments | Ethereum | — |
| CryptoSwapFactory | Various deployments | Ethereum | — |

## Key Events

- **`TokenExchange`** — Emitted on every swap in a Curve pool
- **`AddLiquidity`** — Emitted when liquidity is added to a pool
- **`RemoveLiquidity`** — Emitted when liquidity is removed (balanced withdrawal)
- **`RemoveLiquidityOne`** — Emitted on single-sided liquidity removal
- **`PlainPoolDeployed`** — Emitted when a new plain pool is created via the Factory

## Overview

### StableSwap
Pool implementation for assets that trade near parity (e.g., USDC/USDT/DAI). Uses the StableSwap invariant for minimal slippage.

### CryptoSwap
Pool implementation for volatile asset pairs (e.g., ETH/BTC/USDT). Uses a modified invariant with internal price oracles.

### Factory
Permissionless factory for deploying new StableSwap pools and gauges.

### CryptoSwapFactory
Factory for deploying new CryptoSwap (volatile) pools.

## Links

- [Website](https://curve.fi)
- [Docs](https://resources.curve.fi)
- [GitHub](https://github.com/curvefi)
