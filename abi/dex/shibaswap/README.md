# ShibaSwap

Uniswap V2 fork DEX built for the Shiba Inu ecosystem.

## Contracts

| Contract | Address | Chain | Explorer |
|----------|---------|-------|----------|
| UniswapV2Router02 | `0x03f7724180AA6b939894B5Ca4314783B0b36b329` | Ethereum | [Etherscan](https://etherscan.io/address/0x03f7724180AA6b939894B5Ca4314783B0b36b329) |

## Key Events

The Router ABI does not define events directly — swap events (`Swap`, `Mint`, `Burn`, `Sync`) are emitted by the underlying pair contracts (Uniswap V2 pattern).

## Overview

### UniswapV2Router02
The swap router handling token swaps, liquidity additions, and removals across ShibaSwap pairs. Standard Uniswap V2 Router interface.

## Links

- [Website](https://shibaswap.com)
