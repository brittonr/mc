# Tasks

- [x] [serial] Satisfy `dependency_checkpoint`. r[mc_compatibility.roi_10_pinned_projectile_damage_proof.dependency_checkpoint]
- [x] [serial] Satisfy `causal_runner_gate`. r[mc_compatibility.roi_10_pinned_projectile_damage_proof.causal_runner_gate]
- [x] [serial] Satisfy `dry_run_evidence`. r[mc_compatibility.roi_10_pinned_projectile_damage_proof.dry_run_evidence]
- [x] [serial] Satisfy `live_receipt_and_promotion`. r[mc_compatibility.roi_10_pinned_projectile_damage_proof.live_receipt_and_promotion]

## Progress

- Dependency checkpoint recorded at `docs/evidence/protocol-763-roi-10-projectile-damage-pin-checkpoint-2026-05-27.md`.
- Checkpoint BLAKE3 recorded at `docs/evidence/protocol-763-roi-10-projectile-damage-pin-checkpoint-2026-05-27.b3`.
- Selected dependency strategy: use clean local nested `valence/` at exact commit `e5d18ad04010d92881267ac1ea43922ae91821f5`, never `HEAD`, for the next live proof.
- Runner gate now rejects projectile damage attribution unless `VALENCE_REV` is the pinned commit and `projectile_damage_causality` observes attacker client use/swing, server projectile use/hit for the same attacker/victim pair, and victim client post-hit health update with no order violations.
- Runner tests passed: `cargo test --manifest-path tools/mc-compat-runner/Cargo.toml` with `32 passed`.
- Dry-run gate passed: `nix build .#checks.x86_64-linux.mc-compat-valence-ctf-projectile-damage-attribution-dry-run --no-link -L --no-update-lock-file --option builders ''`.
- Live pinned receipt passed: `docs/evidence/protocol-763-roi-10-projectile-damage-pinned-2026-05-27.receipt.json`; BLAKE3 `cf84fcb81ae557ecfbd2ff0b1f8b94af7bf07eaa85c20b1cde442929e3e3e529`.
- Reviewable logs copied under `docs/evidence/`: run log, Valence log, both client logs, dry-run log/receipt, summary, and `.b3` manifest.
- Matrix, current bundle, residual catalog, and local checkers re-promote projectile damage attribution as a bounded maintained row with explicit non-claims.

## Review correction

- Same-family review WARNed that the dependency checkpoint content was outside the supplied review paths.
- Added review evidence: `docs/evidence/protocol-763-roi-10-dependency-checkpoint-review-2026-05-27.md`.
- Added verification log: `docs/evidence/protocol-763-roi-10-dependency-checkpoint-review-2026-05-27.run.log`.
- Added BLAKE3 manifest: `docs/evidence/protocol-763-roi-10-dependency-checkpoint-review-2026-05-27.b3`.
- Verification result: checkpoint sidecar validates, required checkpoint fields are present, nested `valence/` is clean at `e5d18ad04010d92881267ac1ea43922ae91821f5`, and instrumentation markers remain present.
