# CoW Protocol

Batch auction-based DEX that protects users from MEV by matching orders peer-to-peer (Coincidence of Wants) before routing to on-chain liquidity.

## Contracts

| Contract | Address | Chain | Explorer |
|----------|---------|-------|----------|
| GPv2Settlement | `0x9008D19f58AAbD9eD0D60971565AA8510560ab41` | Ethereum | [Etherscan](https://etherscan.io/address/0x9008D19f58AAbD9eD0D60971565AA8510560ab41) |

## Key Events

- **`Trade`** — Emitted for each individual trade within a batch settlement
- **`Settlement`** — Emitted when a batch of orders is settled
- **`OrderInvalidated`** — Emitted when an order is cancelled
- **`PreSignature`** — Emitted when an on-chain pre-signature is submitted for an order
- **`Interaction`** — Emitted for each external contract interaction during settlement

## Overview

### GPv2Settlement
The core settlement contract that executes batch auctions. Solvers compete to find the best execution for a batch of user orders, matching them peer-to-peer where possible and routing remaining volume through on-chain AMMs.

## Links

- [Website](https://cow.fi)
- [Docs](https://docs.cow.fi)
- [GitHub](https://github.com/cowprotocol)
