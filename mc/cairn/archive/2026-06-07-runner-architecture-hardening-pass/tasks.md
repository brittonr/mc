# Tasks

- [x] [serial] Select and document one bounded runner/checker hardening seam, with public outputs and non-claims named before code changes. r[mc_compatibility.runner_architecture_hardening.contract]
  - Evidence: `docs/evidence/runner-architecture-hardening-contract-2026-06-07.md` and `docs/evidence/runner-architecture-hardening-2026-06-07.b3`.
- [x] [depends:contract] Run baseline focused runner/checker tests and dry-runs for the selected seam. r[mc_compatibility.runner_architecture_hardening.baseline]
  - Evidence: `docs/evidence/runner-architecture-hardening-baseline-2026-06-07.run.log` and `docs/evidence/runner-architecture-hardening-2026-06-07.b3`.
- [x] [depends:baseline] Extract the selected seam into pure deterministic core logic and a thin imperative shell without changing public output. r[mc_compatibility.runner_architecture_hardening.core]
  - Evidence: `tools/check_scenario_manifest.rs`, `docs/evidence/runner-architecture-hardening-evidence-2026-06-07.md`, and `docs/evidence/runner-architecture-hardening-2026-06-07.b3`.
- [x] [depends:core] Add positive parity tests and negative malformed, unknown, missing-evidence, stale-revision, and overclaim fixtures for the selected seam. r[mc_compatibility.runner_architecture_hardening.tests]
  - Evidence: `docs/evidence/runner-architecture-hardening-tests-2026-06-07.run.log`, `docs/evidence/runner-architecture-hardening-postfmt-tests-2026-06-07.run.log`, `docs/evidence/runner-architecture-hardening-negative-fixtures-2026-06-07.run.log`, and `docs/evidence/runner-architecture-hardening-2026-06-07.b3`.
- [x] [depends:tests] Record reviewable architecture-hardening evidence under `docs/evidence/` without adding compatibility coverage. r[mc_compatibility.runner_architecture_hardening.evidence]
  - Evidence: `docs/evidence/runner-architecture-hardening-evidence-2026-06-07.md` and `docs/evidence/runner-architecture-hardening-2026-06-07.b3`.
- [x] [depends:evidence] Run relevant runner/checker tests, evidence-manifest/task-evidence checks, Cairn gates, sync/archive checks, and post-archive validation. r[mc_compatibility.runner_architecture_hardening.validation]
  - Evidence: `docs/evidence/runner-architecture-hardening-closeout-2026-06-07.run.log` and `docs/evidence/runner-architecture-hardening-2026-06-07.b3`.
