# Aave

The largest decentralized lending and borrowing protocol, enabling users to supply and borrow crypto assets with variable interest rates.

## Contracts

| Contract | Address | Chain | Explorer |
|----------|---------|-------|----------|
| V3 Pool | `0x87870Bca3F3fD6335C3F4ce8392D69350B4fA4E2` | Ethereum | [Etherscan](https://etherscan.io/address/0x87870Bca3F3fD6335C3F4ce8392D69350B4fA4E2) |
| AToken | Various deployments | Ethereum | — |
| VariableDebtToken | Various deployments | Ethereum | — |
| AaveOracle | Various deployments | Ethereum | — |
| PoolAddressesProvider | Various deployments | Ethereum | — |

## Key Events

- **`Supply`** — Emitted when a user supplies assets to a lending pool
- **`Borrow`** — Emitted when a user borrows assets against their collateral
- **`Repay`** — Emitted when a user repays borrowed assets
- **`LiquidationCall`** — Emitted when an undercollateralized position is liquidated
- **`Withdraw`** — Emitted when a user withdraws supplied assets

## Overview

### V3 Pool
The core lending pool contract handling all supply, borrow, repay, and liquidation operations. Supports efficiency mode (eMode) for correlated assets and isolation mode for new listings.

### AToken
Interest-bearing token representing a user's supply position. Balance automatically increases as interest accrues.

### VariableDebtToken
Token representing a user's variable-rate borrow position. Balance increases over time as interest accumulates.

### AaveOracle
Price oracle aggregating asset prices from Chainlink feeds for collateral and liquidation calculations.

### PoolAddressesProvider
Registry contract storing addresses of all Aave V3 protocol components for a given market.

## Links

- [Website](https://aave.com)
- [Docs](https://docs.aave.com)
- [GitHub](https://github.com/aave)
