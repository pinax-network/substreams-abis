# DODO

Proactive Market Maker (PMM) DEX that uses oracle-guided pricing to provide better liquidity and reduce impermanent loss.

## Contracts

| Contract | Address | Chain | Explorer |
|----------|---------|-------|----------|
| V2 Pool (DVM) | `0x2BBD66fC4898242BDBD2583BBe1d76E8b8f71445` | Ethereum | [Etherscan](https://etherscan.io/address/0x2BBD66fC4898242BDBD2583BBe1d76E8b8f71445) |
| V2 RouteProxy | Various deployments | Ethereum | — |

## Key Events

### V2 Pool
- **`DODOSwap`** — Emitted on each swap within a DODO V2 pool (fromToken, toToken, fromAmount, toAmount, trader, receiver)
- **`DODOFlashLoan`** — Emitted for flash loan operations
- **`BuyShares`** / **`SellShares`** — Liquidity provision and withdrawal
- **`Burn`** / **`Mint`** — LP token burn and mint events

### V2 RouteProxy
- **`OrderHistory`** — Emitted for each swap routed through the DODO aggregator
- **`OwnershipTransferred`** — Emitted when contract ownership changes

## Overview

### V2 Pool
The DODO V2 Pool contract (DVM — DODO Vending Machine) implements the PMM algorithm for individual trading pairs. Each pool holds two tokens and provides swap, flash loan, and liquidity management functionality. The `DODOSwap` event is the primary event for tracking trades.

### V2 RouteProxy
The DODO V2 routing proxy that aggregates liquidity across DODO pools and external sources to find optimal swap paths for users.

## Links

- [Website](https://dodoex.io)
- [Docs](https://docs.dodoex.io)
- [GitHub](https://github.com/DODOEX)
