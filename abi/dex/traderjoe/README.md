# Trader Joe

Leading DEX on Avalanche featuring the Liquidity Book (LB) model with bin-based concentrated liquidity.

## Contracts

| Contract | Address | Chain | Explorer |
|----------|---------|-------|----------|
| LBRouter | `0xb4315e873dBcf96Ffd0acd8EA43f689D8c20fB30` | Avalanche | [Snowtrace](https://snowtrace.io/address/0xb4315e873dBcf96Ffd0acd8EA43f689D8c20fB30) |
| LBFactory | Various deployments | Avalanche | — |

## Key Events

- **`LBPairCreated`** — Emitted when a new Liquidity Book pair is deployed
- **`FeeRecipientSet`** — Emitted when the fee recipient is updated
- **`FlashLoanFeeSet`** — Emitted when the flash loan fee is configured
- **`PresetSet`** — Emitted when a new bin step preset is configured
- **`LBPairIgnoredStateChanged`** — Emitted when a pair's ignored state changes for routing

## Overview

### LBRouter
The swap router for Liquidity Book pools. Routes trades through bin-based liquidity for zero-slippage swaps within individual bins.

### LBFactory
Factory deploying new Liquidity Book pairs with configurable bin steps and fee parameters.

## Links

- [Website](https://traderjoexyz.com)
- [Docs](https://docs.traderjoexyz.com)
- [GitHub](https://github.com/traderjoe-xyz)
