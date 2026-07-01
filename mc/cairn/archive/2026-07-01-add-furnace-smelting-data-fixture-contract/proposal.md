## Candidate decision

Selected: `add-furnace-smelting-data-fixture-contract`

Why now: the selected-row furnace core exists but intentionally uses local in-memory fixture rows only. The next evidence blocker is a target-version data fixture contract before Paper/vanilla parity or Valence shell work can make stronger claims.

Prerequisites satisfied: `docs/furnace-smelting-selected-row-core.md` names extracted Java Edition 1.20.1 recipe/fuel fixtures as next required evidence; `cairn/specs/vanilla-composable-plugins/spec.md` records selected-row core non-claims; archived Cairns `2026-07-01-add-furnace-smelting-behavior-card` and `2026-07-01-add-furnace-smelting-selected-row-core` provide the predecessor card and core.

Deferred alternatives: Paper/vanilla furnace receipts are deferred until target-version fixture rows exist; Valence Bevy/ECS shell work is deferred until data and schedule evidence exist; smoker/blast-furnace, hopper, XP, recipe-book, and chunk-unload behavior remain out of scope until separate behavior cards exist.

Non-claims: no fixture implementation in this planning package, no all-recipe breadth, no Paper/vanilla parity, no Valence runtime integration, no DefaultPlugins membership changes, no public-server safety, and no production readiness.

## Why

A future furnace selected-row claim needs reviewable Java Edition 1.20.1 recipe/fuel fixture data. The current core proves local unit semantics only. A dedicated fixture contract Cairn keeps the next implementation bounded and prevents accidental claims based on wiki text or ad-hoc local constants.

## What Changes

- Define a follow-on contract for a small extracted-data fixture surface covering one selected standard-furnace recipe row and one selected fuel row.
- Require a focused validator with positive and negative tests for schema shape, target version/protocol, selected-row membership, malformed rows, and non-claim boundaries.
- Extend the `vanilla-composable-plugins` spec with target-version fixture requirements before Paper parity or Valence shell work.

## Impact

- **Files**: Future implementation will likely add docs/fixture files and a focused Rust validator under `tools/`; this Cairn adds only proposal/design/tasks/spec delta.
- **Testing**: Planned tests must include positive and negative fixture validation plus Cairn gates, task-evidence, and evidence-manifest checks when implemented.
- **Non-claims**: This change package does not implement fixtures, prove target-version data, claim Paper/vanilla parity, change Valence runtime behavior, or broaden beyond the selected standard-furnace row.
