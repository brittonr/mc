# Tasks

- [x] [serial] Satisfy `equipment_packet_observation`. r[mc_compatibility.roi_02_equipment_update_rail.equipment_packet_observation]
  - Evidence: live `target/mc-compat-equipment-update/equipment-update-observation.json` passed with Stevenarella milestone `equipment_probe_entity_equipment` from Valence `EntityEquipmentUpdateS2c`.
- [x] [serial] Satisfy `server_equipment_milestone`. r[mc_compatibility.roi_02_equipment_update_rail.server_equipment_milestone]
  - Evidence: Valence CTF emits `MC-COMPAT-MILESTONE equipment_update_state username=... slot=chest item=DiamondChestplate source=team_inventory_setup`; runner requires `server_equipment_update_state`.
- [x] [serial] Satisfy `receipt_and_gate`. r[mc_compatibility.roi_02_equipment_update_rail.receipt_and_gate]
  - Live receipt: `target/mc-compat-equipment-update/equipment-update-observation.json` => `status=pass`, `scenario.passed=true`, `mode=run`; BLAKE3 `fdc7217ced89b9d018a42ab46d72e7cf33e15906f937050d10580854298c309b`.
  - Dry-run receipt: `target/mc-compat-equipment-update/equipment-update-observation-dry-run.json` => `status=pass`, `scenario.passed=false`, `mode=dry-run`; BLAKE3 `a8b83683a03e2601b7a18e2fe3f5ac02ca535f866afe8bac9cce801967b74810`.
  - Deterministic gate: `nix build .#checks.x86_64-linux.mc-compat-valence-ctf-equipment-update-observation-dry-run --no-link -L --no-update-lock-file --option builders ''` passed.
  - Focused compile checks passed: `cargo check --manifest-path tools/mc-compat-runner/Cargo.toml`, `cargo check --example ctf`, and `cargo check --bin stevenarella`.
