# Agent Instructions

Start with `docs/repo-navigation.md` for repository structure, edit routing, and command anchors.

When making durable documentation updates, keep `README.md` (user-facing overview) and `docs/repo-navigation.md` (maintainer navigation guide) aligned with the current repository layout and workflows.

## Dependencies

- Bun only: `package.json` sets `packageManager` and `devEngines`, so npm refuses to install or run scripts here. Use `bun install`, `bun run <script>` and `bun add <pkg>`.
- CI builds only the Rust crate and installs no JS dependencies; any job that does must use `bun install --frozen-lockfile`. Commit `bun.lock` with every dependency change.
- `bunfig.toml` lets `bun add` and `bun update` pick only versions published at least 5 days ago (`minimumReleaseAge = 432000`, in seconds) and saves exact versions. `.npmrc` sets `min-release-age=5` (in days) for any npm still in use.
- Never download a tool on the fly (`npx <pkg>`, `bunx <pkg>`, `<pkg>@latest`): add it as a pinned devDependency and run `bunx --no-install <bin>`.
- No Dependabot or Renovate: update dependencies by hand and review the `bun.lock` diff.
