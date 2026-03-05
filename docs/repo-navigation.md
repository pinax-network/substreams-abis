# Repo Navigation Guide

This document captures durable, tool-agnostic knowledge for navigating and editing `substreams-abis`.

## Workspace layout

- `Cargo.toml`: crate manifest for `substreams-abis`
- `src/`: Rust binding modules consumed by crate users
- `abi/`: source ABI JSON files (organized to mirror `src/` module categories)
- `tests/`: Rust integration tests for event decoding
- `tools/codegen/`: Rust code generator that converts ABI JSON files to Rust bindings
- `cli.ts`, `lib/`, `package.json`: Bun/TypeScript operational tooling for ABI management
- `.github/workflows/`: CI (`ci.yml`) and publish (`publish.yml`) entrypoints
- `target/`, `node_modules/`: build/dependency artifacts

## Architecture and data flow (mental model)

1. ABI JSON files are maintained under `abi/...`.
2. Code generation reads those ABI files and writes Rust modules into `src/...`.
3. `mod.rs` files and `src/lib.rs` define the public module tree.
4. Integration tests under `tests/` validate decoding behavior against known logs.
5. CI runs `cargo test` and `cargo check --target wasm32-unknown-unknown`.

The `abi/` and `src/` trees are intentionally parallel, so protocol/category placement in one generally indicates where the counterpart belongs in the other.

## Common task routing (where to edit for X)

- Add a new ABI for an existing protocol:
  - edit `abi/<category>/<protocol>/.../*.json`
  - run codegen
  - ensure the corresponding `src/.../mod.rs` exports are present

- Add a brand-new top-level category:
  - add directories in `abi/` and `src/`
  - add `pub mod <category>;` in `src/lib.rs`
  - add module declarations in relevant `mod.rs`

- Adjust token ingestion/cleanup tooling:
  - edit TypeScript CLI internals under `lib/*.ts`
  - command wiring lives in `cli.ts`

- Debug decode behavior:
  - start from a failing integration test in `tests/*.rs`
  - trace target contract bindings in `src/...`
  - verify ABI inputs in `abi/...`

- Update CI/publish behavior:
  - edit `.github/workflows/ci.yml`
  - edit `.github/workflows/publish.yml`

## Key command anchors

- Generate bindings for all ABIs:
  - `cargo run --manifest-path tools/codegen/Cargo.toml`

- Generate bindings for specific ABI paths:
  - `cargo run --manifest-path tools/codegen/Cargo.toml -- abi/dex/uniswap/v4`

- Validate crate behavior:
  - `cargo test`
  - `cargo check --target wasm32-unknown-unknown`

- Run CLI help and workflows:
  - `bun run cli.ts --help`
  - `bun run cli.ts fetch-token ...`
  - `bun run cli.ts fetch-all ...`
  - `bun run cli.ts filter-standard ...`
  - `bun run cli.ts keccak ...`
  - `bun run cli.ts build-registry ...`

## Generated/artifact directories vs source-of-truth

- Source-of-truth inputs:
  - `abi/**/*.json` (primary ABI definitions)
  - selected TypeScript operational code under `lib/`
  - manifests/workflows (`Cargo.toml`, `package.json`, `.github/workflows/*.yml`)

- Generated or environment artifacts:
  - `src/**/*.rs` binding files are code-generated from ABI files (while `mod.rs`/`lib.rs` are still maintained as module wiring)
  - `target/` build artifacts
  - `node_modules/` installed JS dependencies

- Mixed artifact area to treat carefully:
  - `abi/tokens/erc20/*.sol` are fetched source artifacts useful for inspection, but Rust generation consumes ABI JSON files.
