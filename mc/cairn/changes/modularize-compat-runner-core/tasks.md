# Tasks

- [ ] [serial] Document the runner functional-core / imperative-shell boundaries and identify which existing functions/constants move first. r[repository_layout.compat_runner_modularization.boundary]
- [ ] [depends:boundary] Move scenario definitions, milestone specs, forbidden-pattern specs, and behavior metadata out of `main.rs` without changing parsed scenario behavior. r[repository_layout.compat_runner_modularization.scenario_core]
- [ ] [depends:scenario_core] Move receipt/config validation logic into pure modules with positive and negative tests for valid receipts, missing fields, wrong types, malformed evidence, and broad overclaims. r[repository_layout.compat_runner_modularization.pure_validation]
- [ ] [depends:pure_validation] Reduce `main.rs` to CLI/orchestration responsibilities and remove core imports that depend on shell-owned constants. r[repository_layout.compat_runner_modularization.dependency_direction]
- [ ] [depends:dependency_direction] Run focused runner tests, maintained dry-run checks, generated-surface freshness checks if touched, Cairn gates, and Cairn validation with reviewable logs under `docs/evidence/`. r[repository_layout.compat_runner_modularization.validation]
