# v2.0.0

## Breaking Changes

- Upgraded to `substreams` 0.8.0 and `substreams-ethereum` 0.12.0, which replace prost with buffa for the Firehose block model. A crate using `substreams-abis` 2.x must use these versions too. `substreams-abis` 1.x stays on `substreams` 0.7 and `substreams-ethereum` 0.11.
- Regenerated every binding with `substreams-ethereum-abigen` 0.12.0. Event and function decoders now use the `substreams_ethereum::abi` readers and writers instead of `ethabi`. Struct names, fields, field types, topic hashes and method IDs are unchanged.
  - Event `match_log` and `decode` are generic over `substreams_ethereum::LogLike`, so they take an owned `Log`, a `LogView` or buffa's `LogLazyView`.
  - Event `decode` called without `match_log` is stricter. It returns an error when the log's topic count differs from the event's, where v1.x ignored extra topics and panicked on missing ones. It also returns an error when the data is shorter than the event's minimum encoded size, which `ethabi` sometimes accepted. `match_log` already required both, so `match_log`, `match_and_decode` and `decode` after a successful `match_log` behave as before.
  - Generated files no longer declare a file-level `INTERNAL_ERR` constant.
- `ethabi` 17.2 is now built without default features. Only the constructor decoders use it, and this keeps `getrandom` out of wasm32 builds now that `substreams-ethereum` no longer configures it.

## Changes

- Constructor decoders (`constructor::Constructor`) are now generated for every ABI that declares a constructor. Before, only bindings regenerated after constructor support was added (v1.5.0) had them.
- `tools/codegen` uses `substreams-ethereum` 0.12.0. Each constructor module declares its own `INTERNAL_ERR`.
- Removed an empty fourth topic from the ERC-20 `Transfer` test fixture (the on-chain log has three topics) and added a check that `decode` rejects the extra topic.

# v1.6.0

## New ABIs

### Prediction Markets

- Added Polymarket V2 contract ABIs and generated bindings:
  - `CTFExchange`
  - `NegRiskCTFExchange`
  - `CollateralToken`
  - `CollateralOnramp`
  - `CollateralOfframp`
  - `PermissionedRamp`
  - `CtfCollateralAdapter`
  - `NegRiskCtfCollateralAdapter`

## Documentation

- Scoped existing Polymarket contracts under `v1/` and added `v2/` for the new Polymarket V2 contracts.
- Documented V1-only Polymarket core trading, wallet factory, and resolution contracts that do not have V2 ABIs.

# v1.5.1

## Fixes

- Fixed the CurveFi `StableSwap` constructor deployment-input regression test so constructor arguments are sliced correctly from real creation input.
- Corrected the expected StableSwap constructor `fee` value in the deployment-input test to match the on-chain payload.

# v1.1.0

## ✨ New Protocols

### DEXes
- **CurveFi** — StableSwap (3pool), CryptoSwap (TriCrypto2), MetaPool Registry, CryptoSwap Factory
- **Bancor V3** — Network, NetworkInfo
- **Bancor Carbon** — Carbon Controller (on-chain order book DEX)
- **Balancer V2** — Vault (the main swap entry point)
- **Hashflow** — RFQ-based Router
- **ParaSwap** — Augustus Swapper V6.2
- **WOOFi** — WooRouterV2 (Arbitrum)
- **KyberSwap V2** — MetaAggregationRouterV2
- **Fraxswap** — TWAMM Router
- **ShibaSwap** — UniswapV2Router02

### Other
- **Maker/Sky** — Vat, DaiJoin, DSRManager (stablecoin)
- **Chainlink** — OffchainAggregator, FeedRegistry (oracle)
- **WETH9** — Wrapped ETH with deposit/withdrawal events (token)
- **Camelot** — Router, Factory (Arbitrum DEX)
- **Convex** — Booster, BaseRewardPool (yield)
- **LayerZero** — Endpoint, UltraLightNodeV2 (bridge)

## 📚 Documentation

- **Protocol READMEs** — Every protocol directory now has a README.md with contract addresses, chain deployments, key events, and documentation links
- **Token READMEs** — All 75+ ERC-20 tokens documented with multi-chain deployment addresses (via CoinGecko), plus USDC/USDT variant docs and 7 NFT collections
- **Agent Skills** — New `.agents/skills/` directory with structured instructions for AI agents: add-abi workflow, naming conventions, build/test guide, PR process

## 🔧 CI/CD

- Auto-publish to crates.io on new GitHub releases
- Removed `cargo fmt` CI check (generated code isn't formatted)

## 📊 Stats

- **13 categories**, **50+ protocols**, **120+ contract ABIs**
- **80+ ERC-20 token ABIs** with multi-chain addresses
- **7 ERC-721 collection ABIs**
