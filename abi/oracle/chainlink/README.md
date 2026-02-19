# Chainlink

Decentralized oracle network providing reliable price feeds and off-chain data to smart contracts.

## Contracts

| Contract | Address | Chain | Explorer |
|----------|---------|-------|----------|
| FeedRegistry | `0x47Fb2585D2C56Fe188D0E6ec628a38b74fCeeeDf` | Ethereum | [Etherscan](https://etherscan.io/address/0x47Fb2585D2C56Fe188D0E6ec628a38b74fCeeeDf) |
| OffchainAggregator | Various deployments | Ethereum | — |

## Key Events

- **`AnswerUpdated`** — Emitted when a price feed reports a new answer
- **`NewRound`** — Emitted when a new oracle round begins
- **`NewTransmission`** — Emitted when an off-chain report is transmitted on-chain
- **`FeedConfirmed`** — Emitted when a price feed is confirmed in the registry
- **`FeedProposed`** — Emitted when a new price feed is proposed

## Overview

### FeedRegistry
On-chain registry mapping asset pairs to their Chainlink price feed addresses. Provides a single entry point for querying any supported price feed.

### OffchainAggregator
Individual price feed aggregator using off-chain reporting (OCR). Multiple oracle nodes agree on a price off-chain and submit a single transaction, reducing gas costs.

## Links

- [Website](https://chain.link)
- [Docs](https://docs.chain.link)
- [GitHub](https://github.com/smartcontractkit)
