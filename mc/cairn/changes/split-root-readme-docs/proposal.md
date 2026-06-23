# Proposal: Split root README into focused documentation

## Why

The root README mixes purpose, layout, command reference, scenario command listings, config details, evidence workflow, and verification notes. The long command wall is hard to review and duplicates manifest-derived scenario data. A shorter README with focused linked docs would improve navigation and reduce merge churn.

## What Changes

- Keep root README focused on purpose, component layout, common commands, and links to deeper docs.
- Move scenario command listings, evidence workflow details, config/reference notes, and verification tiers into focused files under `docs/`.
- Generate or validate scenario command/index sections from the scenario manifest where stable.
- Add documentation freshness checks so moved/generated command listings do not drift from flake wrappers or manifest rows.

## Impact

- **Files**: `README.md`, new `docs/*.md` pages, generated docs/index files, scenario manifest checker/generator if touched, evidence docs references, and Cairn artifacts.
- **Testing**: docs link/path checks if available, generated docs freshness, scenario manifest checks, selected dry-run command parity, and Cairn validation/gates.
- **Non-claims**: this is documentation modularity only; it does not change command behavior, scenario semantics, or compatibility evidence claims.
