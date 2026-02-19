# Velodrome

The central trading and liquidity marketplace on Optimism, using the ve(3,3) model to align liquidity incentives with governance.

## Contracts

| Contract | Address | Chain | Explorer |
|----------|---------|-------|----------|
| Router | `0xa062aE8A9c5e11aaA026fc2670B0D65cCc8B2858` | Optimism | [Optimistic Etherscan](https://optimistic.etherscan.io/address/0xa062aE8A9c5e11aaA026fc2670B0D65cCc8B2858) |
| PoolFactory | Various deployments | Optimism | — |

## Key Events

- **`PoolCreated`** — Emitted when a new liquidity pool is deployed
- **`SetCustomFee`** — Emitted when a custom fee is configured for a pool
- **`SetFeeManager`** — Emitted when the fee manager is updated
- **`SetPauseState`** — Emitted when pool creation is paused or unpaused

## Overview

### Router
The swap router for executing trades across Velodrome pools. Supports both stable and volatile pool types.

### PoolFactory
Factory deploying new liquidity pools with configurable fees managed by veVELO governance.

## Links

- [Website](https://velodrome.finance)
- [Docs](https://docs.velodrome.finance)
- [GitHub](https://github.com/velodrome-finance)
