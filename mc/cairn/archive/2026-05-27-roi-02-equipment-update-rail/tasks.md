# Tasks

- [x] [serial] Satisfy `equipment_packet_observation`. r[mc_compatibility.roi_02_equipment_update_rail.equipment_packet_observation]
  - Evidence: live `target/mc-compat-equipment-update/equipment-update-observation.json` passed with Stevenarella milestone `equipment_probe_entity_equipment` from Valence `EntityEquipmentUpdateS2c`.
- [x] [serial] Satisfy `server_equipment_milestone`. r[mc_compatibility.roi_02_equipment_update_rail.server_equipment_milestone]
  - Evidence: Valence CTF emits `MC-COMPAT-MILESTONE equipment_update_state username=... slot=chest item=DiamondChestplate source=team_inventory_setup`; runner requires `server_equipment_update_state`.
- [x] [serial] Satisfy `receipt_and_gate`. r[mc_compatibility.roi_02_equipment_update_rail.receipt_and_gate]
  - Historical live receipt: `target/mc-compat-equipment-update/equipment-update-observation.json` => `status=pass`, `scenario.passed=true`, `mode=run`; BLAKE3 `fdc7217ced89b9d018a42ab46d72e7cf33e15906f937050d10580854298c309b`.
  - Review follow-up live receipt: `docs/evidence/protocol-763-roi-02-equipment-update-observation-2026-05-27.receipt.json` => `status=pass`, `scenario.passed=true`, `mode=run`; BLAKE3 `8100dde3ebb3476984235009e277d7e973037b7873b2fdb30c413093e1498d3d`.
  - Review follow-up run log: `docs/evidence/protocol-763-roi-02-equipment-update-observation-2026-05-27.run.log`; BLAKE3 `184223509605c1dc9b89db3d8c77ff0b8b7a1c103bec8031de75515d4777dc16`.
  - Reviewable dry-run receipt: `docs/evidence/protocol-763-roi-02-equipment-update-observation-2026-05-27.dry-run.receipt.json` => `status=pass`, `scenario.passed=false`, `mode=dry-run`, required/missing milestone `entity_equipment_update`; BLAKE3 `44383cc9cdb5fe0eaab3ce7da11280eb217a448c81b44b37087b7c1bd5b3696b`.
  - Runner verification log: `docs/evidence/protocol-763-runner-verification-2026-05-27.log`; BLAKE3 `6ddd9f6256e06fb9b2d8afb7cd35e92cf5f7a87d4a1e9819fb0afb46a3098a4f`.
  - Review fix: `tools/mc-compat-runner` now keys `entity_equipment_update` on the stable `equipment_probe_entity_equipment` marker, matching current Stevenarella logs.
  - Deterministic gate: `nix build .#checks.x86_64-linux.mc-compat-valence-ctf-equipment-update-observation-dry-run --no-link -L --no-update-lock-file --option builders ''` passed.
  - Focused compile checks passed: `cargo check --manifest-path tools/mc-compat-runner/Cargo.toml`, `cargo check --example ctf`, and `cargo check --bin stevenarella`.
