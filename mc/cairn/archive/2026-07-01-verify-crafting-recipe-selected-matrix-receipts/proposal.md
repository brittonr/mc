## Candidate decision

Selected: `verify-crafting-recipe-selected-matrix-receipts`

Why now: the crafting behavior card, selected-matrix pure core, and validated Java Edition 1.20.1 / protocol 763 fixture now exist. The older `survival-crafting-recipe-breadth` Paper/reference and Valence receipts already record the same shaped chest, shapeless oak-planks, and invalid stick-input selected matrix. The missing seam is a fail-closed handoff proving that the fixture/core inputs are the same rows as the reviewable receipt evidence before any stronger target-version behavior claim is promoted.

Prerequisites satisfied: `docs/crafting-recipe-behavior-card.md`, `docs/crafting-recipe-selected-matrix-core.md`, `docs/crafting-recipe-selected-matrix-data-fixture.md`, accepted `cairn/specs/vanilla-composable-plugins/spec.md` crafting requirements, and archived `survival-crafting-recipe-breadth` paired receipt evidence under `docs/evidence/`.

Deferred alternatives: rerunning live Paper/Valence crafting receipts is deferred unless the handoff checker finds stale or mismatched evidence; Valence Bevy/ECS shell-contract work is deferred until the selected-matrix evidence bridge is reviewable; all-recipe extraction, data-pack loading, recipe-book behavior, automated crafter behavior, shift-click/drag/split handling, arbitrary collection modes, and DefaultPlugins membership changes remain separate targets.

Non-claims: no new live parity run in this planning package, no Valence runtime integration, no DefaultPlugins membership change, no all-recipe breadth, no arbitrary collection mode breadth, no data-pack or recipe-book behavior, no broad vanilla parity, no broad Minecraft compatibility, no public-server safety, and no production readiness.

## Why

The selected-matrix fixture and pure core are local unit evidence only. Separately, the archived survival crafting breadth work has Paper/reference and Valence receipt evidence for the same finite selected matrix. Without a deterministic bridge, reviewers must manually infer that these artifacts are equivalent, and later Valence shell work could overclaim from either the local fixture or the older receipt bundle.

## What Changes

- Add selected-matrix crafting receipt-handoff requirements to the `vanilla-composable-plugins` spec.
- Require a handoff contract that maps the Nickel fixture rows to normalized Paper/reference and Valence receipt metrics for target scope, row identity, inputs, outputs, collection mode, backend identity, receipt paths, and retained non-claims.
- Require a focused Rust checker with a pure comparison core plus a thin file-reading/export shell.
- Require positive and negative tests for matching selected-matrix evidence, missing or stale backend evidence, malformed receipt rows, item/count/slot mismatches, unsupported collection modes, missing non-claims, and overbroad crafting claims.
- Allow reusing archived `survival-crafting-recipe-breadth` Paper/Valence receipts only if the checker proves they match the selected fixture and preserve the bounded non-claims.

## Impact

- **Files**: future implementation will likely add `tools/check_crafting_recipe_receipt_handoff.rs`, a handoff document under `docs/`, promoted evidence under `docs/evidence/`, updates to crafting selected-matrix docs, and accepted `vanilla-composable-plugins` spec sync. This target-hunt package adds only the Cairn proposal/design/tasks/spec delta.
- **Testing**: planned checks include current fixture/core baseline validation, focused checker self-tests with positive and negative fixtures, selected-matrix handoff validation, Cairn gates/validation, task-evidence validation, accepted-spec sync verification, evidence-manifest checks, and flake evidence checks when implementation promotes artifacts.
- **Non-claims**: the change will not add a crafting Bevy/ECS shell, change Valence defaults, claim all recipes, claim all collection modes, claim data-pack or recipe-book behavior, or claim broad vanilla parity.
