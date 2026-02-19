# Bancor

Decentralized AMM pioneer featuring single-sided liquidity provision and impermanent loss protection.

## Contracts

| Contract | Address | Chain | Explorer |
|----------|---------|-------|----------|
| BancorNetworkV3 | `0xeEF417e1D5CC832e619ae18D2F140De2999dD4fB` | Ethereum | [Etherscan](https://etherscan.io/address/0xeEF417e1D5CC832e619ae18D2F140De2999dD4fB) |
| CarbonController | `0xC537e898CD774e2dCBa3B14Ea6f34C93d5eA45e1` | Ethereum | [Etherscan](https://etherscan.io/address/0xC537e898CD774e2dCBa3B14Ea6f34C93d5eA45e1) |
| StandardPoolConverter | Various deployments | Ethereum | — |
| ConverterRegistry | Various deployments | Ethereum | — |

## Key Events

- **`Conversion`** — Emitted on every token swap through a Bancor converter
- **`LiquidityAdded`** — Emitted when liquidity is added to a pool
- **`LiquidityRemoved`** — Emitted when liquidity is removed from a pool
- **`TokenRateUpdate`** — Emitted when token conversion rates change
- **`NewConverter`** — Emitted when a new converter is created via the factory

## Overview

### BancorNetworkV3
The V3 network contract enabling single-sided liquidity deposits with impermanent loss protection. Handles routing and trade execution.

### CarbonController
Bancor Carbon's on-chain limit order / range order system allowing asymmetric liquidity strategies.

### StandardPoolConverter
Legacy converter handling token swaps between paired reserves using the Bancor formula.

### ConverterRegistry
Registry tracking all deployed converters and their associated tokens.

## Links

- [Website](https://bancor.network)
- [Docs](https://docs.bancor.network)
- [GitHub](https://github.com/bancorprotocol)
