# Hashflow

Request-for-quote (RFQ) based DEX offering MEV-protected, zero-slippage trades with pricing from professional market makers.

## Contracts

| Contract | Address | Chain | Explorer |
|----------|---------|-------|----------|
| Router | `0x00000000F9490004C11Cef243f5400493c00Ad63` | Ethereum | [Etherscan](https://etherscan.io/address/0x00000000F9490004C11Cef243f5400493c00Ad63) |

## Key Events

- **`NewConduit`** — Emitted when a new trading conduit is registered

## Overview

### Router
The core Hashflow router that processes RFQ trades. Market makers provide signed quotes off-chain, and users submit them to the router for guaranteed-price execution without slippage.

## Links

- [Website](https://hashflow.com)
- [Docs](https://docs.hashflow.com)
- [GitHub](https://github.com/hashflownetwork)
