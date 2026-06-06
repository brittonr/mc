# Tasks

- [ ] [serial] Define the bounded status-effect contract, target/effect/amplifier/duration, packet rows, and explicit non-claims. r[mc_compatibility.entity_status_effect_packets_promotion.contract]
- [ ] [depends:contract] Add deterministic positive and negative checker fixtures for valid evidence, missing row id, stale revisions, wrong effect metrics, missing apply/remove correlation, and broad effect/modifier overclaims. r[mc_compatibility.entity_status_effect_packets_promotion.checker]
- [ ] [depends:checker] Add the isolated runner/client/fixture rail for one status-effect packet scenario. r[mc_compatibility.entity_status_effect_packets_promotion.rail]
- [ ] [depends:rail] Produce reviewable receipts/logs, normalized inputs, child revision metadata, and BLAKE3 manifests under `docs/evidence/`. r[mc_compatibility.entity_status_effect_packets_promotion.artifacts]
- [ ] [depends:artifacts] Promote only the configured status-effect row in packet inventory, acceptance matrix, and current bundle while preserving broad non-claims. r[mc_compatibility.entity_status_effect_packets_promotion.matrix]
- [ ] [depends:matrix] Run checker, runner, packet inventory, evidence manifest, task-evidence, Cairn gate, and validation checks with reviewable logs. r[mc_compatibility.entity_status_effect_packets_promotion.validation]
