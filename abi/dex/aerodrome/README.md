# Aerodrome Finance

The central liquidity hub on Base, a ve(3,3) DEX that combines voting-escrowed tokenomics with concentrated liquidity.

## Contracts

| Contract | Address | Chain | Explorer |
|----------|---------|-------|----------|
| Router | `0xcF77a3Ba9A5CA399B7c97c74d54e5b1Beb874E43` | Base | [BaseScan](https://basescan.org/address/0xcF77a3Ba9A5CA399B7c97c74d54e5b1Beb874E43) |
| PoolFactory | `0x420DD381b31aEf6683db6B902084cB0FFECe40Da` | Base | [BaseScan](https://basescan.org/address/0x420DD381b31aEf6683db6B902084cB0FFECe40Da) |

## Key Events

- **`PoolCreated`** — Emitted when a new liquidity pool is deployed
- **`SetCustomFee`** — Emitted when a custom fee is set for a pool
- **`SetFeeManager`** — Emitted when the fee manager is updated

## Overview

### Router
The main swap router for executing token swaps across Aerodrome pools. Supports both stable and volatile pool types.

### PoolFactory
Factory contract for deploying new liquidity pools. Manages pool creation and fee configuration.

## Links

- [Website](https://aerodrome.finance)
- [Docs](https://aerodrome.finance/docs)
- [GitHub](https://github.com/aerodrome-finance)
