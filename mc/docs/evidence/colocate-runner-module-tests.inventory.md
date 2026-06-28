# Colocate runner module tests inventory

## Scope

This change moves the former `compat/runner/src/main.rs` root test module into owner-local colocated test files. It does not change runner behavior or promote new Minecraft compatibility, semantic-equivalence, production-readiness, public-server-safety, full CTF, or full survival correctness claims.

## Baseline and final counts

- Baseline runner suite: `docs/evidence/colocate-runner-module-tests.runner-tests.baseline.run.log` (`191 passed`).
- Final runner suite: `docs/evidence/colocate-runner-module-tests.runner-tests.final.run.log` (`191 passed`).
- Root entrypoint after the move declares shared test support, config tests, and an empty explicit integration boundary only; unrelated owner-unit test families live beside their modules.

## Owner inventory

| Owner family | Colocated file | Coverage retained |
| --- | --- | --- |
| Config / CLI parsing | `compat/runner/src/config_colocated_tests.rs` | defaults, CLI/env precedence, config files, invalid backend, scenario parsing, client checkout preflight |
| Planning / scenario router | `compat/runner/src/planning_colocated_tests.rs` | typed route parsing, alias plan parity, deterministic plan positives, fail-before-side-effect negatives |
| Wire protocol helpers | `compat/runner/src/wire_colocated_tests.rs` | varint/string/packet happy paths and malformed/truncated input failures |
| Backend shell / layout-adjacent runner shell | `compat/runner/src/backend_shell_colocated_tests.rs` | git revision evidence, Valence source resolution, cleanup dry-run/apply, missing Valence diagnostic |
| Evidence bundle | `compat/runner/src/evidence_bundle_colocated_tests.rs` | valid failure bundle and malformed/unreviewable bundle negatives |
| Receipt validation | `compat/runner/src/receipt_validation_colocated_tests.rs` | matching Paper/Valence receipts, protocol mismatch, malformed summary mutations |
| Receipt rendering | `compat/runner/src/receipts_colocated_tests.rs` | dry-run child revisions, paired reference shape non-claims, typed-event/MCP/frame/triage/negative-rail receipt positives and malformed/overclaim negatives |
| Scenario catalog/core | `compat/runner/src/scenario_core_colocated_tests.rs` | generated manifest parser parity and invalid static scenario definitions |
| Evidence receipts | `compat/runner/src/evidence_receipts_colocated_tests.rs` | latency/WAN/public-safety/negative-live-rail/load-network safety positives and fail-closed inputs |
| Client driver | `compat/runner/src/client_driver_colocated_tests.rs` | MCP preflight/live evidence, multi-client planning, projectile dry-run/travel/damage dependency coverage |
| Evidence core / scenario oracles | `compat/runner/src/evidence_core_colocated_tests.rs` | matcher positives/negatives, typed-event graph positives/negatives, CTF/survival/combat/inventory scenario oracle positives and fail-closed fixtures |
| Cross-module runner boundary | `compat/runner/src/runner_integration_tests.rs` | failure-bundle shell writes reviewable artifacts and runner result merging preserves the original failure when follow-up evidence writing fails |

## Shared test support

`compat/runner/src/test_support.rs` holds deterministic helpers shared by multiple owner modules: explicit-argument config construction, temporary checkout/git fixtures, shared test identities, and typed-event fixture builders. Helpers derive outputs from explicit inputs or per-test temp paths and do not mutate global process environment.

## Validation evidence

- Cairn baseline gates/validation: `docs/evidence/colocate-runner-module-tests.gate-proposal.baseline.run.log`, `docs/evidence/colocate-runner-module-tests.gate-design.baseline.run.log`, `docs/evidence/colocate-runner-module-tests.gate-tasks.baseline.run.log`, `docs/evidence/colocate-runner-module-tests.cairn-validate.baseline.run.log`.
- Runner formatting/tests: `docs/evidence/colocate-runner-module-tests.runner-fmt-check.run.log`, `docs/evidence/colocate-runner-module-tests.runner-tests.final.run.log`.
- Integration dry-runs: `docs/evidence/colocate-runner-module-tests.integration-smoke-dry-run.run.log`, `docs/evidence/colocate-runner-module-tests.maintained-dry-runs.final.run.log`.
- Evidence manifests: `docs/evidence/colocate-runner-module-tests.evidence-manifest-refresh.run.log`, `docs/evidence/colocate-runner-module-tests.evidence-manifest-check.run.log`.
