# SunPump

Meme token launchpad on the Tron blockchain, enabling permissionless token creation and bonding curve sales.

## Contracts

| Contract | Address | Chain | Explorer |
|----------|---------|-------|----------|
| LaunchPad (legacy) | Various deployments | Tron | — |
| LaunchPadProxy (v1) | Various deployments | Tron | — |

## Key Events

- **`TokenCreate`** — Emitted when a new meme token is created
- **`TokenPurchased`** — Emitted when tokens are purchased on the bonding curve
- **`TokenSold`** — Emitted when tokens are sold on the bonding curve
- **`TokenLaunched`** — Emitted when a token graduates from the bonding curve to a DEX
- **`LaunchPending`** — Emitted when a token launch is pending finalization

## Overview

### LaunchPad (legacy)
The original launchpad contract managing token creation and bonding curve trading.

### LaunchPadProxy (v1)
The upgraded launchpad proxy contract with the same functionality, managing the lifecycle of meme tokens from creation through bonding curve to DEX launch.

## Links

- [Website](https://sunpump.meme)
