# Gameplay plugin composition implementation inventory

## Question

Do the Valence gameplay plugin composition changes have a reviewable inventory and implementation boundary for shared contracts, arena/layer scope, and config source separation?

## Inspected evidence

- `servers/valence/examples/ctf.rs` owned a local CTF phase enum, a local plugin contract resource, CTF runtime config parsing over environment inputs, per-tick env refresh systems in gameplay input phases, CTF globals, and gameplay systems that were not guarded by an explicit arena/layer scope marker.
- `servers/valence/examples/survival_compat.rs` owned a local survival phase enum, a local plugin contract resource, env-backed runtime config parsing inside the gameplay plugin, and gameplay systems without an explicit scope resource.
- `servers/valence/examples/terrain.rs` owned a local terrain phase enum and terrain generation systems without shared phase metadata or scope assertions.
- The active Cairn changes require one shared contract vocabulary, explicit plugin scope, and source-adapter/resource separation while preserving non-claims about broad Minecraft compatibility, semantic equivalence, and production readiness.

## Decision

- Added `servers/valence/examples/gameplay_contracts/mod.rs` as the shared functional core for gameplay plugin metadata:
  - `GameplayPhase` and `GAMEPLAY_PHASE_ORDER` provide ordered phase metadata reused by CTF, survival, and terrain.
  - `GameplayPluginContract` plus `GameplayPluginRegistry` provide queryable plugin IDs, phase coverage, and explicit `requires_config_source` metadata.
  - `GameplayScope` and `GameplayScopeRegistry` model arena/layer ownership for focused examples without introducing a public production API claim.
- Migrated CTF, survival, and terrain plugins to insert shared contract metadata and primary gameplay scopes.
- Split env/config I/O into explicit source-adapter plugins:
  - `CtfRuntimeConfigSourcePlugin` owns `CtfRuntimeConfig::from_env`, config insertion, and reload events.
  - `SurvivalRuntimeConfigSourcePlugin` owns `SurvivalRuntimeConfig::from_env`, config insertion, and reload events.
  - Gameplay plugins consume typed/default resources and do not directly read environment variables.
- Added focused positive and negative tests for shared contract metadata, source-adapter reloads, disabled-source absence, missing scopes, wrong scopes, and stale scoped entities.

## Validation evidence

- Focused Valence example checks are recorded in `docs/evidence/run-logs/2026-06-26/gameplay-plugin-composition-focused-tests.run.log`.
- Cairn gates/validation are recorded in `docs/evidence/run-logs/2026-06-26/gameplay-plugin-composition-cairn-gates.run.log`.
- Task-evidence validation is recorded in `docs/evidence/run-logs/2026-06-26/gameplay-plugin-composition-task-evidence-prearchive.run.log`, `docs/evidence/run-logs/2026-06-26/gameplay-plugin-composition-task-evidence-final.run.log`, and `docs/evidence/run-logs/2026-06-26/gameplay-plugin-composition-task-evidence-postarchive.run.log`.
- Accepted-spec reconciliation is recorded in `docs/evidence/run-logs/2026-06-26/gameplay-plugin-composition-accepted-spec-reconcile.run.log` after the sync receipts showed no accepted-spec content delta.
- Evidence-manifest refresh/check evidence is recorded in `docs/evidence/run-logs/2026-06-26/gameplay-plugin-composition-evidence-manifest-refresh.run.log`, `docs/evidence/run-logs/2026-06-26/gameplay-plugin-composition-evidence-manifest-check-final.run.log`, `docs/evidence/run-logs/2026-06-26/gameplay-plugin-composition-evidence-manifest-check-postarchive.run.log`, `docs/evidence/run-logs/2026-06-26/gameplay-plugin-composition-evidence-manifests-nix-check-final.run.log`, `docs/evidence/run-logs/2026-06-26/gameplay-plugin-composition-post-reconcile-evidence-manifest-refresh.run.log`, and `docs/evidence/run-logs/2026-06-26/gameplay-plugin-composition-evidence-manifests-nix-check-reconciled-final.run.log`.
- Packaged task-evidence validation is recorded in `docs/evidence/run-logs/2026-06-26/gameplay-plugin-composition-cairn-task-evidence-nix-check-final.run.log` and `docs/evidence/run-logs/2026-06-26/gameplay-plugin-composition-cairn-task-evidence-nix-check-reconciled-final.run.log`.
- Sync and archive evidence is recorded in `docs/evidence/run-logs/2026-06-26/gameplay-plugin-composition-sync-dry-run.run.log`, `docs/evidence/run-logs/2026-06-26/gameplay-plugin-composition-sync-execute.run.log`, `docs/evidence/run-logs/2026-06-26/gameplay-plugin-composition-post-sync-validate.run.log`, `docs/evidence/run-logs/2026-06-26/gameplay-plugin-composition-archive-dry-run.run.log`, `docs/evidence/run-logs/2026-06-26/gameplay-plugin-composition-archive-execute.run.log`, `docs/evidence/run-logs/2026-06-26/gameplay-plugin-composition-post-archive-validate.run.log`, and `docs/evidence/run-logs/2026-06-26/gameplay-plugin-composition-cairn-validate-reconciled-final.run.log`.
- BLAKE3 digests for promoted evidence are recorded in `docs/evidence/manifests/2026-06-26/gameplay-plugin-composition.b3`.

## Non-claims

This evidence supports the active Cairn tasks for Valence example plugin organization only. It does not claim broad Minecraft compatibility, semantic equivalence with Paper/vanilla, production public-server safety, or full correctness of the CTF/survival/terrain game modes.

## Owner

Valence gameplay example maintainers.

## Next action

Use the focused tests, Cairn gates, task-evidence gates, accepted-spec reconciliation, manifest refresh/checks, sync receipts, archive receipts, and post-archive validation above as the review evidence for the archived gameplay plugin composition changes.
