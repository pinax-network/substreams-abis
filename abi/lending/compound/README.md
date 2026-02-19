# Compound

Algorithmic money market protocol enabling permissionless lending and borrowing of crypto assets. V3 (Comet) uses a single-asset borrow model.

## Contracts

| Contract | Address | Chain | Explorer |
|----------|---------|-------|----------|
| Comet (USDC) | Various deployments | Ethereum | — |

## Key Events

- **`Supply`** — Emitted when a user supplies the base asset (e.g., USDC)
- **`SupplyCollateral`** — Emitted when a user supplies collateral assets
- **`Withdraw`** — Emitted when a user withdraws the base asset
- **`WithdrawCollateral`** — Emitted when a user withdraws collateral
- **`AbsorbCollateral`** — Emitted during liquidation when collateral is absorbed

## Overview

### Comet (USDC)
The Compound V3 core contract for the USDC market. Unlike V2's multi-asset model, each Comet deployment focuses on a single borrowable asset with multiple collateral types, simplifying risk management.

## Links

- [Website](https://compound.finance)
- [Docs](https://docs.compound.finance)
- [GitHub](https://github.com/compound-finance)
