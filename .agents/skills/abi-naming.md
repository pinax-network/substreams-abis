# Skill: ABI Naming Conventions

## File Naming

- **ABI JSON files:** PascalCase (e.g. `Router.json`, `PoolFactory.json`, `StableSwap.json`)
- **Generated .rs files:** Auto-lowercased by `build.rs` (e.g. `router.rs`, `poolfactory.rs`)
- **Module names in mod.rs:** Match the lowercased filename stem

## Directory Naming

- **Protocol dirs:** lowercase, no hyphens (e.g. `curvefi`, `pancakeswap`, `traderjoe`)
- **Version dirs:** `v1/`, `v2/`, `v3/`, `legacy/`
- **Category dirs:** lowercase (e.g. `dex`, `lending`, `bridge`)

## Invalid Rust Identifiers

Some protocol names are invalid Rust identifiers:

| Protocol | Directory Name | Module Name |
|----------|---------------|-------------|
| 1inch | `oneinch` | `oneinch` |
| 0x | `zerox` | `zerox` |
| yield (keyword) | `yield` | `r#yield` |

## Filenames with Special Characters

`build.rs` lowercases the file stem but preserves dots, plus signs, and hyphens in filenames. These can't be Rust module names directly. Use `#[path]` attributes:

```rust
// For files like: tethertoken_v0.4.18+commit.9cf6e910.rs
#[path = "tethertoken_v0.4.18+commit.9cf6e910.rs"]
pub mod tethertoken_v0_4_18;
```

## Case-Sensitive Directories

`build.rs` only replaces hyphens with underscores in directory components — it does NOT lowercase them. If an ABI is in a mixed-case directory (e.g. `UmaCtfAdapter/`), the generated `.rs` files will be in a mixed-case dir too. Handle with `#[path]`:

```rust
#[path = "UmaCtfAdapter/mod.rs"]
pub mod umactfadapter;
```

## Hyphen Handling

`build.rs` converts hyphens to underscores in directory names only (e.g. `erc20-tokens/` → `erc20_tokens/`). Hyphens in filenames are preserved in the generated `.rs` filename. Use `#[path]` if needed.

## When Merging vs. Keeping Separate ABIs

### Merge (keep superset) when:
- One ABI is a strict superset of another (same events + all functions + extras)
- Example: USDC FiatTokenV2_1 ⊂ FiatTokenV2_2

### Keep separate when:
- Different event signatures (different topic hashes)
- Different chains/bridge variants
- Different admin models
- Neither is a superset of the other
