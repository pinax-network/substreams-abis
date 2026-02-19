# KyberSwap

DEX aggregator and liquidity protocol featuring concentrated liquidity pools (Elastic) and meta-aggregation routing.

## Contracts

| Contract | Address | Chain | Explorer |
|----------|---------|-------|----------|
| MetaAggregationRouterV2 | `0x6131B5fae19EA4f9D964eAc0408E4408b66337b5` | Ethereum | [Etherscan](https://etherscan.io/address/0x6131B5fae19EA4f9D964eAc0408E4408b66337b5) |
| Elastic Factory | Various deployments | Ethereum | — |

## Key Events

- **`Swapped`** — Emitted on every swap through the MetaAggregationRouter
- **`Exchange`** — Emitted for exchange events in the router
- **`PoolCreated`** — Emitted when a new KyberSwap Elastic pool is deployed
- **`Fee`** — Emitted when fees are collected
- **`ClientData`** — Emitted with client metadata for swap tracking

## Overview

### MetaAggregationRouterV2
The aggregation router that finds optimal swap routes across multiple DEXes and liquidity sources. Supports both simple swaps and complex multi-hop routes.

### Elastic Factory
Factory for deploying KyberSwap Elastic concentrated liquidity pools with configurable fee tiers and tick spacing.

## Links

- [Website](https://kyberswap.com)
- [Docs](https://docs.kyberswap.com)
- [GitHub](https://github.com/KyberNetwork)
