# Hashflow

Request-for-quote (RFQ) based DEX offering MEV-protected, zero-slippage trades with pricing from professional market makers.

## Contracts

| Contract | Address | Chain | Explorer |
|----------|---------|-------|----------|
| Pool | `0xCa310B1B942A30Ff4b40a5E1b69AB4607eC79Bc1` | Ethereum | [Etherscan](https://etherscan.io/address/0xCa310B1B942A30Ff4b40a5E1b69AB4607eC79Bc1) |
| Pool | `0xCa310B1B942A30Ff4b40a5E1b69AB4607eC79Bc1` | Arbitrum | [Arbiscan](https://arbiscan.io/address/0xCa310B1B942A30Ff4b40a5E1b69AB4607eC79Bc1) |
| Pool | `0xCa310B1B942A30Ff4b40a5E1b69AB4607eC79Bc1` | Polygon | [Polygonscan](https://polygonscan.com/address/0xCa310B1B942A30Ff4b40a5E1b69AB4607eC79Bc1) |
| Pool | `0xCa310B1B942A30Ff4b40a5E1b69AB4607eC79Bc1` | BNB Chain | [BscScan](https://bscscan.com/address/0xCa310B1B942A30Ff4b40a5E1b69AB4607eC79Bc1) |
| Pool | `0xCa310B1B942A30Ff4b40a5E1b69AB4607eC79Bc1` | Avalanche | [Snowtrace](https://snowtrace.io/address/0xCa310B1B942A30Ff4b40a5E1b69AB4607eC79Bc1) |
| Pool | `0x3efC75C8BeF358669b31c5c2d1F54FAe9d5dE8FD` | Optimism | [Etherscan](https://optimistic.etherscan.io/address/0x3efC75C8BeF358669b31c5c2d1F54FAe9d5dE8FD) |
| Router | `0x00000000F9490004C11Cef243f5400493c00Ad63` | Ethereum | [Etherscan](https://etherscan.io/address/0x00000000F9490004C11Cef243f5400493c00Ad63) |

## Key Events

### Pool
- **`Trade`** — Emitted when an RFQ trade is executed (trader, effectiveTrader, txid, baseToken, quoteToken, baseTokenAmount, quoteTokenAmount)
- **`XChainTrade`** — Emitted for cross-chain trade initiation
- **`XChainTradeFill`** — Emitted when a cross-chain trade is filled on the destination chain
- **`RemoveLiquidity`** — Emitted when a market maker removes liquidity
- **`UpdateSigner`** — Emitted when the pool signer is updated
- **`UpdateWithdrawalAccount`** — Emitted when a withdrawal account authorization changes

### Router
- **`NewConduit`** — Emitted when a new trading conduit is registered

## Overview

### Pool
The Hashflow Pool contract holds market maker liquidity and executes RFQ trades. Market makers provide signed quotes off-chain, and users submit them through the router for guaranteed-price execution without slippage. Each pool is associated with a specific market maker.

### Router
The core Hashflow router that routes RFQ trades to the appropriate pool. It manages conduit registration and trade execution.

## Links

- [Website](https://hashflow.com)
- [Docs](https://docs.hashflow.com)
- [GitHub](https://github.com/hashflownetwork)
