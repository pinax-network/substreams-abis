# Ethereum Name Service (ENS)

Decentralized naming system on Ethereum mapping human-readable names (e.g., `vitalik.eth`) to addresses and other records.

## Contracts

| Contract | Address | Chain | Explorer |
|----------|---------|-------|----------|
| ENSRegistry | `0x00000000000C2E074eC69A0dFb2997BA6C7d2e1e` | Ethereum | [Etherscan](https://etherscan.io/address/0x00000000000C2E074eC69A0dFb2997BA6C7d2e1e) |
| EthRegistrarController (v1) | Various deployments | Ethereum | — |
| EthRegistrarController (v0) | Various deployments | Ethereum | — |
| EthRegistrarController (base) | Various deployments | Base | — |
| PublicResolver (v1) | Various deployments | Ethereum | — |
| NameWrapper | Various deployments | Ethereum | — |
| ReverseRegistrar | Various deployments | Ethereum | — |

## Key Events

- **`NameRegistered`** — Emitted when a new ENS name is registered
- **`NameRenewed`** — Emitted when a name registration is renewed
- **`AddrChanged`** — Emitted when the address record for a name is updated
- **`NameWrapped`** — Emitted when a name is wrapped in the NameWrapper for enhanced permissions
- **`NewOwner`** — Emitted when ownership of a name node changes

## Overview

### ENSRegistry
The core registry mapping name nodes to their owner, resolver, and TTL. All ENS lookups start here.

### EthRegistrarController
Handles `.eth` name registration and renewal with pricing logic and commitment schemes to prevent front-running.

### PublicResolver
Default resolver storing address records, text records, content hashes, and other data for ENS names.

### NameWrapper
Wraps ENS names as ERC-1155 tokens, enabling fuse-based permission controls and expiry enforcement.

### ReverseRegistrar
Manages reverse resolution, mapping addresses back to ENS names (the `.addr.reverse` namespace).

## Links

- [Website](https://ens.domains)
- [Docs](https://docs.ens.domains)
- [GitHub](https://github.com/ensdomains)
