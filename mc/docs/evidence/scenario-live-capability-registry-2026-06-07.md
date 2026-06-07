# Scenario live probe capability registry — 2026-06-07

## Scope

This evidence supports Cairn change `scenario-live-probe-capability-registry`. The change makes targeted packet live-promotion capability metadata explicit in pure runner scenario core data. It does not promote any targeted packet row by itself.

## Registry contract

`tools/mc-compat-runner/src/scenario_core.rs` now defines `ScenarioLiveCapability` entries with:

- scenario id;
- targeted packet row id;
- exact packet row identifiers;
- capability kind (`targeted-packet-live-probe` or `targeted-packet-live-blocker`);
- backend/client path labels;
- evidence mode;
- required live signals;
- required non-claims;
- optional blocker reason.

The static registry currently records blocker entries for all eight targeted packet rows, including the three live-rail candidates (`creative-inventory-action`, `resource-pack-status`, and `sign-editor-open-update`). Blocker entries are selection guidance only; they are not live-promotion evidence.

## Fail-closed validation

Pure scenario-core validation rejects:

- duplicate scenario/row capability pairs;
- unknown scenarios;
- unknown targeted packet rows;
- unsupported evidence modes;
- empty required signals;
- missing required non-claims;
- blocked capabilities without blocker reasons.

`tools/check_scenario_manifest.rs` also checks that the registry surface remains present in `scenario_core.rs`, so source drift is caught by the existing scenario manifest flake check.

## Future live-rail workflow

1. Add or update a `ScenarioLiveCapability` entry for the scenario/packet row pair.
2. Keep blocker entries as `fixture-bounded-blocker` until a deterministic owned-local live rail and receipt exist.
3. When a live rail lands, switch the capability kind and evidence mode only with row-specific KV/receipt/checker evidence.
4. Run scenario-core tests, scenario manifest checks, targeted packet checks if packet rows changed, evidence-manifest/task-evidence checks, and Cairn gates before archive.

## Validation evidence

- Baseline scenario manifest and scenario-core tests: `docs/evidence/scenario-live-capability-registry-baseline-2026-06-07.run.log` (`exit_status=0`).
- Registry implementation checks: `docs/evidence/scenario-live-capability-registry-checks-2026-06-07.run.log` (`exit_status=0`).
- Focused dry-run check: `docs/evidence/scenario-live-capability-registry-dry-run-checks-2026-06-07.run.log` (`exit_status=0`).
- Cairn gates: `docs/evidence/scenario-live-capability-registry-cairn-gates-2026-06-07.run.log` (`exit_status=0`).
- Evidence manifest refresh/checks and task-evidence gate: `docs/evidence/scenario-live-capability-registry-evidence-manifest-refresh-2026-06-07.run.log` and `docs/evidence/scenario-live-capability-registry-evidence-checks-2026-06-07.run.log` (`exit_status=0`).
- Sync/archive/post-archive validation: `docs/evidence/scenario-live-capability-registry-sync-2026-06-07.run.log`, `docs/evidence/scenario-live-capability-registry-post-sync-validate-2026-06-07.run.log`, `docs/evidence/scenario-live-capability-registry-archive-2026-06-07.run.log`, `docs/evidence/scenario-live-capability-registry-final-manifest-refresh-2026-06-07.run.log`, and `docs/evidence/scenario-live-capability-registry-post-archive-checks-2026-06-07.run.log` (`exit_status=0`).

## Non-claims

This registry does not claim live parity for any packet row, full protocol 763 compatibility, broad Minecraft compatibility, public-server safety, production readiness, or arbitrary gameplay semantics.
