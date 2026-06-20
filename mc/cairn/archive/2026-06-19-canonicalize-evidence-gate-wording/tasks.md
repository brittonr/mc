# Tasks

- [x] [serial] Run focused baseline evidence gates before checker/doc changes. r[mc_compatibility.canonical_evidence_gate_wording.validation]
  - Evidence: `docs/evidence/canonical-evidence-gate-wording-baseline-2026-06-19.run.log` and `docs/evidence/canonical-evidence-gate-wording-2026-06-19.b3`.
- [x] [depends:validation] Replace stale WAN and CTF compatibility-alias tokens with canonical non-claim labels. r[mc_compatibility.canonical_evidence_gate_wording.catalog]
  - Evidence: `docs/evidence/canonical-evidence-gate-wording-focused-2026-06-19.run.log`, `docs/evidence/protocol-763-current-evidence-bundle.md`, `docs/evidence/protocol-763-ctf-rule-ledger-2026-05-27.md`, and `docs/evidence/canonical-evidence-gate-wording-2026-06-19.b3`.
- [x] [depends:catalog] Name canonical checker tokens and keep row-count expectations derived from maintained row inventories. r[mc_compatibility.canonical_evidence_gate_wording.checker_constants] r[mc_compatibility.canonical_evidence_gate_wording.row_inventory]
  - Evidence: `docs/evidence/canonical-evidence-gate-wording-focused-2026-06-19.run.log`, `tools/check_wan_tolerance_bounded_telemetry.rs`, `tools/check_ctf_invalid_pickup_ownership.rs`, `tools/check_ctf_invalid_return_drop.rs`, `tools/check_full_survival_compatibility_gate.rs`, and `docs/evidence/canonical-evidence-gate-wording-2026-06-19.b3`.
- [x] [depends:checker_constants] Refresh BLAKE3 manifests and run focused row/aggregate evidence gates. r[mc_compatibility.canonical_evidence_gate_wording.validation]
  - Evidence: `docs/evidence/canonical-evidence-gate-wording-focused-2026-06-19.run.log` and `docs/evidence/canonical-evidence-gate-wording-2026-06-19.b3`.
- [x] [depends:validation] Run Cairn proposal/design/tasks gates, sync/archive checks, and final validation. r[mc_compatibility.canonical_evidence_gate_wording.validation]
  - Evidence: `docs/evidence/canonical-evidence-gate-wording-cairn-gates-2026-06-19.run.log`, `docs/evidence/canonical-evidence-gate-wording-sync-2026-06-19.run.log`, `docs/evidence/canonical-evidence-gate-wording-archive-2026-06-19.run.log`, `docs/evidence/canonical-evidence-gate-wording-post-archive-2026-06-19.run.log`, and `docs/evidence/canonical-evidence-gate-wording-2026-06-19.b3`.
