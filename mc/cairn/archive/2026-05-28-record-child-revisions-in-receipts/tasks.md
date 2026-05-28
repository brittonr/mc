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
- [x] [depends:verified] Refresh one live survival receipt and compare it against the existing oracle checkpoint. r[mc_compatibility.receipts.child_revisions.verified]
  - Evidence: Live survival rail pueue task 61 passed using parent `455fa5c`, Valence `1fac05a`, and Stevenarella `9921e68`. Refreshed receipt `docs/evidence/protocol-763-survival-break-place-pickup-2026-05-28.receipt.json` records `client.git_rev=9921e686f56270cb5810c1f6187d19b051ecc236`, `client.git_status=clean`, `valence.git_rev_requested=1fac05a`, `valence.git_rev_resolved=1fac05a6d012f27b83d88d83c59e5ab320a78164`, and `valence.git_status=clean`, superseding the oracle-only proof while retaining the oracle checkpoint for review history.
