# Tasks

- [x] [serial] Add child revision fields to the receipt schema and JSON output. r[mc_compatibility.receipts.child_revisions.recorded]
  - Evidence: `tools/mc-compat-runner/src/main.rs` now emits `client.git_rev`, `client.git_status`, `client.git_dirty`, `client.git_diagnostics`, plus Valence requested/resolved revision, status, dirty, and diagnostics fields in scenario receipts.
- [x] [serial] Add git probing helpers in the runner I/O shell for client and Valence worktrees. r[mc_compatibility.receipts.child_revisions.recorded]
  - Evidence: Runner git I/O helpers resolve `HEAD` and `git status --porcelain` for the client checkout and Valence worktree; receipt construction consumes pure `GitRevisionEvidence` values.
- [x] [serial] Add positive and negative unit tests for clean, dirty, and unavailable child revisions. r[mc_compatibility.receipts.child_revisions.verified]
  - Evidence: `cargo test --manifest-path tools/mc-compat-runner/Cargo.toml` passed with `git_revision_evidence_core_reports_clean_dirty_and_unavailable` and dry-run receipt coverage.
- [x] [depends:recorded] Update dry-run receipts with deterministic placeholder child revision fields. r[mc_compatibility.receipts.child_revisions.dry_run]
  - Evidence: `dry_run_receipt_records_deterministic_child_revision_placeholders` asserts `dry-run` revision/status placeholders and `git_dirty=false`; the survival dry-run Nix check passed after the receipt shape change.
- [x] [depends:recorded] Update evidence checks to reject promoted non-legacy receipts that lack machine-recorded child revisions. r[mc_compatibility.receipts.child_revisions.gated]
  - Evidence: `tools/check_acceptance_matrix.py --self-test` covers missing child fields rejection, machine field acceptance, and oracle fallback; `tools/check_acceptance_matrix.py` passes for current legacy/oracle evidence.
- [ ] [depends:verified] Refresh one live survival receipt and compare it against the existing oracle checkpoint. r[mc_compatibility.receipts.child_revisions.verified]
