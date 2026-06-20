# Tasks

- [x] [serial] Record the dry-run coverage contract, waiver shape, maintained-status semantics, and explicit non-claim boundary. r[mc_compatibility.harness_dry_run_coverage.contract]
  Evidence: docs/evidence/harness-dry-run-coverage-2026-06-20.run.log; docs/evidence/harness-dry-run-coverage-2026-06-20.b3
- [x] [depends:contract] Add a pure manifest coverage evaluator plus positive and negative fixtures for covered rows, waiver-backed rows, missing wrappers, empty waiver reasons, and stale exclusion metadata. r[mc_compatibility.harness_dry_run_coverage.checker]
  Evidence: docs/evidence/harness-dry-run-coverage-2026-06-20.run.log; docs/evidence/harness-dry-run-coverage-2026-06-20.b3
- [x] [depends:checker] Convert eligible maintained exclusions into deterministic dry-run receipt-shape wrappers and Nix checks without changing scenario names or live evidence claims. r[mc_compatibility.harness_dry_run_coverage.wrappers]
  Evidence: docs/evidence/harness-dry-run-coverage-2026-06-20.run.log; docs/evidence/harness-dry-run-coverage-2026-06-20.b3
- [x] [depends:wrappers] Update README and current evidence bundle wording so dry-run shape evidence and live/reference evidence remain separate. r[mc_compatibility.harness_dry_run_coverage.docs]
  Evidence: docs/evidence/harness-dry-run-coverage-2026-06-20.run.log; docs/evidence/harness-dry-run-coverage-2026-06-20.b3
- [x] [depends:docs] Run focused runner tests, scenario-manifest checks, affected dry-run checks, maintained dry-run aggregate, evidence manifest checks, Cairn gates, and Cairn validation with reviewable logs. r[mc_compatibility.harness_dry_run_coverage.validation]
  Evidence: docs/evidence/harness-dry-run-coverage-2026-06-20.run.log; docs/evidence/harness-dry-run-coverage-maintained-aggregate-2026-06-20.run.log; docs/evidence/harness-dry-run-coverage-2026-06-20.b3
