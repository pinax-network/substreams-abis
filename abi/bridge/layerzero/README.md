# LayerZero

Omnichain interoperability protocol enabling cross-chain messaging between blockchains.

## Contracts

| Contract | Address | Chain | Explorer |
|----------|---------|-------|----------|
| Endpoint | `0x66A71Dcef29A0fFBDBE3c6a460a3B5BC225Cd675` | Ethereum | [Etherscan](https://etherscan.io/address/0x66A71Dcef29A0fFBDBE3c6a460a3B5BC225Cd675) |
| UltraLightNodeV2 | `0x4D73AdB72bC3DD368966edD0f0b2148401A178E2` | Ethereum | [Etherscan](https://etherscan.io/address/0x4D73AdB72bC3DD368966edD0f0b2148401A178E2) |

## Key Events

- **`Packet`** — Emitted when a cross-chain message packet is sent
- **`PacketReceived`** — Emitted when a message packet is received on the destination chain
- **`PayloadStored`** — Emitted when a message payload is stored for later execution
- **`HashReceived`** — Emitted when a message hash is received by the UltraLightNode
- **`RelayerParams`** — Emitted with relayer configuration parameters

## Overview

### Endpoint
The user-facing contract for sending and receiving cross-chain messages. Applications interact with the Endpoint to dispatch messages to other chains.

### UltraLightNodeV2
The messaging library that validates cross-chain messages using configurable oracle and relayer pairs. Handles the security verification of message packets.

## Links

- [Website](https://layerzero.network)
- [Docs](https://docs.layerzero.network)
- [GitHub](https://github.com/LayerZero-Labs)
