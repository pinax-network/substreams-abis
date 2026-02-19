# Across Protocol

Optimistic cross-chain bridge powered by UMA's optimistic oracle, enabling fast token transfers between EVM chains.

## Contracts

| Contract | Address | Chain | Explorer |
|----------|---------|-------|----------|
| SpokePool | `0x5c7BCd6E7De5423a257D81B442095A1a6ced35C5` | Ethereum | [Etherscan](https://etherscan.io/address/0x5c7BCd6E7De5423a257D81B442095A1a6ced35C5) |

## Key Events

- **`V3FundsDeposited`** — Emitted when a user deposits tokens to bridge cross-chain
- **`FilledV3Relay`** — Emitted when a relayer fills a deposit on the destination chain
- **`RequestedV3SlowFill`** — Emitted when a slow fill is requested through the canonical bridge
- **`ExecutedRelayerRefundRoot`** — Emitted when relayer refunds are distributed
- **`RelayedRootBundle`** — Emitted when a new root bundle is relayed from the HubPool

## Overview

### SpokePool
The core contract deployed on each supported chain that handles deposits and fills. Users deposit tokens on the origin chain, and relayers fill those deposits on the destination chain for fast bridging.

## Links

- [Website](https://across.to)
- [Docs](https://docs.across.to)
- [GitHub](https://github.com/across-protocol)
