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
