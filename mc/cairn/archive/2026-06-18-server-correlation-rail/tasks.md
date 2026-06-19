# Tasks

- [x] [serial] Record the server-correlation rail contract, supported rows, receipt fields, validation rules, and non-claims. r[mc_compatibility.server_correlation_rail.contract]
  - Evidence: `docs/evidence/server-correlation-rail-contract-2026-06-18.md`, `docs/evidence/server-correlation-rail-checks-2026-06-18.run.log`, and `docs/evidence/server-correlation-rail-2026-06-18.b3`.
- [x] [depends:contract] Run baseline targeted-packet, scenario-manifest, and Cairn gates before adding the checker. r[mc_compatibility.server_correlation_rail.baseline]
  - Evidence: `docs/evidence/server-correlation-rail-baseline-2026-06-18.run.log` and `docs/evidence/server-correlation-rail-2026-06-18.b3`.
- [x] [depends:baseline] Implement the pure receipt validator and thin CLI shell for owned-local server-correlation receipts. r[mc_compatibility.server_correlation_rail.checker]
  - Evidence: `docs/evidence/server-correlation-rail-checks-2026-06-18.run.log`, `tools/check_server_correlation_receipts.rs`, and `docs/evidence/server-correlation-rail-2026-06-18.b3`.
- [x] [depends:checker] Add positive and negative fixtures for resource-pack and sign-editor correlation, including blocked/missing/overclaim cases. r[mc_compatibility.server_correlation_rail.fixtures]
  - Evidence: `docs/evidence/server-correlation-resource-pack-fixture-2026-06-18.receipt.json`, `docs/evidence/server-correlation-sign-editor-fixture-2026-06-18.receipt.json`, `docs/evidence/server-correlation-rail-checks-2026-06-18.run.log`, and `docs/evidence/server-correlation-rail-2026-06-18.b3`.
- [x] [depends:fixtures] Wire the checker into flake checks without promoting any targeted packet row. r[mc_compatibility.server_correlation_rail.integration]
  - Evidence: `docs/evidence/server-correlation-rail-checks-2026-06-18.run.log`, `flake.nix`, and `docs/evidence/server-correlation-rail-2026-06-18.b3`.
- [x] [depends:integration] Emit reviewable evidence under `docs/evidence/` and validate manifests/task evidence. r[mc_compatibility.server_correlation_rail.evidence]
  - Evidence: `docs/evidence/server-correlation-rail-2026-06-18.kv`, `docs/evidence/server-correlation-rail-2026-06-18.receipt.json`, `docs/evidence/server-correlation-rail-evidence-2026-06-18.md`, `docs/evidence/server-correlation-rail-checks-2026-06-18.run.log`, and `docs/evidence/server-correlation-rail-2026-06-18.b3`.
- [x] [depends:evidence] Run Cairn sync/archive and post-archive validation. r[mc_compatibility.server_correlation_rail.validation]
  - Evidence: `docs/evidence/server-correlation-rail-closeout-2026-06-18.run.log`, `docs/evidence/server-correlation-rail-task-evidence-2026-06-18.run.log`, `docs/evidence/server-correlation-rail-sync-2026-06-18.run.log`, `docs/evidence/server-correlation-rail-pre-archive-2026-06-18.run.log`, `docs/evidence/server-correlation-rail-archive-2026-06-18.run.log`, `docs/evidence/server-correlation-rail-post-archive-2026-06-18.run.log`, and `docs/evidence/server-correlation-rail-2026-06-18.b3`.
