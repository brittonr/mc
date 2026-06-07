# Runner scenario modules evidence — 2026-06-06

## Scope

This evidence supports Cairn change `split-mc-compat-runner-scenario-modules`. The change is structural: it splits scenario identity/spec/validation logic out of `tools/mc-compat-runner/src/main.rs` without adding new compatibility claims.

## Implementation notes

- `tools/mc-compat-runner/src/scenario_core.rs` now owns the pure scenario core: `Scenario`, `ScenarioSpec`, `ProbeTeam`, `ScenarioRunStrategy`, `ScenarioBehaviorKind`, `NegativeLiveRailBehavior`, static `SCENARIO_SPECS`/`ALL_SCENARIOS`, lookup helpers, and static spec validation.
- `tools/mc-compat-runner/src/main.rs` remains the imperative shell for CLI parsing, backend/client orchestration, environment mutation, log collection, correlation, and receipt writing. Its shell-side `ScenarioBehavior` trait applies process/env hooks from the pure `ScenarioBehaviorKind` values.
- `tools/check_scenario_manifest.rs` now validates runner surfaces across both `main.rs` and `scenario_core.rs`, with self-tests proving split-surface acceptance and missing-surface rejection.

## Positive and negative tests

`scenario_core::tests::scenario_core_validates_static_specs_and_lookup_parity` proves valid static specs preserve parse/name/milestone/forbidden-pattern/behavior lookup parity for every scenario.

`scenario_core::tests::scenario_core_rejects_invalid_static_specs` exercises fail-closed invalid definitions for:

- missing canonical alias / manifest alias drift;
- duplicated canonical names;
- missing client milestones;
- missing forbidden patterns;
- unsupported/default behavior for projectile damage attribution.

The scenario-manifest checker self-test additionally proves runner surface validation accepts strings split across `main.rs` and `scenario_core.rs` and rejects an incomplete split surface.

## Validation receipts

- Baseline before split: `docs/evidence/runner-scenario-modules-baseline-2026-06-06.run.log`.
- Runner tests after split: `docs/evidence/runner-scenario-modules-runner-tests-2026-06-06.run.log` (`118 passed`, `exit_status=0`).
- Scenario manifest check after split: `docs/evidence/runner-scenario-modules-scenario-manifest-2026-06-06.run.log` (`exit_status=0`).
- Focused dry-run checks after split: `docs/evidence/runner-scenario-modules-dry-run-checks-2026-06-06.run.log` (`exit_status=0`).
- Cairn gates/validation before sync: `docs/evidence/runner-scenario-modules-cairn-gates-2026-06-06.run.log` and `docs/evidence/runner-scenario-modules-cairn-validate-2026-06-06.run.log` (`exit_status=0`).
- Accepted spec sync/post-sync validation: `docs/evidence/runner-scenario-modules-sync-2026-06-06.run.log` and `docs/evidence/runner-scenario-modules-post-sync-validate-2026-06-06.run.log` (`exit_status=0`).
- Final active-task citation check: `docs/evidence/runner-scenario-modules-task-evidence-final-2026-06-06.run.log` (`exit_status=0`).
- Evidence manifest refresh/check: `docs/evidence/runner-scenario-modules-evidence-manifest-refresh-2026-06-06.run.log` and `docs/evidence/runner-scenario-modules-evidence-manifest-check-2026-06-06.run.log` (`exit_status=0`).
- Archive: `docs/evidence/runner-scenario-modules-archive-2026-06-06.run.log` (`exit_status=0`).

## Non-claims

This evidence does not claim new gameplay behavior, new live Paper/Valence parity, production readiness, broader protocol coverage, or public-server safety. Existing compatibility and evidence surfaces are intended to remain unchanged by this refactor.
