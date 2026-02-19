# 1inch

Leading DEX aggregator that sources liquidity across multiple protocols to find the best swap rates.

## Contracts

| Contract | Address | Chain | Explorer |
|----------|---------|-------|----------|
| V5 AggregationRouter | Various deployments | Ethereum | — |
| V6 AggregationRouter | Various deployments | Ethereum | — |

## Key Events

- **`OrderFilled`** — Emitted when a limit order is filled
- **`OrderFilledRFQ`** — (V5) Emitted when an RFQ order is filled
- **`OrderCancelled`** — (V6) Emitted when an order is cancelled
- **`OrderCanceled`** — (V5) Emitted when an order is cancelled
- **`NonceIncreased`** — (V5) Emitted when a user's nonce is incremented to cancel orders

## Overview

### V5 AggregationRouter
The 1inch V5 router supporting aggregated swaps, limit orders, and RFQ orders. Routes trades across multiple DEXes for optimal execution.

### V6 AggregationRouter
The latest 1inch router with improved gas efficiency, epoch-based order invalidation, and enhanced limit order capabilities.

## Links

- [Website](https://1inch.io)
- [Docs](https://docs.1inch.io)
- [GitHub](https://github.com/1inch)
