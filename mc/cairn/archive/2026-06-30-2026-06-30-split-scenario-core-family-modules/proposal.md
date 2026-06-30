# Proposal: Split mc-compat scenario core into family modules

## Why

`compat/runner/src/scenario_core.rs` centralizes the scenario enum, aliases, milestones, behavior metadata, run strategies, live capability contracts, targeted packet contracts, and static validation in one large module. Scenario families now evolve independently, but edits still require touching a single broad file and revalidating unrelated behavior.

## What Changes

- Inventory scenario families, generated manifest ownership, live capability contracts, and existing validation surfaces before extraction.
- Split scenario behavior and contracts into focused family modules for CTF, inventory, survival, combat/projectile/equipment, negative rails, MCP, and targeted packet live capabilities.
- Keep `compat/config/scenario-manifest.ncl` and generated scenario surfaces as the source for stable scenario rows where possible.
- Preserve scenario names, aliases, run strategies, milestone IDs, forbidden patterns, receipt expectations, live capability rows, and non-claims.
- Add positive and negative tests for scenario lookup, alias coverage, manifest parity, family behavior, and live capability validation.

## Impact

- **Files**: `compat/runner/src/scenario_core.rs`, `scenario_behavior_metadata.rs`, `scenario_catalog.rs`, generated scenario surfaces if ownership changes, scenario tests, docs if derived-surface ownership changes, and Cairn artifacts.
- **Testing**: baseline scenario core tests/generated-surface checks, post-change scenario tests, maintained dry-runs for representative families, Cairn gates, and Cairn validation.
- **Non-claims**: scenario metadata maintainability only; this does not add new live compatibility evidence or broaden any scenario claim.
