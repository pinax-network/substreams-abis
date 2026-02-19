# Convex Finance

Yield optimizer built on top of Curve Finance, boosting CRV rewards for liquidity providers without requiring them to lock CRV.

## Contracts

| Contract | Address | Chain | Explorer |
|----------|---------|-------|----------|
| Booster | `0xF403C135812408BFbE8713b5A23a04b3D48AAE31` | Ethereum | [Etherscan](https://etherscan.io/address/0xF403C135812408BFbE8713b5A23a04b3D48AAE31) |
| BaseRewardPool | Various deployments | Ethereum | — |

## Key Events

- **`Deposited`** — Emitted when a user deposits Curve LP tokens into Convex
- **`Withdrawn`** — Emitted when a user withdraws Curve LP tokens from Convex

## Overview

### Booster
The main Convex contract that manages Curve LP token deposits. Handles staking Curve LP tokens, boosting CRV rewards using Convex's locked veCRV position, and distributing rewards to depositors.

### BaseRewardPool
Reward distribution contract for each Convex pool. Tracks staked balances and distributes boosted CRV, CVX, and any extra reward tokens to depositors.

## Links

- [Website](https://convexfinance.com)
- [Docs](https://docs.convexfinance.com)
- [GitHub](https://github.com/convex-eth)
