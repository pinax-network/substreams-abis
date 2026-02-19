# ParaSwap

DEX aggregator middleware that optimizes swap routes across multiple liquidity sources for best execution.

## Contracts

| Contract | Address | Chain | Explorer |
|----------|---------|-------|----------|
| Augustus V6.2 | `0x6A000F20005980200259B80c5102003040001068` | Ethereum | [Etherscan](https://etherscan.io/address/0x6A000F20005980200259B80c5102003040001068) |

## Key Events

- **`DiamondCut`** — Emitted when the contract's facets are updated (diamond proxy pattern)
- **`OwnershipTransferred`** — Emitted when contract ownership changes

## Overview

### Augustus V6.2
The main ParaSwap swap router using a diamond proxy pattern. Aggregates liquidity from Uniswap, Curve, Balancer, and other DEXes to find optimal swap paths with minimal slippage.

## Links

- [Website](https://paraswap.io)
- [Docs](https://developers.paraswap.network)
- [GitHub](https://github.com/paraswap)
