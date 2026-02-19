# Polymarket

The largest decentralized prediction market, enabling trading on the outcomes of real-world events using conditional tokens on Polygon.

## Contracts

| Contract | Address | Chain | Explorer |
|----------|---------|-------|----------|
| CTFExchange | Various deployments | Polygon | — |
| NegRiskCTFExchange | Various deployments | Polygon | — |
| ConditionalTokens | Various deployments | Polygon | — |
| NegRiskAdapter | Various deployments | Polygon | — |
| FixedProductMarketMaker | Various deployments | Polygon | — |
| UmaCtfAdapter | Various deployments | Polygon | — |

## Key Events

- **`OrderFilled`** — Emitted when a trade order is filled on the exchange
- **`OrdersMatched`** — Emitted when buy and sell orders are matched
- **`ConditionPreparation`** — Emitted when a new prediction market condition is created
- **`ConditionResolution`** — Emitted when a market outcome is resolved
- **`PositionSplit`** — Emitted when collateral is split into conditional outcome tokens

## Overview

### CTFExchange
The central limit order book exchange for trading conditional tokens (outcome shares). Supports limit orders with off-chain matching.

### NegRiskCTFExchange
Exchange variant for negative risk markets where outcomes are mutually exclusive (e.g., multi-outcome elections).

### ConditionalTokens
The Gnosis Conditional Token Framework — ERC-1155 tokens representing positions in prediction markets. Handles splitting, merging, and redeeming outcome tokens.

### NegRiskAdapter
Adapter enabling negative risk markets where buying "No" on one outcome is equivalent to buying "Yes" on all other outcomes.

### FixedProductMarketMaker
AMM-based market maker for prediction markets using a fixed product (LMSR) pricing model.

### UmaCtfAdapter
Oracle adapter using UMA's optimistic oracle to resolve market outcomes.

## Links

- [Website](https://polymarket.com)
- [Docs](https://docs.polymarket.com)
- [GitHub](https://github.com/Polymarket)
