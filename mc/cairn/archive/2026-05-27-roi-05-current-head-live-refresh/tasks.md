# Tasks

- [x] [serial] Satisfy `representative_receipt`. r[mc_compatibility.roi_05_current_head_live_refresh.representative_receipt]
  - Evidence: current-head projectile rail live run passed and wrote `target/mc-compat-current-head-live-refresh/projectile-hit-current-head.json` with `status=pass`, `mode=run`, `dry_run=false`, `scenario.passed=true`, no missing client/server milestones, and `triage.suggested_boundary=none`.
  - Reviewable receipt copy: `docs/evidence/protocol-763-current-head-projectile-hit-2026-05-27.receipt.json`.
  - Reviewable run log copy: `docs/evidence/protocol-763-current-head-projectile-hit-2026-05-27.run.log`.
  - Command: `VALENCE_REV=HEAD VALENCE_WORKTREE=/tmp/valence-compat-current-head-projectile-refresh VALENCE_TARGET_DIR=/tmp/valence-compat-current-head-projectile-refresh-target CLIENT_TIMEOUT=300 MC_COMPAT_PROJECTILE_HIT_RECEIPT=target/mc-compat-current-head-live-refresh/projectile-hit-current-head.json nix run --no-update-lock-file .#mc-compat-valence-ctf-projectile-hit -- --run`.
  - BLAKE3: `756b6f732e71ae370808b2a653d1310baa88875f2c3345a1c87444fcffb51c6c  docs/evidence/protocol-763-current-head-projectile-hit-2026-05-27.receipt.json`.
  - Run log BLAKE3: `05429930472e764a6a2b140ce9c0a7652552659210b4bb1407d93d0d2cd7fada  docs/evidence/protocol-763-current-head-projectile-hit-2026-05-27.run.log`.
- [x] [serial] Satisfy `bundle_alignment`. r[mc_compatibility.roi_05_current_head_live_refresh.bundle_alignment]
  - Evidence: `docs/evidence/protocol-763-current-evidence-bundle.md` now records the representative current-head receipt, reviewable receipt/log copies, maintained command, source run log path, payload commits (`a2dddea`, Valence `e5d18ad`, Stevenarella `616ee72`), BLAKE3, outcome, and scoped non-claims without moving the 11 historical matrix rows.
- [x] [serial] Satisfy `verification`. r[mc_compatibility.roi_05_current_head_live_refresh.verification]
  - Live verification passed: pueue task `30` (`roi05 live current-head projectile`) completed successfully.
  - Deterministic index checks passed: `python3 tools/check_acceptance_matrix.py`; `python3 tools/check_current_evidence_bundle.py`.
  - Dry-run projectile check passed: pueue task `31` (`python3 tools/check_acceptance_matrix.py && python3 tools/check_current_evidence_bundle.py && nix build .#checks.x86_64-linux.mc-compat-valence-ctf-projectile-hit-dry-run --no-link -L --no-update-lock-file --option builders ''`).
