# Tasks

- [x] [serial] Satisfy `projectile_scenario`. r[mc_compatibility.roi_03_projectile_hit_rail.projectile_scenario]
  - Evidence: `nix run .#mc-compat-valence-ctf-projectile-hit -- --run` wrote `target/mc-compat-projectile-hit/projectile-hit.json` with `status=pass`, `mode=run`, `scenario.passed=true`, and observed `multi_client_count`, protocol/join/render, red team selection, remote-player spawn, `projectile_probe_use_item_sent`, and `projectile_probe_swing_sent`.
- [x] [serial] Satisfy `hit_attribution`. r[mc_compatibility.roi_03_projectile_hit_rail.hit_attribution]
  - Evidence: Stevenarella drove named two-client probe users `compatbota`/`compatbotb`; Valence CTF emitted server projectile loadout evidence; runner receipt records packet summary `two_client_login`, `play_join_game`, `projectile_use_item`, `projectile_hit_attribution`.
- [x] [serial] Satisfy `receipt_nonclaims`. r[mc_compatibility.roi_03_projectile_hit_rail.receipt_nonclaims]
  - Evidence: live receipt BLAKE3 `22310a0373f86bbff5e6bc116934d092b89f775cf5d539b08d04ff5564ad855b`; dry-run receipt BLAKE3 `50d0709a192435a7efc3ade64abd4d06f01b31dccedaf9cda35439ced114ae0b`; receipts keep correctness/semantic-equivalence claims false.
- [x] [serial] Satisfy `dry_run_check`. r[mc_compatibility.roi_03_projectile_hit_rail.dry_run_check]
  - Evidence: `nix build .#checks.x86_64-linux.mc-compat-valence-ctf-projectile-hit-dry-run --no-link -L --no-update-lock-file --option builders ''` passed; local dry-run receipt `target/mc-compat-projectile-hit/projectile-hit-dry-run.json` passed with `mode=dry-run`.
