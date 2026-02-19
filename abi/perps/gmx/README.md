# GMX

Decentralized perpetual exchange on Arbitrum offering up to 100x leverage with low swap fees and minimal price impact.

## Contracts

| Contract | Address | Chain | Explorer |
|----------|---------|-------|----------|
| V2 EventEmitter | `0xC8ee91A54287DB53897056e12D9819156D3822Fb` | Arbitrum | [Arbiscan](https://arbiscan.io/address/0xC8ee91A54287DB53897056e12D9819156D3822Fb) |
| V2 Router | Various deployments | Arbitrum | — |

## Key Events

- **`EventLog`** — Generic event log for all GMX V2 protocol actions
- **`EventLog1`** — Indexed event log with one topic for efficient filtering
- **`EventLog2`** — Indexed event log with two topics for efficient filtering

> GMX V2 uses a generic event system where all protocol events (swaps, position changes, liquidations, deposits, withdrawals) are emitted through the EventEmitter with structured data payloads.

## Overview

### V2 EventEmitter
Central event hub for GMX V2. All protocol actions emit events through this contract, enabling unified indexing of swaps, position increases/decreases, liquidations, and more.

### V2 Router
Entry point for users to interact with GMX V2 markets. Handles creating orders for swaps, opening/closing leveraged positions, and managing deposits/withdrawals.

## Links

- [Website](https://gmx.io)
- [Docs](https://docs.gmx.io)
- [GitHub](https://github.com/gmx-io)
