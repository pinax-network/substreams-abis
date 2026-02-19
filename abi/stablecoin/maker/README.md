# MakerDAO (Sky)

The protocol behind DAI, the largest decentralized stablecoin. Users lock collateral in vaults to mint DAI.

## Contracts

| Contract | Address | Chain | Explorer |
|----------|---------|-------|----------|
| Vat | `0x35D1b3F3D7966A1DFe207aa4514C12a259A0492B` | Ethereum | [Etherscan](https://etherscan.io/address/0x35D1b3F3D7966A1DFe207aa4514C12a259A0492B) |
| DaiJoin | Various deployments | Ethereum | — |
| DSRManager | Various deployments | Ethereum | — |

## Key Events

- **`LogNote`** — Generic event used by Vat and DaiJoin for all state changes (Maker uses a `note` modifier pattern)
- **`Join`** — (DSRManager) Emitted when DAI is deposited into the DAI Savings Rate
- **`Exit`** — (DSRManager) Emitted when DAI is withdrawn from the DAI Savings Rate

> Maker's core contracts (Vat, DaiJoin) use the `LogNote` pattern where function calls are logged as generic events. Indexers typically decode the function signature from the `LogNote` data.

## Overview

### Vat
The core accounting engine of the Maker Protocol. Tracks all collateral positions (vaults), debt, and system surplus. All DAI creation and destruction flows through the Vat.

### DaiJoin
Adapter contract that converts internal Vat DAI balance to the external ERC-20 DAI token and vice versa.

### DSRManager
Simplified interface for depositing DAI into the DAI Savings Rate (DSR) to earn yield.

## Links

- [Website](https://makerdao.com)
- [Docs](https://docs.makerdao.com)
- [GitHub](https://github.com/makerdao)
