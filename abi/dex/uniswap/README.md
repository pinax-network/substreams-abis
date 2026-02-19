# Uniswap

The original and largest decentralized exchange, pioneering the automated market maker model across four major versions.

## Contracts

| Contract | Address | Chain | Explorer |
|----------|---------|-------|----------|
| V1 Factory | `0xc0a47dFe034B400B47bDaD5FecDa2621de6c4d95` | Ethereum | [Etherscan](https://etherscan.io/address/0xc0a47dFe034B400B47bDaD5FecDa2621de6c4d95) |
| V2 Factory | `0x5C69bEe701ef814a2B6a3EDD4B1652CB9cc5aA6f` | Ethereum | [Etherscan](https://etherscan.io/address/0x5C69bEe701ef814a2B6a3EDD4B1652CB9cc5aA6f) |
| V3 Factory | `0x1F98431c8aD98523631AE4a59f267346ea31F984` | Ethereum | [Etherscan](https://etherscan.io/address/0x1F98431c8aD98523631AE4a59f267346ea31F984) |
| V4 PoolManager | Various deployments | Ethereum | — |
| UniversalRouter | Various deployments | Ethereum | — |

## Key Events

- **`Swap`** — Emitted on every token swap (V2 Pair, V3 Pool, V4 PoolManager)
- **`PoolCreated`** — (V3/V2) Emitted when a new pool/pair is deployed
- **`Mint`** — Emitted when liquidity is added to a pool
- **`Burn`** — Emitted when liquidity is removed from a pool
- **`Initialize`** — (V3/V4) Emitted when a pool is initialized with a starting price

## Overview

### V1 Factory & Exchange
The original Uniswap — each token has a dedicated Exchange contract for ETH/token swaps. Simple and elegant.

### V2 Factory & Pair
Introduced direct token/token pairs and flash swaps. The Pair contract holds reserves and uses the x*y=k constant product formula.

### V3 Factory, Pool & SwapRouter
Concentrated liquidity allowing LPs to provide liquidity within specific price ranges. Dramatically improves capital efficiency.

### V4 PoolManager
Singleton architecture where all pools live in one contract. Introduces hooks for customizable pool logic and uses transient storage for gas savings.

### UniversalRouter
Unified router supporting complex multi-protocol transactions (Uniswap swaps, NFT purchases) in a single transaction.

## Links

- [Website](https://uniswap.org)
- [Docs](https://docs.uniswap.org)
- [GitHub](https://github.com/Uniswap)
