# 0x Protocol

Decentralized exchange infrastructure providing professional-grade liquidity aggregation and RFQ order settlement.

## Contracts

| Contract | Address | Chain | Explorer |
|----------|---------|-------|----------|
| ExchangeProxy | `0xDef1C0ded9bec7F1a1670819833240f027b25EfF` | Ethereum | [Etherscan](https://etherscan.io/address/0xDef1C0ded9bec7F1a1670819833240f027b25EfF) |

## Key Events

The ExchangeProxy ABI uses a modular architecture — swap events are emitted through internal features and transformers rather than top-level events.

## Overview

### ExchangeProxy
The main 0x entry point contract using a proxy/transformer pattern. Aggregates liquidity from on-chain DEXes and off-chain RFQ market makers to provide optimal swap execution.

## Links

- [Website](https://0x.org)
- [Docs](https://0x.org/docs)
- [GitHub](https://github.com/0xProject)
