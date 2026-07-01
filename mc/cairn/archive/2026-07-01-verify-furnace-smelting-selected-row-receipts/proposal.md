## Candidate decision

Selected: `verify-furnace-smelting-selected-row-receipts`

Why now: the furnace behavior card, pure selected-row core, and validated Java Edition 1.20.1 fixture now exist, and the older `survival-furnace-smelting-breadth` Paper/Valence receipts already record matching RawIron + Coal selected-row metrics. The missing seam is a fail-closed handoff that proves the selected fixture/core inputs are the same row as the reviewable receipt evidence before any stronger target-version claim is promoted.

Prerequisites satisfied: `docs/furnace-smelting-behavior-card.md`, `docs/furnace-smelting-selected-row-core.md`, `docs/furnace-smelting-selected-row-data-fixture.md`, accepted `cairn/specs/vanilla-composable-plugins/spec.md` furnace requirements, and archived `2026-06-21-survival-furnace-smelting-breadth-parity` paired receipts.

Deferred alternatives: rerunning live Paper/Valence furnace receipts is deferred unless the handoff checker finds stale or mismatched evidence; Valence Bevy/ECS shell integration is deferred until the selected-row evidence bridge is reviewable; all-recipe extraction and smoker/blast-furnace, hopper, XP, recipe-book, data-pack, and chunk-unload work remain separate targets.

Non-claims: no new live parity run in this planning package, no Valence runtime integration, no DefaultPlugins membership change, no all-recipe breadth, no broad vanilla parity, no broad Minecraft compatibility, no public-server safety, and no production readiness.

## Why

The current selected-row fixture and pure core are local unit evidence only. Separately, the archived survival breadth work has Paper/reference and Valence receipt evidence for the same RawIron + Coal row. Without a deterministic bridge, reviewers must manually infer that these artifacts are the same selected row, and later Valence shell work could overclaim from either the local fixture or the older receipt bundle.

## What Changes

- Add a selected-row receipt-handoff contract for comparing the Nickel fixture row against normalized Paper/reference and Valence receipt metrics.
- Require a Rust checker with a pure comparison core plus a thin file-reading shell.
- Require positive and negative fixtures for matching selected-row evidence, stale/missing backend evidence, item/count/timing mismatches, missing non-claims, and broad-furnace overclaims.
- Allow reusing the archived `survival-furnace-smelting-breadth` Paper/Valence receipts only if the checker proves they match the selected fixture and preserve the bounded non-claims.

## Impact

- **Files**: future implementation will likely touch `tools/`, `docs/furnace-smelting-selected-row-data-fixture.md`, `docs/evidence/`, and the accepted `vanilla-composable-plugins` spec during sync; this Cairn adds only proposal/design/tasks/spec delta.
- **Testing**: planned checks include existing fixture/core baseline validation, new checker self-tests with positive and negative fixtures, selected-row handoff validation, Cairn gates/validation, task-evidence validation, accepted-spec sync verification, and evidence-manifest checks.
- **Non-claims**: the change will not add a Bevy/ECS furnace shell, change Valence defaults, claim all recipes/fuels, or claim broad vanilla parity.
