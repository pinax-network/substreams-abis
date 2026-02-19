# Coinbase Wrapped Staked ETH (cbETH)

Coinbase's liquid staking token representing staked ETH, enabling holders to earn staking rewards while maintaining liquidity.

## Contracts

| Contract | Address | Chain | Explorer |
|----------|---------|-------|----------|
| cbETH | `0xBe9895146f7AF43049ca1c1AE358B0541Ea49704` | Ethereum | [Etherscan](https://etherscan.io/address/0xBe9895146f7AF43049ca1c1AE358B0541Ea49704) |

## Key Events

- **`ExchangeRateUpdated`** — Emitted when the cbETH/ETH exchange rate is updated to reflect staking rewards
- **`Mint`** — Emitted when new cbETH tokens are minted
- **`Burn`** — Emitted when cbETH tokens are burned for redemption
- **`Transfer`** — Standard ERC-20 transfer event
- **`OracleUpdated`** — Emitted when the exchange rate oracle is changed

## Overview

### cbETH
The wrapped staked ETH token issued by Coinbase. Unlike rebasing tokens (stETH), cbETH uses an exchange rate model where the token's value appreciates relative to ETH as staking rewards accrue.

## Links

- [Website](https://coinbase.com/cbeth)
- [Docs](https://help.coinbase.com/en/coinbase/trading-and-funding/staking-rewards/cbeth)
