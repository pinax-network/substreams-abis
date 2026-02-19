# Skill: PR Workflow

## Branch Naming

- `feat/{description}` — New ABIs or features
- `docs/{description}` — Documentation only
- `refactor/{description}` — Code reorganization
- `fix/{description}` — Bug fixes

## Before Opening a PR

1. **Rebase on latest main:**
   ```bash
   git fetch origin main
   git rebase origin/main
   ```

2. **Build and test:**
   ```bash
   cargo build
   cargo test
   ```

3. **Verify WASM target:**
   ```bash
   cargo check --target wasm32-unknown-unknown
   ```

## Opening a PR

- Always add `DenisCarriere` as reviewer
- Use descriptive PR titles: `feat: add CurveFi StableSwap, CryptoSwap ABIs`
- Include a summary of changes in the PR body
- List any skipped items and why (e.g. "Clipper skipped — contract unverified")

## After Opening a PR

- **Wait for merge** before opening the next PR (avoid conflicts)
- If CI fails, fix and force-push to the same branch
- Don't bump `Cargo.toml` version unless instructed

## Git Push

For branches with large generated `.rs` files:
```bash
git -c http.postBuffer=524288000 push
```

## Issues

- Use `pax` label for issues that should be auto-picked up by AI agents
- Assign `DenisCarriere` on issues needing human feedback
- React 🚀 on issues when starting work on them

## Releases

- Only create releases when instructed
- Tag format: `v{major}.{minor}.{patch}`
- Include `RELEASE_NOTES.md` content in the release body
- Releases are created via GitHub API, not git tags
