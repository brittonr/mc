# Tasks

- [x] [serial] Satisfy `feasibility_checkpoint`. r[mc_compatibility.roi_08_projectile_damage_attribution.feasibility_checkpoint]
- [x] [serial] Satisfy `dry_run_gate`. r[mc_compatibility.roi_08_projectile_damage_attribution.dry_run_gate]
- [x] [serial] Satisfy `live_receipt_or_nonclaim`. r[mc_compatibility.roi_08_projectile_damage_attribution.live_receipt_or_nonclaim]

## Evidence

- Baseline runner tests passed before core scenario changes: `nix develop --no-update-lock-file -c cargo test --manifest-path tools/mc-compat-runner/Cargo.toml`.
- Feasibility checkpoint: `docs/evidence/protocol-763-roi-08-projectile-damage-feasibility-2026-05-27.md`; BLAKE3 `f635054c7d47087db102a676000fd292f5f8df254e66a7b68d8e34e1a5591417`.
- Dry-run gate passed: `nix build .#checks.x86_64-linux.mc-compat-valence-ctf-projectile-damage-attribution-dry-run --no-link -L --no-update-lock-file --option builders ''`.
- Reviewable dry-run receipt: `docs/evidence/protocol-763-roi-08-projectile-damage-attribution-2026-05-27.dry-run.receipt.json`; BLAKE3 `631c3cd90500690817ea17098adda4dce378692a6c3d4d85f24865eb5000352d`.
- Live Valence CTF receipt passed: `MC_COMPAT_PROJECTILE_DAMAGE_RECEIPT=target/mc-compat-projectile-damage-attribution/projectile-damage-attribution.json VALENCE_REV=HEAD VALENCE_WORKTREE=/tmp/valence-compat-projectile-damage VALENCE_TARGET_DIR=/tmp/valence-compat-projectile-damage-target CLIENT_TIMEOUT=300 nix run --no-update-lock-file .#mc-compat-valence-ctf-projectile-damage-attribution -- --run`.
- Reviewable live receipt: `docs/evidence/protocol-763-roi-08-projectile-damage-attribution-2026-05-27.receipt.json`; BLAKE3 `39b085d43b09c6392e19b0cc74b7d8192d8bf34b4c5351514ad0b94d0d07c603`.
- Reviewable live run log: `docs/evidence/protocol-763-roi-08-projectile-damage-attribution-2026-05-27.run.log`; BLAKE3 `c6f0cb7b713b43073a927bf43dbd9e5370c1fdb7d30531c0a3765bfccd5adcd3`.
- Evidence summary: `docs/evidence/protocol-763-roi-08-projectile-damage-attribution-2026-05-27.md`; BLAKE3 `71f2a5c8803fd7b329c4e870929e16145a913efcb554f4844be15f782e2c143f`.
- Post-change runner tests passed: `nix develop --no-update-lock-file -c cargo test --manifest-path tools/mc-compat-runner/Cargo.toml`.
- Maintained dry-run aggregate passed: `nix build .#checks.x86_64-linux.mc-compat-maintained-dry-runs --no-link -L --no-update-lock-file --option builders ''`.

## Review correction

The live receipt claim was demoted by `cairn/archive/2026-05-27-roi-09-demote-projectile-damage-claim`. Same-family review found that `VALENCE_REV=HEAD` made the server-side projectile hit dependency insufficiently pinned/reviewable and that the receipt only proved milestone presence, not causal ordering between projectile action and client damage update. Treat the ROI 08 artifacts as experimental audit evidence until a fresh pinned/causal proof is produced.
