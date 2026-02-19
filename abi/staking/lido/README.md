# Lido

The largest liquid staking protocol on Ethereum, allowing users to stake ETH and receive stETH — a rebasing token that accrues staking rewards daily.

## Contracts

| Contract | Address | Chain | Explorer |
|----------|---------|-------|----------|
| stETH | `0xae7ab96520DE3A18E5e111B5EaAb095312D7fE84` | Ethereum | [Etherscan](https://etherscan.io/address/0xae7ab96520DE3A18E5e111B5EaAb095312D7fE84) |
| wstETH | Various deployments | Ethereum | — |
| WithdrawalQueue | Various deployments | Ethereum | — |

## Key Events

- **`Submitted`** — Emitted when a user stakes ETH and receives stETH
- **`TokenRebased`** — Emitted when stETH balances are rebased to reflect staking rewards
- **`TransferShares`** — Emitted on stETH transfers (share-based accounting)
- **`WithdrawalRequested`** — Emitted when a user requests to withdraw staked ETH
- **`WithdrawalClaimed`** — Emitted when a user claims their withdrawn ETH

## Overview

### stETH
The rebasing liquid staking token. Balances automatically update daily to reflect staking rewards. The core Lido contract also handles ETH deposits and validator management.

### wstETH
Wrapped stETH with a non-rebasing balance. The exchange rate to stETH increases over time, making it compatible with DeFi protocols that don't support rebasing tokens.

### WithdrawalQueue
NFT-based queue for stETH→ETH withdrawals. Users request a withdrawal and receive an NFT that becomes claimable once the protocol processes the unstaking.

## Links

- [Website](https://lido.fi)
- [Docs](https://docs.lido.fi)
- [GitHub](https://github.com/lidofinance)
