# Proposal: Migrate remaining survival breadth rows to typed-event pass/fail

## Why

The remaining survival breadth rows share recent paired Paper/Valence evidence and copied typed-event logs, but the scenario manifest still keeps them under waiver-backed substring fallback. The affected rows are `survival-mob-ai-loot-breadth`, `survival-redstone-circuit-breadth`, `survival-biome-dimension-travel`, `survival-world-multichunk-durability`, `survival-container-block-entity-breadth`, and `survival-sign-editing-live`.

Migrating these related rows together burns down the largest survival fallback cluster while preserving the row-scoped survival non-claims and avoiding a premature aggregate survival claim.

## What Changes

- Mark the six remaining survival breadth rows as `typed-event-ready` in the scenario manifest and generated surfaces.
- Extend the typed-event pass/fail gate for each row's required client milestones, Valence server milestones, forbidden surfaces, and row-specific ordering.
- Add positive and negative fixtures for missing evidence and misordered phases across the six row families.
- Add manifest readiness fixtures and documentation updates for the typed-event-ready scenario set.
- Preserve wrappers, receipt schema, dry-run shapes, current-bundle rows, and non-claims.

## Impact

- **Files**: `compat/config/scenario-manifest.ncl`, generated scenario surfaces, `compat/runner/src/main.rs`, `tools/check_scenario_manifest.rs`, README/evidence docs, and Cairn lifecycle files.
- **Testing**: focused typed-event tests, row-family negative fixtures, scenario-manifest checks, generated-surface freshness, historical survival dry-run coverage, evidence manifest validation, Cairn gates, and Cairn validation.
- **Non-claims**: this changes only validation basis for the listed bounded rows. It does not claim full survival compatibility, all mobs, general redstone parity, all dimensions, arbitrary durability, all containers, all sign UI behavior, public-server safety, production readiness, or semantic equivalence.
