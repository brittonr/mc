# Tasks

- [ ] [serial] Add child revision fields to the receipt schema and JSON output. r[mc_compatibility.receipts.child_revisions.recorded]
- [ ] [serial] Add git probing helpers in the runner I/O shell for client and Valence worktrees. r[mc_compatibility.receipts.child_revisions.recorded]
- [ ] [serial] Add positive and negative unit tests for clean, dirty, and unavailable child revisions. r[mc_compatibility.receipts.child_revisions.verified]
- [ ] [depends:recorded] Update dry-run receipts with deterministic placeholder child revision fields. r[mc_compatibility.receipts.child_revisions.dry_run]
- [ ] [depends:recorded] Update evidence checks to reject promoted non-legacy receipts that lack machine-recorded child revisions. r[mc_compatibility.receipts.child_revisions.gated]
- [ ] [depends:verified] Refresh one live survival receipt and compare it against the existing oracle checkpoint. r[mc_compatibility.receipts.child_revisions.verified]
