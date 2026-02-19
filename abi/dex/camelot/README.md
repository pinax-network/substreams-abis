# Camelot

Arbitrum-native DEX built for the Arbitrum ecosystem with custom fee structures and launchpad features.

## Contracts

| Contract | Address | Chain | Explorer |
|----------|---------|-------|----------|
| CamelotRouter | `0xc873fEcbd354f5A56E00E710B90EF4201db2448d` | Arbitrum | [Arbiscan](https://arbiscan.io/address/0xc873fEcbd354f5A56E00E710B90EF4201db2448d) |
| CamelotFactory | `0x6EcCab422D763aC031210895C81787E87B43A652` | Arbitrum | [Arbiscan](https://arbiscan.io/address/0x6EcCab422D763aC031210895C81787E87B43A652) |

## Key Events

- **`PairCreated`** — Emitted when a new trading pair is deployed
- **`OwnerFeeShareUpdated`** — Emitted when the owner fee share changes
- **`ReferrerFeeShareUpdated`** — Emitted when referral fee configuration updates

## Overview

### CamelotRouter
The swap router for executing trades across Camelot liquidity pools. Supports multi-hop swaps with custom fee tiers.

### CamelotFactory
Factory contract for deploying new Camelot trading pairs with configurable fee structures and referral support.

## Links

- [Website](https://camelot.exchange)
- [Docs](https://docs.camelot.exchange)
- [GitHub](https://github.com/CamelotLabs)
