# 0x Protocol

Decentralized exchange infrastructure providing professional-grade liquidity aggregation and RFQ order settlement.

## Contracts

| Contract | Address | Chain | Explorer |
|----------|---------|-------|----------|
| ExchangeProxy | `0xDef1C0ded9bec7F1a1670819833240f027b25EfF` | Ethereum | [Etherscan](https://etherscan.io/address/0xDef1C0ded9bec7F1a1670819833240f027b25EfF) |

## Key Events

- **`LimitOrderFilled`** — Emitted when a limit order is filled (maker, taker, tokens, amounts, fees, pool)
- **`RfqOrderFilled`** — Emitted when an RFQ (Request for Quote) order is filled
- **`OtcOrderFilled`** — Emitted when an OTC (over-the-counter) order is filled
- **`TransformedERC20`** — Emitted on aggregated swaps through the TransformERC20 feature (taker, input/output token, amounts)
- **`LiquidityProviderSwap`** — Emitted when a swap is routed through a liquidity provider

## Overview

### ExchangeProxy
The main 0x entry point contract using a proxy/feature pattern. The proxy delegates to modular feature contracts that emit their own events. This ABI includes the swap-relevant events from the NativeOrdersFeature, TransformERC20Feature, and LiquidityProviderFeature.

### Event Categories
- **Order fills** (`LimitOrderFilled`, `RfqOrderFilled`, `OtcOrderFilled`): Direct peer-to-peer order settlement with maker/taker tokens and amounts
- **Aggregated swaps** (`TransformedERC20`): Swaps routed through the 0x aggregator using transformer pipelines
- **LP swaps** (`LiquidityProviderSwap`): Swaps routed directly to registered liquidity providers

## Links

- [Website](https://0x.org)
- [Docs](https://0x.org/docs)
- [GitHub](https://github.com/0xProject)
