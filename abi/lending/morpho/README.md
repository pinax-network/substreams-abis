# Morpho

Peer-to-peer lending protocol that creates isolated markets with customizable parameters, enabling more efficient lending with higher rates for suppliers and lower rates for borrowers.

## Contracts

| Contract | Address | Chain | Explorer |
|----------|---------|-------|----------|
| MorphoBlue | Various deployments | Ethereum | — |

## Key Events

- **`Supply`** — Emitted when a user supplies assets to a market
- **`Borrow`** — Emitted when a user borrows assets from a market
- **`Repay`** — Emitted when a user repays borrowed assets
- **`Liquidate`** — Emitted when an undercollateralized position is liquidated
- **`CreateMarket`** — Emitted when a new isolated lending market is created

## Overview

### MorphoBlue
The core singleton contract for Morpho's isolated lending markets. Each market is defined by a unique combination of loan token, collateral token, oracle, IRM (interest rate model), and LLTV (liquidation loan-to-value). Fully permissionless and immutable.

## Links

- [Website](https://morpho.org)
- [Docs](https://docs.morpho.org)
- [GitHub](https://github.com/morpho-org)
