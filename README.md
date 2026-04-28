# Substreams ABIs

`substreams-abis` is a Rust crate plus companion Bun/TypeScript tooling for maintaining ABI files and generated Substreams Ethereum bindings.

For a maintainer-oriented map of edit points and workflows, see `docs/repo-navigation.md`.

## Repository layout

- `abi/`: source ABI JSON files (plus some fetched Solidity artifacts under token folders)
- `src/`: generated and curated Rust modules exposed by the crate
- `tests/`: Rust integration tests that decode real-world logs with generated bindings
- `lib/` + `cli.ts`: Bun/TypeScript CLI for ABI ingestion and maintenance tasks
- `tools/codegen/`: Rust ABI → binding generator used to write files into `src/`

Protocol folders may include explicit version scopes such as `v1/` and `v2/`
when contracts evolve across major deployments. Polymarket prediction-market
ABIs use this layout, with unchanged V1 contracts kept under
`abi/prediction/polymarket/v1`.

## Coverage areas

The repository currently includes modules and ABIs for:

- standards and extensions (`standard`)
- tokens and collections (`tokens`)
- DEXs and aggregators (`dex`)
- bridges (`bridge`)
- lending (`lending`)
- naming (`naming`)
- oracles (`oracle`)
- perps (`perps`)
- prediction markets (`prediction`)
- restaking (`restaking`)
- stablecoin systems (`stablecoin`)
- staking (`staking`)
- yield strategies (`yield`)

## Quickstart (Rust)

```rust
use substreams_abis::standard::erc20::events::Transfer;

for trx in block.transactions() {
  for (log, _call_view) in trx.logs_with_calls() {
    let transfer = match Transfer::decode(&log) {
      Ok(transfer) => transfer,
      Err(_) => continue,
    };

    // transfer.from
    // transfer.to
    // transfer.value
  }
}
```

## Common maintenance workflow

1. Add or update ABI JSON files in the relevant `abi/...` directory.
2. Regenerate Rust bindings:

   ```bash
   cargo run --manifest-path tools/codegen/Cargo.toml
   ```

   You can also target only specific files or folders:

   ```bash
   cargo run --manifest-path tools/codegen/Cargo.toml -- abi/dex/uniswap/v4
   ```

3. Update `mod.rs` files where needed (for new modules).
4. Run verification:

   ```bash
   cargo test
   cargo check --target wasm32-unknown-unknown
   ```

## CLI usage (Bun)

```bash
bun run cli.ts fetch-token SHIB 0x95aD61b0a150d79219dCF64E1E6Cc01f0B64C4cE --chain eth
bun run cli.ts fetch-all --chain base --missing --no-duplicates
bun run cli.ts filter-standard --delete
bun run cli.ts keccak -s "Transfer(address,address,uint256)"
bun run cli.ts build-registry -o token-registry.json
bun run cli.ts update-modules
```
