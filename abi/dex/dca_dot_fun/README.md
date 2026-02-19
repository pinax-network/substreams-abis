# DCA.fun

Dollar-cost averaging protocol enabling automated recurring token purchases on-chain.

## Contracts

| Contract | Address | Chain | Explorer |
|----------|---------|-------|----------|
| DcaDotFun | Various deployments | — | — |

## Key Events

- **`CreateOrder`** — Emitted when a new DCA order is created
- **`FillOrder`** — Emitted when a DCA order is executed (filled)
- **`CancelOrder`** — Emitted when a DCA order is cancelled
- **`SetProtocolFee`** — Emitted when the protocol fee is updated
- **`SetFeeCollector`** — Emitted when the fee collector address changes

## Overview

### DcaDotFun
The core contract managing DCA orders. Users create recurring buy orders specifying token, amount, and frequency. Orders are filled by keepers who execute swaps at the scheduled intervals.

## Links

- [Website](https://dca.fun)
