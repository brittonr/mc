# Modularize runner entrypoint responsibility map

## Question

What responsibilities did the runner entrypoint own before extraction, which owner paths now receive them, and what behavior must stay unchanged?

## Inspected evidence

- Baseline gate logs:
  - `docs/evidence/run-logs/2026-06-29/modularize-runner-entrypoint.preflight-gate-proposal.run.log`
  - `docs/evidence/run-logs/2026-06-29/modularize-runner-entrypoint.preflight-gate-design.run.log`
  - `docs/evidence/run-logs/2026-06-29/modularize-runner-entrypoint.preflight-gate-tasks.run.log`
  - `docs/evidence/run-logs/2026-06-29/modularize-runner-entrypoint.preflight-validate.run.log`
- Runner baseline log before entrypoint extraction: `docs/evidence/run-logs/2026-06-29/modularize-runner-entrypoint.baseline-runner-cargo-test.run.log` (`207 passed`, `exit_status=0`).
- Post-extraction boundary log: `docs/evidence/run-logs/2026-06-29/modularize-runner-entrypoint.post-boundary-test-runner-cargo-test.run.log` (`209 passed`, `exit_status=0`).
- Source paths after extraction:
  - `compat/runner/src/main.rs`: binary entrypoint only; delegates to `mc_compat_runner::run_main()`.
  - `compat/runner/src/lib.rs`: application shell and module wiring for the runner library.
  - `compat/runner/src/entrypoint_boundary.rs`: pure positive/negative source-boundary test for the thin entrypoint contract.

## Responsibility map

| Responsibility family | Pre-extraction owner | Post-extraction owner | Verification |
| --- | --- | --- | --- |
| Process exit translation | `compat/runner/src/main.rs` | `compat/runner/src/lib.rs::run_main` | Boundary test plus runner cargo test log. |
| CLI/config construction and precedence | `compat/runner/src/main.rs` | `compat/runner/src/lib.rs` application shell with `config_patches.rs` and `runtime_config.rs` cores | Existing config positive/negative tests in runner cargo logs. |
| Backend runtime dispatch and lifecycle shell | `compat/runner/src/main.rs` plus `backend_shell.rs` | `compat/runner/src/lib.rs` dispatch through `backend_shell.rs` runtime shell | Existing backend runtime and cleanup positive/negative tests in runner cargo logs. |
| Mode dispatch | `compat/runner/src/main.rs` | `compat/runner/src/lib.rs` application shell | Existing dry-run, run, build-client, status, cleanup, matrix, and compare tests in runner cargo logs. |
| Planning data and diagnostics | `compat/runner/src/main.rs` plus `planning.rs` | `compat/runner/src/lib.rs` data types with `planning.rs` pure planner | Existing planning positive/negative tests in runner cargo logs. |
| Receipt and typed-event artifact shell | `compat/runner/src/main.rs` plus `receipts.rs` | `compat/runner/src/lib.rs` file shell with `receipts.rs`, `evidence_receipts.rs`, and `evidence_types.rs` cores | Existing receipt schema, child-revision, typed-event, and non-claim tests in runner cargo logs. |
| Failure-bundle shell | `compat/runner/src/main.rs` plus `evidence_bundle.rs` | `compat/runner/src/lib.rs` artifact/hash/file shell with `evidence_bundle.rs` core | Existing failure-bundle positive/negative tests in runner cargo logs. |
| Scenario behavior adapters and env patching | `compat/runner/src/main.rs` plus `scenario_core.rs`/`scenario_catalog.rs` | `compat/runner/src/lib.rs` adapters with `scenario_core.rs`/`scenario_catalog.rs` pure definitions | Existing scenario metadata, generated-manifest, env-patch, and scenario oracle tests in runner cargo logs. |

## Decision

The binary entrypoint is now deliberately boring: it delegates to the runner library and owns no CLI, environment, filesystem, backend, scenario, receipt, or failure-bundle policy. The application shell still preserves the existing public CLI, environment variables, receipt schemas, scenario semantics, dry-run/live behavior, and non-claim boundaries. This change does not promote new Minecraft compatibility evidence.

## Owner

mc-compat runner.

## Next action

Run dry-run smoke, generated scenario checks, Cairn gates, task-evidence validation, sync, archive, and final validation before closing the change.
