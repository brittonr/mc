# Tasks

- [x] [serial] Satisfy `equipment_state`. r[mc_compatibility.roi_01_armor_equipment_mitigation.equipment_state]
- [x] [serial] Satisfy `mitigation_correlation`. r[mc_compatibility.roi_01_armor_equipment_mitigation.mitigation_correlation]
- [x] [serial] Satisfy `live_receipt`. r[mc_compatibility.roi_01_armor_equipment_mitigation.live_receipt]
- [x] [serial] Satisfy `dry_run_check`. r[mc_compatibility.roi_01_armor_equipment_mitigation.dry_run_check]

## Evidence

- Dry-run check passed: `nix build .#checks.x86_64-linux.mc-compat-valence-ctf-armor-equipment-mitigation-dry-run --no-link -L --no-update-lock-file --option builders ''`.
- Live Valence CTF receipt passed: `VALENCE_REV=HEAD VALENCE_WORKTREE=/tmp/valence-compat-armor-live VALENCE_TARGET_DIR=/tmp/valence-compat-armor-live-target CLIENT_TIMEOUT=180 nix run .#mc-compat-valence-ctf-armor-equipment-mitigation -- --run --receipt target/mc-compat-armor-mitigation/armor-equipment-mitigation.json`.
- Live receipt BLAKE3: `176fdf33d2b8b9047471f577a98f9093904a44ab8da2785baeb80acfc8d97765  target/mc-compat-armor-mitigation/armor-equipment-mitigation.json`.
- Dry-run receipt BLAKE3: `819b042dd37d52a54e0079d28e9535bcc553f0059f77756cb48d83d6705af6db  target/mc-compat-armor-mitigation/armor-equipment-mitigation-dry-run.json`.
- Focused compile checks passed: `cargo check --manifest-path tools/mc-compat-runner/Cargo.toml`, `cargo check --example ctf`, and `cargo check --bin stevenarella`.
