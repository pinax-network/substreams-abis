# Fraxswap

Time-Weighted Average Market Maker (TWAMM) DEX by Frax Finance, enabling large orders to execute over time to minimize price impact.

## Contracts

| Contract | Address | Chain | Explorer |
|----------|---------|-------|----------|
| FraxswapRouter | `0xC14d550632db8592D1243Edc8B95b0Ad06703867` | Ethereum | [Etherscan](https://etherscan.io/address/0xC14d550632db8592D1243Edc8B95b0Ad06703867) |

## Key Events

The FraxswapRouter ABI does not define events directly — swap events are emitted by the underlying pair contracts (Uniswap V2 fork pattern).

## Overview

### FraxswapRouter
The swap router for Fraxswap pools. Supports standard instant swaps and TWAMM long-term orders that execute gradually over a specified time period.

## Links

- [Website](https://frax.finance)
- [Docs](https://docs.frax.finance/fraxswap)
- [GitHub](https://github.com/FraxFinance)
