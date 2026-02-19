# Stargate Finance

Cross-chain liquidity transport protocol built on LayerZero, enabling native asset transfers across chains.

## Contracts

| Contract | Address | Chain | Explorer |
|----------|---------|-------|----------|
| Router | `0x8731d54E9D02c286767d56ac03e8037C07e01e98` | Ethereum | [Etherscan](https://etherscan.io/address/0x8731d54E9D02c286767d56ac03e8037C07e01e98) |

## Key Events

- **`CachedSwapSaved`** — Emitted when a failed swap is cached for retry
- **`Revert`** — Emitted when a cross-chain swap is reverted
- **`RevertRedeemLocal`** — Emitted when a local redeem revert occurs
- **`RedeemLocalCallback`** — Emitted on callback after a local redemption

## Overview

### Router
The main entry point for cross-chain swaps and liquidity operations. Users interact with the Router to transfer native assets between chains using unified liquidity pools.

## Links

- [Website](https://stargate.finance)
- [Docs](https://stargateprotocol.gitbook.io/stargate)
- [GitHub](https://github.com/stargate-protocol)
