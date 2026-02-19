# Skill: Add New ABI

How to add a new protocol or token ABI to the repo.

## Prerequisites

- Etherscan API key (works across all chain explorers)
- Contract must be verified on the block explorer

## Steps

### 1. Fetch the ABI

```bash
curl "https://api.etherscan.io/api?module=contract&action=getabi&address=CONTRACT_ADDR&apikey=KEY"
```

Chain-specific explorers:
- **Ethereum:** `api.etherscan.io`
- **Base:** `api.basescan.org`
- **Optimism:** `api-optimistic.etherscan.io`
- **Arbitrum:** `api.arbiscan.io`
- **Avalanche:** `api.snowtrace.io`
- **BSC:** `api.bscscan.com`
- **Polygon:** `api.polygonscan.com`

Parse the response and extract the `result` field. Pretty-print and save as JSON.

### 2. Place the ABI JSON

Place in the correct category directory under `abi/`:

```
abi/
├── bridge/          — Cross-chain bridges
├── dex/             — DEXes and swap protocols
├── lending/         — Lending/borrowing protocols
├── naming/          — Name services (ENS)
├── oracle/          — Price oracles
├── perps/           — Perpetual exchanges
├── prediction/      — Prediction markets
├── restaking/       — Restaking protocols
├── stablecoin/      — Stablecoin systems
├── staking/         — Liquid staking
├── standard/        — Token standards (ERC-20, ERC-721, etc.)
├── tokens/          — Specific token ABIs
│   ├── erc20/       — ERC-20 token ABIs
│   └── erc721/      — ERC-721 collection ABIs
└── yield/           — Yield protocols
```

**Naming:** Use PascalCase for JSON filenames (e.g. `Router.json`, `PoolFactory.json`).

**Directory structure:** `abi/{category}/{protocol}/{Contract}.json` or with versions: `abi/{category}/{protocol}/v2/{Contract}.json`

### 3. Generate Rust bindings

Run the codegen tool to generate `.rs` files from the new ABI:

```bash
# Generate for a specific protocol directory
cd tools/codegen && cargo run -- ../../abi/dex/newprotocol/

# Or generate for a single file
cd tools/codegen && cargo run -- ../../abi/dex/newprotocol/Router.json

# Or regenerate ALL (slow, rarely needed)
cd tools/codegen && cargo run
```

The tool reads ABI JSON files and generates typed Rust bindings in the corresponding `src/` path.

### 4. Create/Update mod.rs

`mod.rs` files are **manual** — the codegen tool only generates the `.rs` bindings.

For a new protocol directory, create `src/{category}/{protocol}/mod.rs`:
```rust
pub mod contractname;  // lowercase of the JSON filename stem
```

If adding to an existing directory, append the new module to the existing `mod.rs`.

If the parent category is new, also add `pub mod {protocol};` to `src/{category}/mod.rs` and `pub mod {category};` to `src/lib.rs`.

#### Special cases

- **Rust keywords** as module names: use raw identifiers (`pub mod r#yield;`)
- **Filenames with dots/special chars** (e.g. Solidity compiler versions): use `#[path]` attribute:
  ```rust
  #[path = "tethertoken_v0.4.18+commit.9cf6e910.rs"]
  pub mod tethertoken_v0_4_18;
  ```
- **Case-sensitive directories** (e.g. `UmaCtfAdapter`): use `#[path]` attribute:
  ```rust
  #[path = "UmaCtfAdapter/mod.rs"]
  pub mod umactfadapter;
  ```
- **Module names from invalid identifiers:** `1inch` → `oneinch`, `0x` → `zerox`

### 5. Build and Test

```bash
cargo build   # compiles the library (no codegen — that's done in step 3)
cargo test    # run tests
```

### 6. Update README.md

**Required:** Create or update the protocol's `README.md` in the ABI directory.

Include:
- Protocol name and description
- Contract table: Name, Address, Chain, Explorer link
- Key events (read from the ABI JSON — look for `"type": "event"`)
- Overview of each contract
- Links to official site and docs

See any existing `abi/dex/*/README.md` for examples.

### 7. Update Token READMEs (if adding tokens)

If adding ERC-20 tokens:
- Update `abi/tokens/erc20/README.md` — add to the token table and multi-chain deployments section
- Use CoinGecko API to look up multi-chain addresses

If adding ERC-721 tokens:
- Update `abi/tokens/erc721/README.md`

### 8. Commit and PR

- Branch naming: `feat/{description}` for new ABIs, `docs/{description}` for docs-only
- Always run `cargo build` and `cargo test` before pushing
- Add `DenisCarriere` as PR reviewer
- Don't bump the version in `Cargo.toml` unless instructed

## Token ABI Guidelines

Only add specific token ABIs if they have **custom events beyond standard ERC-20**. Standard ERC-20 Transfer/Approval is already in `abi/standard/ERC20.json`.

Examples of tokens worth adding:
- WETH9 (Deposit/Withdrawal events)
- Bridged tokens with mint/burn
- Governance tokens with delegation events
- Rebasing tokens (stETH) with custom events
