# Tasks

- [x] [serial] Satisfy `survival_fixture`. r[mc_compatibility.survival_break_place_pickup.survival_fixture]
  - Evidence: Added dedicated Valence `examples/survival_compat.rs` fixture and Nix app `mc-compat-valence-survival-break-place-pickup`; live receipt records `valence.example="survival_compat"` and owned-local protocol-763 target.
- [x] [serial] Satisfy `client_probe`. r[mc_compatibility.survival_break_place_pickup.client_probe]
  - Evidence: Stevenarella `MC_COMPAT_SURVIVAL_PROBE` emits fixed-coordinate survival milestones. Live receipt `docs/evidence/protocol-763-survival-break-place-pickup-2026-05-28.receipt.json` observed `protocol_detected`, `join_game`, `render_tick`, `survival_break_sent`, `survival_break_update`, `survival_pickup_seen`, `survival_place_sent`, and `survival_place_update` with no forbidden matches.
- [x] [serial] Satisfy `server_correlation`. r[mc_compatibility.survival_break_place_pickup.server_correlation]
  - Evidence: Valence log `docs/evidence/protocol-763-survival-break-place-pickup-2026-05-28.valence.log` and receipt server block observed `server_username_seen`, `server_survival_join`, `server_survival_break`, `server_survival_pickup`, and `server_survival_place` for `compatbot`; `client_server_correlation.passed=true`.
- [x] [serial] Satisfy `receipt_nonclaims`. r[mc_compatibility.survival_break_place_pickup.receipt_nonclaims]
  - Evidence: Evidence doc `docs/evidence/protocol-763-survival-break-place-pickup-2026-05-28.md` records scoped non-claims; receipt keeps `claims_correctness=false`, `claims_semantic_equivalence=false`, `claims_public_server_safety=false`, and `claims_production_readiness=false`.
- [x] [serial] Satisfy `dry_run_check`. r[mc_compatibility.survival_break_place_pickup.dry_run_check]
  - Evidence: `nix build --no-update-lock-file .#checks.x86_64-linux.mc-compat-valence-survival-break-place-pickup-dry-run --no-link -L` passed and checks scenario name, `survival_compat` fixture selection, required client/server milestone names, expected packet summary, and non-claims.
