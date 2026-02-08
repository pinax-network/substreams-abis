# Release Notes - v0.9.0

## Summary

Added 20 new ERC-20 token ABIs with Solidity source and generated Rust bindings (15 ETH Mainnet tokens, 5 Base chain tokens). Also added 3 utility scripts for fetching token ABIs and auto-generating module declarations.

## New Token ABIs Added

### ETH Mainnet Tokens (15)

| Token | Symbol | Contract Address | Rust Module |
|-------|--------|-----------------|-------------|
| Shiba Inu | SHIB | `0x95aD61b0a150d79219dCF64E1E6Cc01f0B64C4cE` | `substreams_abis::evm::tokens::shib` |
| Chainlink | LINK | `0x514910771AF9Ca656af840dff83E8264EcF986CA` | `substreams_abis::evm::tokens::link` |
| Uniswap | UNI | `0x1f9840a85d5aF5bf1D1762F925BDADdC4201F984` | `substreams_abis::evm::tokens::uni` |
| Polygon (MATIC) | MATIC | `0x7D1AfA7B718fb893dB30A3aBc0Cfc608AaCfeBB0` | `substreams_abis::evm::tokens::matic` |
| Arbitrum | ARB | `0xB50721BCf8d664c30412Cfbc6cf7a15145234ad1` | `substreams_abis::evm::tokens::arb` |
| Lido DAO | LDO | `0x5A98FcBEA516Cf06857215779Fd812CA3beF1B32` | `substreams_abis::evm::tokens::ldo` |
| Aave | AAVE | `0x7Fc66500c84A76Ad7e9c93437bFc5Ac33E2DDaE9` | `substreams_abis::evm::tokens::aave` |
| Maker | MKR | `0x9f8F72aA9304c8B593d555F12eF6589cC3A579A2` | `substreams_abis::evm::tokens::mkr` |
| Curve DAO | CRV | `0xD533a949740bb3306d119CC777fa900bA034cd52` | `substreams_abis::evm::tokens::crv` |
| Pepe | PEPE | `0x6982508145454Ce325dDbE47a25d4ec3d2311933` | `substreams_abis::evm::tokens::pepe` |
| ApeCoin | APE | `0x4d224452801ACEd8B2F0aebE155379bb5D594381` | `substreams_abis::evm::tokens::ape` |
| Synthetix | SNX | `0xC011a73ee8576Fb46F5E1c5751cA3B9Fe0af2a6F` | `substreams_abis::evm::tokens::snx` |
| Compound | COMP | `0xc00e94Cb662C3520282E6f5717214004A7f26888` | `substreams_abis::evm::tokens::comp` |
| The Graph | GRT | `0xc944E90C64B2c07662A292be6244BDf05Cda44a7` | `substreams_abis::evm::tokens::grt` |
| Fetch.ai | FET | `0xaea46A60368A7bD060eec7DF8CBa43b7EF41Ad85` | `substreams_abis::evm::tokens::fet` |

### Base Chain Tokens (5)

| Token | Symbol | Contract Address | Rust Module |
|-------|--------|-----------------|-------------|
| Aerodrome | AERO | `0x940181a94A35A4569E4529A3CDfB74e38FD98631` | `substreams_abis::evm::tokens::aero` |
| Brett | BRETT | `0x532f27101965dd16442E59d40670FaF5eBB142E4` | `substreams_abis::evm::tokens::brett` |
| Degen | DEGEN | `0x4ed4E862860beD51a9570b96d89aF5E1B0Efefed` | `substreams_abis::evm::tokens::degen` |
| Toshi | TOSHI | `0xAC1Bd2486aAf3B5C0fc3Fd868558b082a531B2B4` | `substreams_abis::evm::tokens::toshi` |
| Moonwell | WELL | `0xA88594D404727625A9437C3f886C7643872296AE` | `substreams_abis::evm::tokens::well` |

## New Files Added

### ABI JSON Files (20)
- `abi/evm/tokens/AAVE.json`
- `abi/evm/tokens/AERO.json`
- `abi/evm/tokens/APE.json`
- `abi/evm/tokens/ARB.json`
- `abi/evm/tokens/BRETT.json`
- `abi/evm/tokens/COMP.json`
- `abi/evm/tokens/CRV.json`
- `abi/evm/tokens/DEGEN.json`
- `abi/evm/tokens/FET.json`
- `abi/evm/tokens/GRT.json`
- `abi/evm/tokens/LDO.json`
- `abi/evm/tokens/LINK.json`
- `abi/evm/tokens/MATIC.json`
- `abi/evm/tokens/MKR.json`
- `abi/evm/tokens/PEPE.json`
- `abi/evm/tokens/SHIB.json`
- `abi/evm/tokens/SNX.json`
- `abi/evm/tokens/TOSHI.json`
- `abi/evm/tokens/UNI.json`
- `abi/evm/tokens/WELL.json`

### Solidity Source Files (20)
- `abi/evm/tokens/AAVE.sol`
- `abi/evm/tokens/AERO.sol`
- `abi/evm/tokens/APE.sol`
- `abi/evm/tokens/ARB.sol`
- `abi/evm/tokens/BRETT.sol`
- `abi/evm/tokens/COMP.sol`
- `abi/evm/tokens/CRV.sol`
- `abi/evm/tokens/DEGEN.sol`
- `abi/evm/tokens/FET.sol`
- `abi/evm/tokens/GRT.sol`
- `abi/evm/tokens/LDO.sol`
- `abi/evm/tokens/LINK.sol`
- `abi/evm/tokens/MATIC.sol`
- `abi/evm/tokens/MKR.sol`
- `abi/evm/tokens/PEPE.sol`
- `abi/evm/tokens/SHIB.sol`
- `abi/evm/tokens/SNX.sol`
- `abi/evm/tokens/TOSHI.sol`
- `abi/evm/tokens/UNI.sol`
- `abi/evm/tokens/WELL.sol`

### Generated Rust Modules (20)
- `src/evm/tokens/aave.rs`
- `src/evm/tokens/aero.rs`
- `src/evm/tokens/ape.rs`
- `src/evm/tokens/arb.rs`
- `src/evm/tokens/brett.rs`
- `src/evm/tokens/comp.rs`
- `src/evm/tokens/crv.rs`
- `src/evm/tokens/degen.rs`
- `src/evm/tokens/fet.rs`
- `src/evm/tokens/grt.rs`
- `src/evm/tokens/ldo.rs`
- `src/evm/tokens/link.rs`
- `src/evm/tokens/matic.rs`
- `src/evm/tokens/mkr.rs`
- `src/evm/tokens/pepe.rs`
- `src/evm/tokens/shib.rs`
- `src/evm/tokens/snx.rs`
- `src/evm/tokens/toshi.rs`
- `src/evm/tokens/uni.rs`
- `src/evm/tokens/well.rs`

### Utility Scripts (3)
- `scripts/fetch-token.sh` -- Fetch ABI and Solidity source for a single token from Etherscan/Basescan. Supports ETH, Base, Arbitrum, Optimism, Polygon, and BSC chains.
- `scripts/fetch-all-tokens.sh` -- Batch-fetch ABIs for all tokens in the built-in registry (44 tokens). Supports `--chain` filtering and `--missing` to skip already-downloaded tokens.
- `scripts/update-modules.sh` -- Auto-generate `src/evm/tokens/mod.rs` from existing Rust source files after adding new ABIs. Runs `cargo build` to produce Rust bindings first.

## Usage Examples

### Importing new token modules

```rust
use substreams_abis::evm::tokens::shib;
use substreams_abis::evm::tokens::link;
use substreams_abis::evm::tokens::uni;
use substreams_abis::evm::tokens::aave;
use substreams_abis::evm::tokens::aero;
```

### Decoding events

```rust
use substreams_abis::evm::tokens::shib::events::Transfer;
use substreams_abis::evm::tokens::uni::events::Approval;

for (log, call_view) in trx.logs_with_calls() {
    if let Some(transfer) = Transfer::decode(&log) {
        // transfer.from, transfer.to, transfer.value
    }
}
```

### Adding more tokens with the new scripts

```bash
# Fetch a single token
./scripts/fetch-token.sh SUSHI 0x6B3595068778DD592e39A122f4f5a5cF09C90fE2

# Fetch a Base token
./scripts/fetch-token.sh MORPHO 0xBAa5CC21fd487B8Fcc2F632f3F4E8D37262a0842 base

# Fetch all tokens in the registry that haven't been downloaded yet
./scripts/fetch-all-tokens.sh --missing

# Regenerate mod.rs after adding new ABIs
./scripts/update-modules.sh
```

## Breaking Changes

None. This is an additive-only release.

## Build & Test

- `cargo build` passes
- All 12 tests pass
- 30 total token modules in `src/evm/tokens/mod.rs` (10 existing + 20 new)

## Version

- Previous: 0.8.0
- New: 0.9.0
